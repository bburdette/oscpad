#![feature(custom_derive)]

extern crate websocket;

use std::thread;
use std::sync::{Arc, Mutex};
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::os;
use std::env;
use std::convert;

use std::io::Read;
use std::io::{Error,ErrorKind};
use std::error;
use std::string::*;
use std::collections::BTreeMap;

use std::net::UdpSocket;
use std::net::SocketAddr;

use websocket::{Server, Message, Sender, Receiver};
use websocket::header::WebSocketProtocol;
use websocket::stream::WebSocketStream;
use websocket::server::sender;

extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

#[macro_use]
mod tryopt;
mod stringerror;
mod controls;
mod broadcaster;

extern crate tinyosc;
use tinyosc as osc;

extern crate serde_json;
use serde_json::Value;
use serde_json::ser;

/*
macro_rules! try_opt { 
  ($e: expr) => { 
    match $e { 
      Some(x) => x, 
      None => return None 
      } 
  } 
}

macro_rules! try_opt_resbox { 

  ($e: expr, $s: expr) => { 
    match $e { 
      Some(x) => x, 
      None => return Err(Box::new(Error::new(ErrorKind::Other, $s))), 
      } 
  } 
}

macro_rules! try_opt_res {
  ($e: expr, $s: expr) =>  
    (match $e {
      Some(val) => val,
      None => {
          let err = Error::new(ErrorKind::Other, $s) ;
          return Err(std::convert::From::from(err))
      },
    })
}
*/

/*

hmmm not getting why this doesn't work... want it to be like the try! macro.

macro_rules! try_opt_res {
  ($e: expr, $s: expr) =>  
    (match $e {
      Some(val) => val,
      None => {
          let err = Err(Error::new(ErrorKind::Other, $s)) ;
          return $crate::result::Result::Err($crate::convert::From::from(err))
      },
    })
}
*/

fn loadString(file_name: &str) -> Option<String>
{
  let path = &Path::new(&file_name);
  let f = File::open(path);
  let mut result = String::new();
  match f { 
    Ok(mut of) => {
      match of.read_to_string(&mut result) { 
        Err(e) => {
          println!("err: {:?}", e);
          None
        },
        Ok(len) => { 
          println!("read {} bytes", len);
          Some(result)
        }, 
      } 
    },
    Err(e) => {
      println!("err: {:?}", e);
      None
    },
  }
}


fn main() {
  // read in the settings json.
  let args = env::args();
  let mut iter = args.skip(1); // skip the program name
  match iter.next() {
    Some(file_name) => {
      
      match startserver(&file_name) {
        Err(e) => println!("error starting server: {:?}", e),
        _ => (),
      }

    }
    None => {
      println!("oscpad syntax: ");
      println!("oscpad <json config file>");
    }
  }
}

fn startserver(file_name: &String) -> Result<(), Box<std::error::Error> >
{
    println!("loading config file: {}", file_name);
    let configstring = try_opt_resbox!(loadString(&file_name), 
                                       "error reading config file");

    // read config file as json
    let configval: Value = try!(serde_json::from_str(&configstring[..]));

    let obj = configval.as_object().unwrap();
    
    let htmlfilename = 
      try_opt_resbox!(
        try_opt_resbox!(obj.get("htmlfile"), 
                        "'htmlfile' not found in config file").as_string(), 
        "failed to convert to string");
    let htmlstring = 
        try_opt_resbox!(loadString(&htmlfilename[..]), 
                       "error loading html file!");

    let guifilename = 
      try_opt_resbox!(
        try_opt_resbox!(obj.get("guifile"), 
                        "'guifile' not found in config file").as_string(), 
        "failed to convert to string");
    let guistring = loadString(&guifilename[..]).unwrap();
        try_opt_resbox!(loadString(&guifilename[..]), 
                       "error loading gui file!");

    let ip = String::new() + 
      try_opt_resbox!(
        try_opt_resbox!(obj.get("ip"), 
                       "'ip' not found!").as_string(), 
        "'ip' not a string!");

    let wip = String::new() + 
      try_opt_resbox!(
        try_opt_resbox!(obj.get("wip"), 
                       "'wip' not found!").as_string(), 
        "'wip' not a string!");

    let oscrecvip = String::new() + 
      try_opt_resbox!(
        try_opt_resbox!(obj.get("oscrecvip"), 
                       "'oscrecvip' not found!").as_string(), 
        "'oscrecvip' not a string!");

    let oscsendip = String::new() + 
      try_opt_resbox!(
        try_opt_resbox!(obj.get("oscsendip"), 
                       "'oscsendip' not found!").as_string(), 
        "'oscsendip' not a string!");

    // deserialize the gui string into json.
    let guival: Value = try!(serde_json::from_str(&guistring[..])); 

    let blah = try!(controls::deserializeRoot(&guival));

    println!("title: {} count: {} ", 
      blah.title, blah.rootControl.controlType());
    println!("controls: {:?}", blah.rootControl);

    // from control tree, make a map of ids->controls.
    let mapp = controls::makeControlMap(&*blah.rootControl);
    let cnm = controls::controlMapToNameMap(&mapp);

    let cm = Arc::new(Mutex::new(mapp));

    let guijson = String::new() + &guistring[..];

    let oscsocket = try!(UdpSocket::bind(&oscrecvip[..]));
    let mut bc = broadcaster::Broadcaster::new();
    let wsos = try!(oscsocket.try_clone());
    let wscm = cm.clone();
    let wsbc = bc.clone();
    let wsoscsendip = oscsendip.clone();

    thread::spawn(move || { 
      oscmain(oscsocket, oscsendip, bc, cm, cnm)
      }); 

    thread::spawn(move || { 
      websockets_main(wip, guijson, wscm, wsbc, wsoscsendip, wsos);
      });

    try!(Iron::new(move |req: &mut Request| {
        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, &*htmlstring)))
    }).http(&ip[..]));

    Ok(())
}

