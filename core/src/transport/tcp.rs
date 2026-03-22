use tokio_util::{sync::CancellationToken,task::JoinMap};

use tokio::{net::{TcpStream, TcpListener, ToSocketAddrs, tcp::{OwnedReadHalf, OwnedWriteHalf}}, sync::{mpsc::{channel,Sender, Receiver}}, io::{AsyncReadExt}};

use std::{collections::HashMap};

use bytes::BytesMut;

use postcard::{to_stdvec,take_from_bytes,from_bytes};

use crate::{net::{ActiveSession, PendingSession, NetClient,NetService,Message},payload::{Payload}, transport::TransportError, peer::{Peer,PeerId}, message::{IncomingMessage,OutgoingMessage}};

const CHANNELSIZE: usize = 128;
// Max message len current 226
const BUFSIZE: usize = 256;
pub struct TcpTransport{
    incoming_messages: Receiver<(PeerId,Message)>,
    message_sender: Sender<(PeerId, Message)>,
    active_conns: JoinMap<PeerId, Result<(), TransportError>>,
    cancel_tokens: HashMap<PeerId,CancellationToken>,
    sessions: HashMap<PeerId, ActiveSession>,
    write_streams: HashMap<PeerId, OwnedWriteHalf>,
    client: NetClient,
    listener: TcpListener, // Used to establish a new session - not in trait currently - todo
}

impl TcpTransport {
    pub async fn bind<T: ToSocketAddrs>(client: NetClient, addr: T) -> Result<Self, TransportError> {
        // Create a listener on given IP
        let listener = TcpListener::bind(addr).await?;
        let (sender,receiver) = channel(CHANNELSIZE);
        Ok(TcpTransport {
            incoming_messages: receiver,
            message_sender: sender,
            active_conns: JoinMap::new(),
            cancel_tokens: HashMap::new(),
            sessions: HashMap::new(),
            write_streams: HashMap::new(),
            client,
            listener,
        })
    }
}

