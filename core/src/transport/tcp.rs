use tokio_util::sync::CancellationToken;

use tokio::net::{TcpStream, TcpListener, ToSocketAddrs};

use std::{collections::HashMap};

use postcard::{to_slice,from_bytes};

use crate::{net::{NetService, NetSession, NetClient},payload::{Payload}, transport::TransportError, peer::{Peer,PeerId}, message::{IncomingMessage,OutgoingMessage}};

const BUFSIZE: usize = 256;
pub struct TcpTransport{
    sessions: HashMap<PeerId, (NetSession,TcpStream)>,
    client: NetClient,
    listener: TcpListener,
}

impl TcpTransport {
    pub async fn bind<T: ToSocketAddrs>(client: NetClient, addr: T) -> Result<Self, TransportError> {
        // Create a listener on given IP
        let listener = TcpListener::bind(addr).await?;
        Ok(TcpTransport {
            sessions: HashMap::new(),
            client,
            listener,
        })
    }

}

impl NetService for TcpTransport {
    type Error = TransportError;

    async fn add_session(&mut self, client: (Peer, NetSession)) -> Result<(), Self::Error> {
        if self.sessions.contains_key(&client.0.id) {
            return Err(TransportError::PeerAlreadyConnected)
        }
        let stream = TcpStream::connect(client.0.address).await?;
        self.sessions.insert(client.0.id, (client.1, stream));
        Ok(())
    }

    fn drop_session(&mut self, peer: &PeerId) -> Result<(), Self::Error> {
        self.sessions.remove(peer).ok_or(TransportError::SessionNotFound)?;
        Ok(())
    }

    async fn listen(&self, token: CancellationToken) -> Result<IncomingMessage, Self::Error> {
        loop {
            tokio::select!{
                _ = token.cancelled() => return Err(TransportError::Cancelled),
            }
        }
        todo!();
    }
    
    async fn broadcast(&self, msg: Payload, token: CancellationToken) -> Result<(), Self::Error> {
        loop {
            tokio::select!{
                _ = token.cancelled() => return Err(TransportError::Cancelled),
            }
        }
        todo!();
    }
    
    async fn transmit(&self, msg: Payload, target: Peer, token: CancellationToken) -> Result<(), Self::Error> {
        todo!();
        // Need to implement encyrption using NetSession
        // Need to get a new NetSession by doing a handshake with the target peer
        // I have a NetClient.
        // My NetClient can do an encryption handshake with a target NetIdentity
        // Then I accept the handshake with my NetClient and get a NetSession
        // NetSession can then encrypt and decrypt. 
        // But my target needs the same NetSession - how does it get that?
        // If this is Diffie-Hellman then both sides have a NetClient, both sides send their own public key in the form of a NetIdentity.
        // The handshake then creates a shared secret through DH magic
        // The accept creates a net session?

        // So I need to:
        // 1. Send my NetIdentity to the target peer
        // 2. Await receiving a NetIdentity - error if it is not a valid NetIdentity
        // 3. Create a NetSession using the received NetIdentity
        // 1a. Do steps 1-3 also on the target peer as well
        // 4. Encrypt the message using the NetSession
        // 5. Send the encrypted message to the target peer
        // 6. Decrypt the message using the NetSession
        // 7. Process the message
            tokio::select!{
                _ = token.cancelled() => return Err(TransportError::Cancelled),
                r = async {
                    let stream = TcpStream::connect(target.address).await?;
                    let mut buf = [0u8; BUFSIZE];
                    let data = to_slice(&msg, &mut buf)?;
                    loop {
                        stream.writable().await?;
                        
                        match stream.try_write(data) {
                            Ok(_) => break,
                            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => panic!("Would block"),
                            Err(e) => return Err(e.into()),
                        }
                    }
                    return Ok(());
                } => return r,
            }
    }
}

#[cfg(test)]
mod test {    
    use super::*;
    use crate::{payload::{Query,TagQuery,Reply,Action}, tag::{Tag,TagPayload}, pow::{Pow}};
    use std::sync::mpsc::{channel,Sender};
    use tokio::time::timeout;
    use std::{thread,time::{Duration}};
    use std::io::Read;

