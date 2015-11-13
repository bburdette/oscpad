use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use websocket::{Message, Sender};
use websocket::stream::WebSocketStream;
use websocket::server::sender;

type sendBlah = Arc<Mutex<sender::Sender<WebSocketStream>>>;

#[derive(Clone)]
pub struct Broadcaster {
    tvs : Arc<Mutex<Vec<sendBlah >>>
}

fn mysockeq(saleft: &SocketAddr, saright: &SocketAddr) -> bool {
  match (saleft, saright) {
    (&SocketAddr::V4(l),&SocketAddr::V4(r)) => l == r,
    (&SocketAddr::V6(l),&SocketAddr::V6(r)) => l == r,
    _ => false,
  }
}


impl Broadcaster {
  pub fn new() -> Broadcaster {
    Broadcaster {
      tvs : Arc::new(Mutex::new(Vec::new()))
    }
  }

  pub fn register(&mut self, sender : sendBlah) {
    let mut tvs = self.tvs.lock().unwrap();

    tvs.push(sender);
  }

  pub fn broadcast(&mut self, msg : Message) {
    let mut tvs = self.tvs.lock().unwrap();

    for tv in tvs.iter_mut() {
      let mut tvsend = tv.lock().unwrap();
      match tvsend.send_message(msg.clone()) {
        Err(e) => {},
        _ => {}
      }
    }
  }
   
  pub fn broadcast_others(&mut self, sa: &SocketAddr , msg : Message) {
    let mut tvs = self.tvs.lock().unwrap();

    for tv in tvs.iter_mut() {
      let mut tvsend = tv.lock().unwrap();
      match tvsend.get_mut().peer_addr() { 
        Ok(sa_send) => { 
          println!("checking eq: {:?}, {:?}", sa, sa_send);
          if !mysockeq(sa,&sa_send) {
            println!("sending to: {:?}", sa_send);
            match tvsend.send_message(msg.clone()) {
              Err(e) => {},
              _ => {}
            }
          }
         },
        Err(e) => {},
      }
    }
  }
}


