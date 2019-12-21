use std::thread;
use std::net::UdpSocket;
use std::sync::mpsc::{self, TryRecvError,Receiver,Sender};
use std::str;
use std::sync::{Arc,Mutex};
use itertools::Itertools;

pub struct UDP_Endpoint {
    port: u32,
    read_thread: Option<thread::JoinHandle<()>>,
    msg_channel_receiv: Option<Receiver<String>>,
    msg_channel_sender: Sender<String>,
    quit_channel_receiv: Receiver<()>,
    quit_channel_sender: Sender<()>,
    socket: UdpSocket
}


impl UDP_Endpoint {
    pub fn new(port: u32) -> UDP_Endpoint {
        let (tx, rx) = mpsc::channel();
        let (qtx, qrx) = mpsc::channel();
        let addr = String::from("127.0.0.1:") + &port.to_string();

        println!("Try to start UDP-Server {0}", addr);
        let socket = UdpSocket::bind(&addr).expect("Failed to ramp up UDP Endpoint");

        UDP_Endpoint { port, read_thread: None, msg_channel_receiv: Some(rx), msg_channel_sender: tx, socket,quit_channel_receiv:qrx,quit_channel_sender:qtx }
    }

    fn receive_loop<>(socket: UdpSocket, msgChannel: Sender<String>) {
        let mut buf = [0; 2048];
        println!("Receiver loop started");

        loop {
            match socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    println!("amt: {}", amt);
                    println!("src: {}", src);
                    println!("Received {}", buf[0 .. amt].iter().format(""));
                    let str = str::from_utf8(&buf[0 .. amt]).unwrap();
                    msgChannel.send(str.to_string());
                    //Todo process data
                }
                Err(err) =>
                    println!("Read error: {}", err)
            }
        }
    }

    pub fn start_Server(&mut self) ->  (Sender<()>,Receiver<String>) {
        let l_socket = self.socket.try_clone().unwrap();
        let sender = self.msg_channel_sender.clone();
        let reader_thread = thread::spawn(move || {
            UDP_Endpoint::receive_loop(l_socket,sender );
        });

        self.read_thread = Some(reader_thread);


        (self.quit_channel_sender.clone(),self.msg_channel_receiv.take().unwrap())
    }

    fn stop_Server(&mut self) -> bool {
        // Todo: Send cancel mesasge via channel
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_UDP_Endpoint(){

        assert!(true);

    }
}