async fn read_from_stream(stream: &mut OwnedReadHalf) -> Result<BytesMut, TransportError> {
    let res = loop {
        stream.readable().await?;
        let mut buf = BytesMut::with_capacity(BUFSIZE);
        let res = stream.read_buf(&mut buf).await;
        match res {
            Ok(0) => {
                return Err(TransportError::ConnectionClosed(None));
            },
            Ok(_) => {
                break Ok(buf);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
            Err(e) => return Err(e.into()),
        }
    };
    res
}

async fn read_message(stream: &mut OwnedReadHalf,id: PeerId, results: &Sender::<(PeerId,Message)>) -> Result<(), TransportError> {
    let mut msg_buf = BytesMut::with_capacity(BUFSIZE);
    loop {
        let data = read_from_stream(stream).await.map_err(|e| {
            match e {
                // Add peer Id info
                TransportError::ConnectionClosed(None) => {
                    TransportError::ConnectionClosed(Some(id))
                },
                _ => e,
            }
        })?;
        msg_buf.extend_from_slice(&data);
        let rem = match take_from_bytes::<Message>(&msg_buf) {
            Ok((msg,rem)) => {
                results.send((id,msg)).await?;
                BytesMut::from(rem)
            },
            Err(e) => {
                match e {
                    postcard::Error::DeserializeUnexpectedEnd => {
                        // Buffer not large enough - continue reading
                        continue
                    },
                    _ => return Err(e.into()),
                }
            }
        };
        // take_from_bytes cannot mutate data so msg_buf will still contain the used + remaining bytes
        // replace it with the remaining bytes
        msg_buf = rem;
    }
}

impl NetService for TcpTransport {
    type Error = TransportError;

    async fn add_session(&mut self, client: (Peer, ActiveSession)) -> Result<(), Self::Error> {
        if self.sessions.contains_key(&client.0.id) {
            return Err(TransportError::PeerAlreadyConnected(Some(client.0.id)));
        }
        let (mut reader,writer) = TcpStream::connect(client.0.address).await?.into_split();
        self.sessions.insert(client.0.id, client.1);
        self.write_streams.insert(client.0.id,writer);

        let cncl = CancellationToken::new();
        let cncl_task = cncl.clone();
        let send = self.message_sender.clone();
        self.active_conns.spawn(client.0.id,async move {
            // Use cancellation token instead of handle abortion to retrieve the OwnedReadHalf on cancel
            loop {
                tokio::select!{
                    _ = cncl_task.cancelled() => return Err(TransportError::ReadCancelled(reader,Some(client.0.id))),
                    out = read_message(&mut reader,client.0.id,&send) => {
                        match out {
                            Ok(_) => {},
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    },
                }
            }
        });
        self.cancel_tokens.insert(client.0.id, cncl);
        Ok(())
    }

    async fn drop_session(&mut self, peer: &PeerId) -> Result<(), Self::Error> {
        self.sessions.remove(peer).ok_or(TransportError::SessionNotFound(Some(*peer)))?;
        self.write_streams.remove(peer).ok_or(TransportError::SessionNotFound(Some(*peer)))?;
        let aborted = self.active_conns.abort(peer);
        if !aborted {
            return Err(TransportError::ConnectionNotInMap(Some(*peer)))
        };
        self.cancel_tokens.remove(peer);
        Ok(())
    }

    async fn listen(&mut self, token: CancellationToken) -> Result<IncomingMessage, Self::Error> {
        loop {
            tokio::select!{
                _ = token.cancelled() => { return Err(TransportError::Cancelled); },
                Some((id,res)) = self.active_conns.join_next() => {
                    match res {
                        Ok(r) => {
                            match r {
                                Ok(_) => {}, // Session ended gracefully, ignore it
                                Err(e) => {
                                    match self.drop_session(&id).await {
                                        Ok(_) => {},
                                        Err(e) => {
                                            match e {
                                                // join_next removes the connection from the map
                                                // drop_session would return an error that we can ignore
                                                // We still want to know if other parts of drop_session fail
                                                TransportError::ConnectionNotInMap(_) => {},
                                                _ => return Err(e)
                                            }
                                        }
                                    }
                                    return Err(e);
                                }
                            }
                        },
                        // We don't want to stop listening if the connection had previously been cancelled and is hanging around
                        Err(e) if e.is_cancelled() => continue,
                        Err(e) => return Err(e.into())
                    }
                }
                msg = self.incoming_messages.recv() => {
                    if let Some((id,encrypted)) = msg {
                        let session = self.sessions.get_mut(&id).ok_or(TransportError::SessionNotFound(Some(id)))?;
                        let decrypted = session.receive(encrypted)?;
                        let outgoing = from_bytes::<OutgoingMessage>(&decrypted)?;
                        return Ok(IncomingMessage::receive(id, outgoing))
                    } else {
                        // Channel has been closed
                        return Err(TransportError::MessageChannelClosed)
                    }
                }
            }
        }
    }
    
    async fn broadcast(&mut self, msg: Payload, token: CancellationToken) -> Result<(), Self::Error> {
        let keys = self.sessions.keys().map(|k| k.clone()).collect::<Vec<PeerId>>();
        // Todo - make this non-serial if poss? Transmit requires mutable borrow of self due to session needing to be mutable.
        for peer in keys {
            self.transmit(msg.clone(), peer, token.clone()).await?;
        }
        Ok(())
    }
    
    async fn transmit(&mut self, msg: Payload, target: PeerId, token: CancellationToken) -> Result<(), Self::Error> {
            let session = self.sessions.get_mut(&target).ok_or(TransportError::SessionNotFound(Some(target)))?;
            let stream = self.write_streams.get(&target).ok_or(TransportError::SessionNotFound(Some(target)))?;
            
            let res = tokio::select!{
                _ = token.cancelled() => {
                    Err(TransportError::Cancelled)
                },
                _ = async {
                    // Serialize once to get a format that can be encrypted
                    let data = to_stdvec(&msg)?;
                    let encrypted_data = session.send(&data)?;
                    // Serialize again to get a sendable stream
                    let serialized_data = to_stdvec(&encrypted_data)?;
                    loop {
                        stream.writable().await?;
                        match stream.try_write(&serialized_data) {
                            Ok(_) => break,
                            Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => continue,
                            Err(e) => return Err(e.into()),
                        };
                    }
                    Ok::<(),TransportError>(())
                } => Ok(()),
            };
            
            match res {
                Ok(()) => Ok(()),
                Err(err) => Err(err),
            }
    }
}

#[cfg(test)]
mod test {    
    use super::*;
    use crate::{message::OutgoingMessage,payload::{Action, Query, Reply, TagQuery}, pow::Pow, tag::{Tag,TagPayload}};
    use std::{io::Write, sync::mpsc::{Sender, channel}};
    use tokio::time::timeout;
    use std::{time::{Duration}};
    use std::io::Read;
    use postcard::from_bytes;

    // Timeout for async function calls
    const TIMEOUT: Duration = Duration::from_millis(10);
    // <Payload> struct size is 200
    const BUFSIZE: usize = 256;

    // Create a generic transport for testing
    async fn ephemeral_transport() -> TcpTransport {
        TcpTransport::bind(NetClient::Ephemeral, "127.0.0.1:0").await.unwrap()
    }

    async fn static_transport() -> TcpTransport {
        TcpTransport::bind(NetClient::from_seed([1u8; 32]), "127.0.0.1:0").await.unwrap()
    }

    #[tokio::test]
    async fn test_bind() {
        // This test needs to use concrete addresses (non port 0) to check it fails correctly
        let client = NetClient::Ephemeral;
        let addr = "127.0.0.1:30000";
        let transport = TcpTransport::bind(client, addr).await;
        
        let static_client = NetClient::from_seed([1u8; 32]);
        let addr = "127.0.0.1:30030";
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
        let srv = TcpListener::bind("127.0.0.1:0").await.expect("Failed to start test server");
        let addr = srv.local_addr().expect("Failed to get local address of thread");
        tokio::spawn(async move {
            srv.accept().await
        });
        
        // First add should succeed - unique Peer ID
        let peer = Peer::new(client.identity().expect("Expect static identity"), addr.to_string());
        let pend_session = PendingSession::new([0u8;32],Some(0u64));
        let session = pend_session.activate(None).expect("Failed to active test session");
        let expect_ok = transport.add_session((peer, session)).await;
        assert!(expect_ok.is_ok(),"Failed to add session: {}",expect_ok.err().unwrap());
        assert_eq!(transport.sessions.len(), 1);
        
        // Creating a new Peer with the same peer ID should fail when added.
        // See comment in NetService trait about whether this behaviour should be changed.
        let peer = Peer::new(client.identity().expect("Expect static identity"), addr.to_string());
        let pend_session = PendingSession::new([1u8;32],Some(1u64));
        let session = pend_session.activate(None).expect("Failed to active test session");
        let expect_fail = transport.add_session((peer,session)).await;
        assert!(expect_fail.is_err(),"Expected failure adding second entry with same peer ID");
    }
    
    #[tokio::test]
    async fn test_drop_session() {
        let srv = TcpListener::bind("127.0.0.1:0").await.expect("Failed to start test server");
        let addr = srv.local_addr().expect("Failed to get local address of thread");
        
        // Create a transport client with 3 sessions
        let mut transport = ephemeral_transport().await;
        let mut ids = vec!();
        for i in 0..3 {
            let client = NetClient::from_seed([i as u8;32]);
            let peer = Peer::new(client.identity().expect("Expect static identity"), addr.to_string());
            
            let pend_session = PendingSession::new([i as u8;32],Some(i as u64));
            let net_session = pend_session.activate(None).expect("Failed to active test session");
            transport.sessions.insert(peer.id,net_session);
            
            let (_,write_stream) = TcpStream::connect(addr).await.expect("Failed to connect to test server on {addr}").into_split();
            transport.write_streams.insert(peer.id,write_stream);
            transport.active_conns.spawn(peer.id, async {Ok(())});
            transport.cancel_tokens.insert(peer.id, CancellationToken::new());

            
            ids.push(peer.id);
        }
        
        assert_eq!(transport.sessions.len(),3,"Expected 3 elements in starting transport");
        assert_eq!(transport.write_streams.len(),3,"Expected 3 elements in starting transport");
        assert_eq!(transport.active_conns.len(),3,"Expected 3 elements in starting transport");
        
        for i in (0..3).rev() {
            let expect_ok = transport.drop_session(&ids.pop().unwrap()).await;
            assert!(expect_ok.is_ok());
            assert_eq!(transport.sessions.len(),i,"Failed to remove session {i} from sessions");
            assert_eq!(transport.write_streams.len(),i,"Failed to remove session {i} from write_streams");
        }
        
        let client = NetClient::from_seed([5 as u8;32]);
        let peer = Peer::new(client.identity().expect("Expect static identity"), addr.to_string());
        let expect_err = transport.drop_session(&peer.id).await;

        assert!(expect_err.is_err(),"Expected failure removing non-existent session");
        
    }
    
    #[tokio::test]
    async fn test_transmit(){
        // Start a listener and move it into a thread after getting the port assigned by the OS.
        // Send a channel into the thread to read success/failure of expected values
        // Give it a handler function checking expected results and sending true/false over a channel

        // Create a dummy active session that can be used for testing message encryption
        let (mut sender_shared_session,mut receiver_shared_session ) = gen_shared_sessions([0u8;32],5u64);

        // Test that shared keys match and encryption/decryption work
        let dummy_data = [1u8; 32];
        let encrypt = sender_shared_session.send(&dummy_data).expect("Failed to encrypt dummy data");
        let decrypt = receiver_shared_session.receive(encrypt).expect("Failed to decrypt dummy data");
        assert_eq!(decrypt, dummy_data,"Decrypted dummy data did not match - discontinuing");

        let client = NetClient::from_seed([1u8;32]);
        let (send,results) = channel::<bool>();
        let expect_messages = sample_messages();
        let send_messages = expect_messages.clone();
        
        let srv = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to start test server");
        let addr = srv.local_addr().expect("Failed to get local address of thread");

        std::thread::spawn(move || {

            let (stream, _) = srv.accept().expect("Failed to accept incoming stream");
                expect_message_tcp(
                    stream, 
                    expect_messages,
                    &mut receiver_shared_session,
                    send.clone());
        });
        let mut transport = ephemeral_transport().await;
        let peer = Peer::new(client.identity().expect("Expect static identity"), addr.to_string());
        transport.add_session((peer.clone(),sender_shared_session)).await.expect("Failed to add test session to transport");
        for (i,msg) in send_messages.into_iter().enumerate(){
            let cncl = CancellationToken::new();
            let res = timeout(
                TIMEOUT,
                transport.transmit(msg,peer.id.clone(), cncl)
            ).await;
            assert!(res.is_ok(),"Failed to transmit message {}: {}",i,res.err().unwrap());
            let got_msg = results.recv().expect("Failed to check receive result");
            assert!(got_msg,"Message {} did not match expected",i);
        }

    }
    
    #[tokio::test]
    async fn test_listen(){

        let mut transport = static_transport().await;
        
        let mut threads = vec!();

        let mut expect_messages = vec!();
        // Set up test clients to listen to - send a different message from each one
        let sample_messages = sample_messages();
        let num_messages = sample_messages.len();
        for (i,msg) in sample_messages.into_iter().enumerate() {
            let (mut sender_session,receiver_session) = gen_shared_sessions([i as u8;32],i as u64 + 12);
            
            let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to start test server");
            let addr = listener.local_addr().expect("Failed to get local address of thread");
            
            let client = NetClient::from_seed([i as u8;32]);
            let peer = Peer::new(client.identity().expect("Expect static ID"), addr.to_string());
            
            expect_messages.push(
                IncomingMessage{
                    from: peer.id.clone(),
                    payload: msg.clone(),
                    id: 0,
                }
            );
            
            let send_message = OutgoingMessage{
                to: transport.client.identity().expect("Expected an ID").peer_id(),
                payload: msg.clone(),
                id: 0,
            };

            transport.add_session((peer,receiver_session)).await.expect("Failed to add test session to transport");
            
            let handle = std::thread::spawn(move || {
                let (mut stream, _) = listener.accept().expect("Failed to accept incoming stream");
                let msg_bytes = to_stdvec(&send_message).expect("Failed to serialize message");
                let encrypt = sender_session.send(&msg_bytes).expect("Failed to encrypt message");
                let serialized = to_stdvec(&encrypt).expect("Failed to serialize encrypted message");
                stream.write_all(&serialized).expect("Failed to write to stream");
                stream
            });
            threads.push(handle);
            
            
        }
        
        let mut streams = vec!();
        for thread in threads {
            streams.push(thread.join().expect("Failed to join thread"));
        }

        let mut got_messages = vec!();
        for _ in 0..num_messages {
            let cncl = CancellationToken::new();
            let msg = transport.listen(cncl).await.expect("Failed to listen for messages");
            got_messages.push(msg);
        }

        for msg in expect_messages {
            assert!(got_messages.contains(&msg), "Received message did not match expected" );
        }

        drop(streams);

    }

    #[tokio::test]
    async fn test_broadcast(){
        let num_test_clients: usize = 5;

        let mut transport = ephemeral_transport().await;
        let test_message = Payload::Query(Query::Tag(TagQuery::Get));
        let (res_sender,res_receiver) = std::sync::mpsc::channel::<bool>();
        
        let mut threads = vec!();
        // Set up test clients to broadcast to
        for i in 1..=num_test_clients {
            let (sender_session,mut receiver_session) = gen_shared_sessions([i as u8;32],i as u64 + 12);
            
            let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to start test server");
            let addr = listener.local_addr().expect("Failed to get local address of thread");
            
            let client = NetClient::from_seed([i as u8;32]);
            let peer = Peer::new(client.identity().expect("Expect static ID"), addr.to_string());

            let res_chan = res_sender.clone();
            
            transport.add_session((peer,sender_session)).await.expect("Failed to add test session to transport");
            let handle = std::thread::spawn(move || {
                let (stream, _) = listener.accept().expect("Failed to accept incoming stream");
                let expect_messages = vec!(Payload::Query(Query::Tag(TagQuery::Get)));
                expect_message_tcp(
                    stream, 
                    expect_messages,
                    &mut receiver_session,
                    res_chan);
            });
            threads.push(handle);
        }

        // Broadcast the test message - extend the timeout to account for the number of messages
        let cncl = CancellationToken::new();
        let res = timeout(
            TIMEOUT * num_test_clients as u32,
            transport.broadcast(test_message, cncl)
        ).await;

        for thread in threads {
            thread.join().expect("A thread failed");
        }

        // Close the channel to stop the iterator below hanging
        drop(res_sender);

        // Check results
        assert!(res.is_ok(),"Failed to broadcast message {}",res.err().unwrap());
        let got_msg = res_receiver.iter().collect::<Vec<bool>>();
        assert_eq!(got_msg.len(),num_test_clients,"Expected {num_test_clients} but received {} results",got_msg.len());
        assert!(got_msg.iter().all(|x| *x),"Not all messages received");
    }

    // Helper to test receiving messages over TCP
    fn expect_message_tcp(mut stream: std::net::TcpStream, expect_messages: Vec<Payload>, session: &mut ActiveSession, results: Sender<bool>) {
        for expect_message in expect_messages {
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
                    Err(e) => {
                        match e {
                            postcard::Error::SerdeDeCustom => {
                                // Golden path - deserialization failure because the message is encrypted.
                            },
                            _ => {
                                // No real reason to end up here.
                                eprintln!("Error receiving encrypted message: {}", e);
                                results.send(false).expect("Failed to send test failure");
                            }
                        }
                    }
                    Ok(_) => {
                        // Failure - message was not encrypted
                        results.send(false).expect("Failed to send test failure due to bad encrypton");
                        
                    }
                }

                let encrypted = match from_bytes::<Message>(&buf) {
                    Ok(msg) => {
                        msg
                    },
                    Err(e) => {
                        match e {
                            postcard::Error::DeserializeUnexpectedEnd => {
                                // Loop again to get more data
                                continue 'receive;
                            },
                            _ => {
                                eprintln!("Error reading encrypted message: {}", e);
                                results.send(false).expect("Failed to send test failure");
                                break;
                            }
                        }
                    }
                };

                let decrypted = match session.receive(encrypted) {
                    Ok(msg) => {
                        msg
                    },
                    Err(e) => {
                        eprintln!("Failed to decrypt message: {}", e);
                        results.send(false).expect("Failed to send test failure due to decryption");
                        break;
                    }
                };

                match from_bytes::<Payload>(&decrypted) {
                    Ok(msg) => {
                        if msg == expect_message {
                            // Golden path
                            results.send(true).expect("Failed to send test success");
                            break;
                        } else {
                            // Bad deserialization
                            eprintln!("Bad message. Got: \n{msg:?}\n expected: \n{expect_message:?}");
                            results.send(false).expect("Failed to send test failure due to deserialization");
                            break;
                        }
                    },
                    Err(e) => {
                        eprintln!("Error deserializing decrypted message: {}", e);
                        results.send(false).expect("Failed to send test failure due to deserialization after decryption");
                        break;
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
        let fourth_message = Payload::Reply(Reply::Ok);
        vec!(
            first_message,
            second_message,
            third_message,
            fourth_message
        )
    }

    fn gen_shared_sessions(shared: [u8;32], conn_id: u64) -> (ActiveSession, ActiveSession) {
        let sender_shared_session: ActiveSession = PendingSession::new(shared.clone(),Some(conn_id)).activate(None).expect("Failed to create shared test session");
        let receiver_shared_session= PendingSession::new(shared,Some(conn_id)).activate(None).expect("Failed to create shared test session");
        (sender_shared_session, receiver_shared_session)
    }
}