    // Timeout for async function calls
    const TIMEOUT: Duration = Duration::from_millis(10);
    // <Payload> struct size is 200
    const BUFSIZE: usize = 256;

    // Create a generic transport for testing
    async fn ephemeral_transport() -> TcpTransport {
        TcpTransport::bind(NetClient::Ephemeral, "127.0.0.1:0").await.unwrap()
    }

    #[tokio::test]
    async fn test_bind() {
        // This test needs to use concrete addresses (non port 0) to check it fails correctly
        let client = NetClient::Ephemeral;
        let addr = "127.0.0.1:80";
        let transport = TcpTransport::bind(client, addr).await;
        
        let static_client = NetClient::from_seed([1u8; 32]);
        let addr = "127.0.0.1:8080";
        let transport_2 = TcpTransport::bind(static_client, addr).await;
        
        let client = NetClient::from_seed([1u8; 32]);
        let transport_3 = TcpTransport::bind(client, addr).await;


        assert!(transport.is_ok(), "Bind should succeed with Ephemeral client but got: {}",transport.err().unwrap());
        assert!(transport_2.is_ok(), "Bind should succeed with Static client but got: {}",transport_2.err().unwrap());
        assert!(transport_3.is_err(), "Bind should fail with duplicate address but got success");
        match transport_3.err().unwrap() {
            TransportError::IO(_) => {},
            err @ _ => panic!("Expected PeerAlreadyConnected error, got: {}", err)
        }
    }

    #[tokio::test]
    async fn test_add_session() {
        let mut transport = ephemeral_transport().await;
        let client = NetClient::from_seed([1u8;32]);
        
        
        // Start a listener and move it into a thread after getting the port assigned by the OS.
        // Consider making the thread cancellable. 
        // Consider making the listener more flexible as a double.
        let srv = TcpListener::bind("127.0.0.1:0").await.expect("Failed to start test server");
        let addr = srv.local_addr().expect("Failed to get local address of thread");
        tokio::spawn(async move {
            srv.accept().await
        });
        
        // First add should succeed - unique Peer ID
        let peer = Peer::new(client.identity().expect("Expect static identity"), addr.to_string());
        let session = NetSession::new([0u8;32],0u64);
        let expect_ok = transport.add_session((peer, session)).await;
        assert!(expect_ok.is_ok(),"Failed to add session: {}",expect_ok.err().unwrap());
        assert_eq!(transport.sessions.len(), 1);
        
        // Creating a new Peer with the same peer ID should fail when added.
        // See comment in NetService trait about whether this behaviour should be changed.
        let peer = Peer::new(client.identity().expect("Expect static identity"), addr.to_string());
        let session = NetSession::new([1u8;32],1u64);
        let expect_fail = transport.add_session((peer,session)).await;
        assert!(expect_fail.is_err(),"Expected failure adding second entry with same peer ID");
    }
    
    #[tokio::test]
    async fn test_drop_session() {
        let srv = TcpListener::bind("127.0.0.1:0").await.expect("Failed to start test server");
        let addr = srv.local_addr().expect("Failed to get local address of thread");
        // sessions: HashMap<PeerId, (NetSession,TcpStream)>,
        
        // Create a transport client with 3 sessions
        let mut transport = ephemeral_transport().await;
        let mut ids = vec!();
        for i in 0..3 {
            let client = NetClient::from_seed([i as u8;32]);
            let net_session = NetSession::new([i as u8;32],i as u64);
            let stream = TcpStream::connect(addr).await.expect("Failed to connect to test server on {addr}");
            let peer = Peer::new(client.identity().expect("Expect static identity"), addr.to_string());
            transport.sessions.insert(peer.id,(net_session,stream));
            ids.push(peer.id);
        }
        
        assert_eq!(transport.sessions.len(),3,"Expected 3 elements in starting transport");
        
        for i in (0..3).rev() {
            let expect_ok = transport.drop_session(&ids.pop().unwrap());
            assert!(expect_ok.is_ok());
            assert_eq!(transport.sessions.len(),i,"Failed to remove session {i}");
        }
        
        let client = NetClient::from_seed([5 as u8;32]);
        let peer = Peer::new(client.identity().expect("Expect static identity"), addr.to_string());
        let expect_err = transport.drop_session(&peer.id);

        assert!(expect_err.is_err(),"Expected failure removing non-existent session");
        
    }
    
