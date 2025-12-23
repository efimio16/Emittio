use tokio::sync::mpsc;

// const CHAN_SIZE: usize = 1024;

pub struct Dispatcher<R, T> {
    pub rx: mpsc::Receiver<R>,
    pub tx: mpsc::Sender<T>,
}
pub struct Handler<R, T> {
    pub rx: mpsc::Receiver<R>,
    pub tx: mpsc::Sender<T>,
}

pub fn new<R, T>(size: usize) -> (Dispatcher<R, T>, Handler<T, R>) {
    let (send_tx, send_rx) = mpsc::channel(size);
    let (recv_tx, recv_rx) = mpsc::channel(size);

    (Dispatcher { rx: send_rx, tx: recv_tx }, Handler { rx: recv_rx, tx: send_tx })
}