fn websockets_main( ipaddr: String, 
                    aSConfig: String, 
                    cm: Arc<Mutex<controls::controlMap>>,
                    broadcaster: broadcaster::Broadcaster, 
                    oscsendip: String, 
                    oscsocket: UdpSocket ) 
{
	let server = Server::bind(&ipaddr[..]).unwrap();

	for connection in server {

    println!("new websockets connection!");
		// Spawn a new thread for each connection.
    let blah = String::new() + &aSConfig[..];
    
    let scm = cm.clone();
    
    let osock = oscsocket.try_clone().unwrap();
    let osend = oscsendip.clone();

    let mut broadcaster = broadcaster.clone();

    thread::spawn(move || {
      // Get the request
			let request = connection.unwrap().read_request().unwrap(); 
      // Keep the headers so we can check them
			let headers = request.headers.clone(); 
			
			request.validate().unwrap(); // Validate the request
			
			let mut response = request.accept(); // Form a response
			
			if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
				if protocols.contains(&("rust-websocket".to_string())) {
					// We have a protocol we want to use
					response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
				}
			}
			
			let mut client = response.send().unwrap(); // Send the response
			
			let ip = client.get_mut_sender()
				.get_mut()
				.peer_addr()
				.unwrap();
			
			println!("Connection from {}", ip);
     
      let message = Message::Text(blah);
			client.send_message(message.clone()).unwrap();
			
			let (sender, mut receiver) = client.split();

      let sendmeh = Arc::new(Mutex::new(sender));
      
      broadcaster.register(sendmeh.clone());      
			
			for message in receiver.incoming_messages() {
				let message = message.unwrap();
        println!("message: {:?}", message);
	
        let replyy = Message::Text("replyyyblah".to_string());
	
				match message {
					Message::Close(_) => {
						let message = Message::Close(None);
            let mut sender = sendmeh.lock().unwrap();
						sender.send_message(message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					Message::Ping(data) => {
            println!("Message::Ping(data)");
						let message = Message::Pong(data);
            let mut sender = sendmeh.lock().unwrap();
						sender.send_message(message).unwrap();
					}
          Message::Text(texxt) => {
            let jsonval: Value = serde_json::from_str(&texxt[..]).unwrap();
            let s_um = controls::decodeUpdateMessage(&jsonval);
            match s_um { 
              Some(updmsg) => {
                let mut scmun = scm.lock().unwrap();
                let cntrl = scmun.get_mut(controls::getUmId(&updmsg));
                match cntrl {
                  Some(x) => {
                    (*x).update(&updmsg);
                    println!("some x: {:?}", *x);
                    broadcaster.broadcast_others(&ip, Message::Text(texxt));
                    match ctrlUpdateToOsc(&updmsg, &**x) { 
                      Ok(v) => osock.send_to(&v, &osend[..]),
                      _ => Err(Error::new(ErrorKind::Other, "meh")) 
                    };
                    ()
                  },
                  None => println!("none"),
                }
              },
              _ => println!("uknown msg"),
            }
          },
					_ => { 
            println!("sending replyy");
            let mut sender = sendmeh.lock().unwrap();
            sender.send_message(replyy).unwrap()
          }
				}
			}
		});
	}
}

fn ctrlUpdateToOsc(um: &controls::UpdateMsg, ctrl: &controls::Control) -> Result<Vec<u8>, Error>
{
  match um {
    &controls::UpdateMsg::Button { controlId: ref id
             , updateType: ref ut
             } => { 
      // find the control in the map.
      let mut arghs = Vec::new();
      arghs.push (
        match ut { 
          &controls::ButtonUpType::Pressed => osc::Argument::s("b_pressed"),
          &controls::ButtonUpType::Unpressed => osc::Argument::s("b_unpressed"),
        });
      let msg = osc::Message { path: ctrl.oscname(), arguments: arghs };
      msg.serialize() 
    },
    &controls::UpdateMsg::Slider  { controlId: ref id
            , updateType: ref ut
            , location: ref loc
            } => {
      let mut arghs = Vec::new();
      arghs.push (
        match ut { 
          &controls::SliderUpType::Pressed => osc::Argument::s("s_pressed"),
          &controls::SliderUpType::Moved => osc::Argument::s("s_moved"),
          &controls::SliderUpType::Unpressed => osc::Argument::s("s_unpressed"),
        });
      let l = *loc as f32;
      arghs.push(osc::Argument::f(l));
      let msg = osc::Message { path: ctrl.oscname(), arguments: arghs };
      msg.serialize() 
    },
    &controls::UpdateMsg::Label { controlId: ref id
             , label: ref labtext
             } => { 
      // find the control in the map.
      let mut arghs = Vec::new();
      arghs.push(osc::Argument::s("l_label"));
      arghs.push(osc::Argument::s(&labtext[..]));
      let msg = osc::Message { path: ctrl.oscname(), arguments: arghs };
      msg.serialize() 
    },
   } 
} 

fn oscToCtrlUpdate(om: &osc::Message, cnm: &controls::controlNameMap) -> Result<controls::UpdateMsg, Box<std::error::Error> >
{
  // find the control by name.  
  let cid = try_opt_resbox!(cnm.get(om.path), "control name not found!");

  match om.arguments.len() {
    1 => {
      let evt = &om.arguments[0];
      match evt {
        &osc::Argument::s("b_pressed") => {
          // blah
          Ok(controls::UpdateMsg::Button 
            { controlId: cid.clone(), updateType: controls::ButtonUpType::Pressed })
        }, 
        &osc::Argument::s("b_unpressed") => {
          Ok(controls::UpdateMsg::Button 
            { controlId: cid.clone(), updateType: controls::ButtonUpType::Unpressed })
        },
        _ => Err(Box::new(Error::new(ErrorKind::Other, "invalid button event"))),
      }
    },
    2 => {
      match (&om.arguments[0], &om.arguments[1]) {
        (&osc::Argument::s("s_pressed"), &osc::Argument::f(loc))  => {
          Ok(controls::UpdateMsg::Slider 
            { controlId: cid.clone(), 
              updateType: controls::SliderUpType::Pressed,
              location: loc as f64 })
        },
        (&osc::Argument::s("s_moved"), &osc::Argument::f(loc))  => {
          Ok(controls::UpdateMsg::Slider 
            { controlId: cid.clone(), 
              updateType: controls::SliderUpType::Moved,
              location: loc as f64 })
        },
        (&osc::Argument::s("s_unpressed"), &osc::Argument::f(loc))  => {
          Ok(controls::UpdateMsg::Slider 
            { controlId: cid.clone(), 
              updateType: controls::SliderUpType::Unpressed,
              location: loc as f64 })
        },
        (&osc::Argument::s("l_label"), &osc::Argument::s(txt))  => {
          Ok(controls::UpdateMsg::Label
            { controlId: cid.clone() 
            , label: txt.to_string()
            })
        },
        _ => Err(Box::new(Error::new(ErrorKind::Other, "invalid slider event"))),
      }
    },
    _ => Err(Box::new(Error::new(ErrorKind::Other, "invalid osc message"))),
  } 
}

fn oscmain( socket: UdpSocket, 
            oscsendip: String, 
            mut bc: broadcaster::Broadcaster, 
            cm: Arc<Mutex<controls::controlMap>>, 
            cnm: controls::controlNameMap)
              -> Result<String, Error> 
{ 
  let mut buf = [0; 1000];

  loop { 
    let (amt, src) = try!(socket.recv_from(&mut buf));

    println!("length: {}", amt);
    let inmsg = match osc::Message::deserialize(&buf[.. amt]) {
      Ok(m) => m,
      Err(e) => {
          return Err(Error::new(ErrorKind::Other, "invalid osc message!"));
        },
      };

    match oscToCtrlUpdate(&inmsg, &cnm) {
      Ok(updmsg) => { 
        let mut cmunlock = cm.lock().unwrap();
        match cmunlock.get_mut(controls::getUmId(&updmsg)) {
          Some(ctl) => {
            (*ctl).update(&updmsg);
          
            let val = controls::encodeUpdateMessage(&updmsg); 
            println!("sending control update {:?}", val);
            match serde_json::ser::to_string(&val) { 
              Ok(s) => bc.broadcast(Message::Text(s)), 
              Err(_) => ()
            }
            ()
           },
          None => (),
        }
      },
      Err(e) => println!("osc decode error: {:?}", e),
    };
 
    println!("osc message received {} {:?}", inmsg.path, inmsg.arguments );
  }    
}