    #[tokio::test]
    async fn test_transmit(){
        // Start a listener and move it into a thread after getting the port assigned by the OS.
        // Send a channel into the thread to read success/failure of expected values
        // Give it a handler function checking expected results and sending true/false over a channel
        
        let client = NetClient::from_seed([1u8;32]);
        let (send,results) = channel::<bool>();
        let expect_messages = sample_messages();
        let send_messages = expect_messages.clone();
        
        let srv = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to start test server");
        let addr = srv.local_addr().expect("Failed to get local address of thread");

        std::thread::spawn(move || {
            for (stream,msg) in srv.incoming().zip(expect_messages.into_iter()) {
                expect_message_tcp(
                    stream.expect("Failed to get incoming stream"), 
                    msg,
                    send.clone());
            }
        });

        let transport = ephemeral_transport().await;
        let peer = Peer::new(client.identity().expect("Expect static identity"), addr.to_string());
        for (i,msg) in send_messages.into_iter().enumerate(){
            let cncl = CancellationToken::new();
            let res = timeout(
                TIMEOUT,
                transport.transmit(msg,peer.clone(), cncl)
            ).await;
            assert!(res.is_ok(),"Failed to transmit message {}: {}",i,res.err().unwrap());
            let got_msg = results.recv().expect("Failed to check receive result");
            assert!(got_msg,"Message {} did not match expected",i);
        }

    }
    
    #[tokio::test]
    async fn test_listen(){
        let mut transport = ephemeral_transport();
        let client = NetClient::from_seed([1u8;32]);
        
        
        // Start a server that will transmit messages
        let srv = TcpListener::bind("127.0.0.1:0").await.expect("Failed to start test server");
        let addr = srv.local_addr().expect("Failed to get local address of thread");
        thread::spawn(async move || {
            match srv.accept().await {
                Ok(_) => {},
                Err(e) => panic!("Failed to accept test server request: {}",e),
            }
        });
        todo!();
    }

    // Helper to test receiving messages over TCP
    fn expect_message_tcp(mut stream: std::net::TcpStream, expect_message: Payload, results: Sender<bool>) {
        'receive: loop {
            let mut buf = [0u8;BUFSIZE];
            match stream.read(&mut buf) {
                Ok(0) => break,
                Ok(_) => {},
                Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
                    continue;
                },
                Err(e) => {
                    eprintln!("Error reading message: {}", e);
                    results.send(false).expect("Failed to send test failure due to TCP read");
                    return
                }
            }

            match from_bytes::<Payload>(&buf) {
                Ok(msg) => {
                    if msg == expect_message {
                        // Golden path
                        results.send(true).expect("Failed to send test success");
                    } else {
                        // Bad deserialization
                        eprintln!("Bad message. Got: \n{msg:?}\n expected: \n{expect_message:?}");
                        results.send(false).expect("Failed to send test failure due to deserialization");
                        break;
                    }
                },
                Err(e) => {
                    match e {
                        postcard::Error::DeserializeUnexpectedEnd => {
                            // Loop again to get more data
                            continue 'receive;
                        },
                        _ => {
                            results.send(false).expect("Failed to send test failure");
                            eprintln!("Error reading message: {}", e);
                            break;
                        }
                    }
                }
            }
        }
    }

    // Helper to generate sample messages for testing
    fn sample_messages() -> Vec<Payload> {
        let first_message = Payload::Query(Query::Tag(TagQuery::Get));
        let second_message = Payload::Reply(Reply::Ok);
        let (test_tag, test_pow) = {(
            Tag::new(&[7u8;32],TagPayload{data:vec!()}).expect("Failed to generate test tag"),
            Pow::new(&[4u8;32],Action::PublishTag, 213u8)
        )};
        let third_message = Payload::Query(Query::Tag(TagQuery::Publish{tag: test_tag, pow: test_pow, nonce: 17u64}));
        vec!(
            second_message,
            first_message,
            third_message
        )
    }

}