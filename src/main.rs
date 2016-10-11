extern crate websocket;

use std::thread;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::path::Path;
use std::env;
use std::str;
use std::io::Read;
use std::io::Write;
use std::io::{Error,ErrorKind};
use std::string::*;
use std::collections::BTreeMap;
use std::fmt::Debug;

use std::net::UdpSocket;
use std::borrow::Cow;

use websocket::{Server, Message, Sender, Receiver};
use websocket::header::WebSocketProtocol;
use websocket::stream::WebSocketStream;
use websocket::message::Type;

extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

#[macro_use]
mod tryopt;
mod stringerror;
mod controls;
mod broadcaster;
mod stringDefaults; 

extern crate tinyosc;
use tinyosc as osc;

extern crate serde_json;
use serde_json::Value;
use serde_json::ser;

fn loadString(file_name: &str) -> Result<String, Box<std::error::Error> >
{
  let path = &Path::new(&file_name);
  let mut inf = try!(File::open(path));
  let mut result = String::new();
  let len = try!(inf.read_to_string(&mut result));
  Ok(result)
}

fn writeString(text: &str, file_name: &str) -> Result<(), Box<std::error::Error> >
{
  let path = &Path::new(&file_name);
  let mut inf = try!(File::create(path));
  inf.write(text.as_bytes());
  Ok(())
}
      
fn makeDefaultPrefs(guifile: &str) -> BTreeMap<String, Value>
{
    let mut map = BTreeMap::new();
    map.insert(String::from("guifile"), Value::String(guifile.to_string()));
    map.insert(String::from("ip"), Value::String("0.0.0.0".to_string()));
    map.insert(String::from("http-port"), Value::String("3030".to_string()));
    map.insert(String::from("websockets-port"), Value::String("1234".to_string()));
    map.insert(String::from("oscrecvip"), Value::String("localhost:9000".to_string()));
    map.insert(String::from("oscsendip"), Value::String("localhost:7".to_string()));
    
    map
}   

fn main() {
  // read in the settings json.
  let args = env::args();
  let mut iter = args.skip(1); // skip the program name
  match iter.next() {
    Some(file_name) => {
      if file_name == "-mkconfigs" 
      {
        let a = iter.next();
        let b = iter.next();
        match (a,b) { 
          (Some(prefsFilename), Some(guiFilename)) => {
            let value = Value::Object(makeDefaultPrefs(&guiFilename[..]));
           
            if let Ok(sv) = serde_json::to_string_pretty(&value) {
              match writeString(&sv[..], &prefsFilename[..]) {
                Ok(()) => println!("wrote example server config to file: {}", prefsFilename),
                Err(e) => println!("error writing default config file: {:?}", e),
              }
              match writeString(stringDefaults::sampleGuiConfig, &guiFilename[..]) {
                Ok(()) => println!("wrote example gui config to file: {}", guiFilename),
                Err(e) => println!("error writing default config file: {:?}", e),
              }
            }
          }
          (Some(prefsFilename), _) => { 
            let value = Value::Object(makeDefaultPrefs("gui.conf"));
           
            if let Ok(sv) = serde_json::to_string_pretty(&value) {
              match writeString(&sv[..], &file_name[..]) {
                Ok(()) => println!("wrote server config to file: {}", file_name),
                Err(e) => println!("error writing default config file: {:?}", e),
              }
            }
          }
          _ => println!("no config filenames!"),
        }
      }
      else {
        match startserver(&file_name) {
          Err(e) => println!("error starting server: {:?}", e),
          _ => (),
        }
      }

    }
    None => {
      println!("oscpad syntax: ");
      println!("oscpad <json config file>");
      println!("oscpad -mkconfigs <example server config file> <example gui config file>");
    }
  }
}

pub struct ControlInfo {
  cm: controls::controlMap,
  guijson: String,
}

fn startserver(file_name: &String) -> Result<(), Box<std::error::Error> >
{
    println!("loading config file: {}", file_name);
    let configstring = try!(loadString(&file_name));

    // read config file as json
    let configval: Value = try!(serde_json::from_str(&configstring[..]));

    let obj = try_opt_resbox!(configval.as_object(), "config file is not a json object!");
    
    let guifilename = 
      try_opt_resbox!(
        try_opt_resbox!(obj.get("guifile"), 
                        "'guifile' not found in config file").as_string(), 
        "failed to convert to string");
    let guistring = try!(loadString(&guifilename[..]));

    let ip = String::new() + 
      try_opt_resbox!(
        try_opt_resbox!(obj.get("ip"), 
                       "'ip' not found!").as_string(), 
        "'ip' not a string!");

    let http_port = String::new() + 
      try_opt_resbox!(
        try_opt_resbox!(obj.get("http-port"), 
                       "'http-port' not found!").as_string(), 
        "'http-port' not a string!");
  
    let mut http_ip = String::from(ip.as_str());
    http_ip.push_str(":");
    http_ip.push_str(&http_port);

    let websockets_port = String::new() + 
      try_opt_resbox!(
        try_opt_resbox!(obj.get("websockets-port"), 
                       "'websockets-port' not found!").as_string(), 
        "'websockets-port' not a string!");

    let mut websockets_ip = String::from(ip.as_str());
    websockets_ip.push_str(":");
    websockets_ip.push_str(&websockets_port);
    
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

    let htmltemplate = {  
      match obj.get("htmlfile") {
        Some(fname) => { 
          let htmlfilename = try_opt_resbox!(fname.as_string(), "failed to convert html file to string!");
          try!(loadString(&htmlfilename[..]))
        }
        None => stringDefaults::mainhtml.to_string()
      }
    };
    
    let htmlstring = htmltemplate.replace("<websockets-port>", &websockets_port);

    // println!("{}", htmlstring);

    let guival: Value = try!(serde_json::from_str(&guistring[..])); 

    let blah = try!(controls::deserializeRoot(&guival));

    println!("title: {} rootcontroltype: {} ", 
      blah.title, blah.rootControl.controlType());
    println!("controls: {:?}", blah.rootControl);

    // from control tree, make a map of ids->controls.
    let mapp = controls::makeControlMap(&*blah.rootControl);
    let guijson = guistring.clone();

    let ci = ControlInfo { cm: mapp, guijson: guijson };

    let cmshare = Arc::new(Mutex::new(ci));
    let wscmshare = cmshare.clone();
    let oscsocket = try!(UdpSocket::bind(&oscrecvip[..]));
    // for sending, bind to this.  if we bind to localhost, we can't
    // send messages to other machines.  
    let oscsendsocket = try!(UdpSocket::bind("0.0.0.0:0"));
    let bc = broadcaster::Broadcaster::new();
    let wsos = try!(oscsendsocket.try_clone());
    let wsbc = bc.clone();
    let wsoscsendip = oscsendip.clone();

    thread::spawn(move || { 
      match oscmain(oscsocket, bc, cmshare) {
        Err(e) => println!("oscmain exited with error: {:?}", e),
        Ok(_) => (),
      }
      }); 

    thread::spawn(move || { 
      match websockets_main(websockets_ip, wscmshare, wsbc, wsoscsendip, wsos) {
        Ok(_) => (),
        Err(e) => println!("error in websockets_main: {:?}", e),
      }
    });

    try!(Iron::new(move |req: &mut Request| {
        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, &*htmlstring)))
    }).http(&http_ip[..]));

    Ok(())
}

fn websockets_main( ipaddr: String, 
                    ci: Arc<Mutex<ControlInfo>>,
                    broadcaster: broadcaster::Broadcaster, 
                    oscsendip: String, 
                    oscsocket: UdpSocket ) 
                  -> Result<(), Box<std::error::Error> >
{
	let server = try!(Server::bind(&ipaddr[..]));

	for connection in server {

    println!("new websockets connection!");
		// Spawn a new thread for each connection.
    
    let sci = ci.clone();
    let osock = try!(oscsocket.try_clone());
    let osend = oscsendip.clone();
    let broadcaster = broadcaster.clone();

    let conn = try!(connection);
    thread::spawn(move || {
      match websockets_client(conn,
                            sci,
                            broadcaster,
                            osend,
                            osock) {
        Ok(_) => (), 
        Err(e) => {
          println!("error in websockets thread: {:?}", e);
          ()
        },
      }
    });
  } 

  Ok(())
}

fn websockets_client(connection: websocket::server::Connection<websocket::stream::WebSocketStream, websocket::stream::WebSocketStream>,
                    ci: Arc<Mutex<ControlInfo>>,
                    mut broadcaster: broadcaster::Broadcaster, 
                    oscsendip: String, 
                    oscsocket: UdpSocket 
                    ) -> Result<(), Box<std::error::Error> >
{
  // Get the request
  let request = try!(connection.read_request());
  // Keep the headers so we can check them
  let headers = request.headers.clone(); 
  
  try!(request.validate()); // Validate the request
  
  let mut response = request.accept(); // Form a response
  
  if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
    if protocols.contains(&("rust-websocket".to_string())) {
      // We have a protocol we want to use
      response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
    }
  }
  
  let mut client = try!(response.send()); // Send the response
  
  let ip = try!(client.get_mut_sender()
                  .get_mut()
                  .peer_addr());
  
  println!("Websocket connection from {}", ip);

  // send up the json of the current controls.
  {
    let sci = ci.lock().unwrap();

    let updarray = controls::cmToUpdateArray(&sci.cm);
  
    // build json message containing both guijson and the updarray.
    // let updvals = updarray.into_iter().map(|x|{controls::encodeUpdateMessage(&x)}).collect();

    let mut updvals = Vec::new();

    for upd in updarray { 
      let um = controls::encodeUpdateMessage(&upd);
      updvals.push(um);
    }
   
    let mut guival: Value = try!(serde_json::from_str(&sci.guijson[..]));

    match guival.as_object_mut() {
      Some(obj) => {
        obj.insert("state".to_string(), Value::Array(updvals));
        ()
      },
      None => (),
    }
  
    let guistring = try!(serde_json::ser::to_string(&guival));
    let message = Message::text(guistring);
    try!(client.send_message(&message));
  }
 
  let (sender, mut receiver) = client.split();

  let sendmeh = Arc::new(Mutex::new(sender));
  
  broadcaster.register(sendmeh.clone());      
  
  for msg in receiver.incoming_messages() {
    let message: Message = try!(msg);
    // println!("message: {:?}", message);

    match message.opcode {
      Type::Close => {
        let message = Message::close();
        // let mut sender = try!(sendmeh.lock());
        let mut sender = sendmeh.lock().unwrap();
        try!(sender.send_message(&message));
        println!("Client {} disconnected", ip);
        return Ok(());
      }
      Type::Ping => {
        println!("Message::Ping(data)");
        let message = Message::pong(message.payload);
        let mut sender = sendmeh.lock().unwrap();
        try!(sender.send_message(&message));
      }
      Type::Text => {
        let u8 = message.payload.to_owned();
        let str = try!(std::str::from_utf8(&*u8));
        let jsonval: Value = try!(serde_json::from_str(str));
        let s_um = controls::decodeUpdateMessage(&jsonval);
        match s_um { 
          Some(updmsg) => {
            let mut sci  = ci.lock().unwrap();
            let cntrl = sci.cm.get_mut(controls::getUmId(&updmsg));
            match cntrl {
              Some(x) => {
                (*x).update(&updmsg);
                println!("some x: {:?}", *x);
                broadcaster.broadcast_others(&ip, Message::text(str));
                match ctrlUpdateToOsc(&updmsg, &**x) { 
                  Ok(v) => oscsocket.send_to(&v, &oscsendip[..]),
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
        println!("unknown websockets msg: {:?}", message);
      }
    }
  }

  Ok(())
}

// TODO:
// break out osc encode/decode into a separate file, looking to be a 
// separate project at some point.
// how should osc messages be structured?
//  - seperate messages for each control attribute?
//  - named pairs, attrib + amount
//  - positional: <location> <state> <labelval>
//  I'm tending towards the named pairs.  But, look at the client code and
// see if that would be an undue hardship.  

fn ctrlUpdateToOsc(um: &controls::UpdateMsg, ctrl: &controls::Control) -> Result<Vec<u8>, Error>
{
  match um {
    &controls::UpdateMsg::Button { controlId: ref id
                                 , label: ref optLab
                                 , state: ref st
                                 } => { 
      // find the control in the map.
      let mut arghs = Vec::new();
      if let &Some(ref state) = st {
        arghs.push (
          match state { 
            &controls::ButtonState::Pressed => osc::Argument::s("b_pressed"),
            &controls::ButtonState::Unpressed => osc::Argument::s("b_unpressed"),
          })
        };

      if let &Some(ref lb) = optLab {
        arghs.push(osc::Argument::s("l_label"));
        // arghs.push(osc::Argument::s(lr));  // &labs[..]));
        arghs.push(osc::Argument::s(&lb[..]));
      };
        
      let msg = osc::Message { path: ctrl.oscname(), arguments: arghs };
      msg.serialize() 
    },
    &controls::UpdateMsg::Slider  { controlId: ref id
                                  , label: ref lb
                                  , state: ref st
                                  , location: ref loc
                                  } => {
      let mut arghs = Vec::new();
      if let &Some(ref state) = st {
        arghs.push (
          match state { 
            &controls::SliderState::Pressed => osc::Argument::s("s_pressed"),
            &controls::SliderState::Unpressed => osc::Argument::s("s_unpressed"),
          });
        };
      if let &Some(ref location) = loc { 
        let l = *location as f32;
        arghs.push(osc::Argument::f(l));
        };
      if let &Some(ref label) = lb {
        arghs.push(osc::Argument::s("l_label"));
        arghs.push(osc::Argument::s(&label[..]));
      };
      
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

/*

parsing the osc messages:

 "b_pressed", "b_


*/

// TODO: find the control itself and check its type.
// then build an update message based on that type.

// building the update message based on type:
//  - create an update message with all None
//  - make a function that takes the args array and an index.
//  - check for the various things at the index.
//  - recursively call self with updated update message.

/*
pub trait ControlUpdate : Debug + Send {
  fn oscToUpdateMsg(&self, om: &osc::Message) -> Option<controls::UpdateMsg>;
}

impl ControlUpdate for controls::Slider
{  
  fn oscToUpdateMsg(&self, om: &osc::Message) -> Option<controls::UpdateMsg> {
    None
    }
}

impl ControlUpdate for controls::Button
{  
  fn oscToUpdateMsg(&self, om: &osc::Message) -> Option<controls::UpdateMsg> {
    None
    }
}

impl ControlUpdate for controls::Label
{  
  fn oscToUpdateMsg(&self, om: &osc::Message) -> Option<controls::UpdateMsg> {
    None
    }
}

impl ControlUpdate for controls::Sizer
{  
  fn oscToUpdateMsg(&self, om: &osc::Message) -> Option<controls::UpdateMsg> {
    None
    }
}
*/

/*
impl ControlUpdate for controls::Slider
{  
  fn oscToUpdateMsg(&self, om: &osc::Message) -> Option<controls::UpdateMsg::Slider> {
    None
    }
}

impl ControlUpdate for controls::Button
{  
  fn oscToUpdateMsg(&self, om: &osc::Message) -> Option<controls::UpdateMsg::Button> {
    None
    }
}

impl ControlUpdate for controls::Label
{  
  fn oscToUpdateMsg(&self, om: &osc::Message) -> Option<controls::UpdateMsg::Label> {
    None
    }
}

impl ControlUpdate for controls::Sizer
{  
  fn oscToUpdateMsg(&self, om: &osc::Message) -> Option<controls::UpdateMsg> {
    None
    }
}


*/

// wow this code is hideous!  do not look!
fn parseOscSliderUpdate(om: &osc::Message,
                        argIndex: usize, 
                        update: controls::UpdateMsg )
  -> Result<controls::UpdateMsg, Box<std::error::Error> >
{
  if argIndex >= om.arguments.len() {
    Ok(update)
  }
  else {
    match om.arguments[argIndex] { 
      osc::Argument::s("pressed") => {
        let mut newupd = update.clone();
        match newupd { 
          controls::UpdateMsg::Button { ref mut state, .. } => 
            { *state = Some(controls::ButtonState::Pressed); },
          controls::UpdateMsg::Slider { ref mut state, .. } => 
            { *state = Some(controls::SliderState::Pressed); },
          _ => (),
          };
        parseOscSliderUpdate(om, 
                             argIndex + 1, newupd)
        },
      osc::Argument::s("unpressed") => {
        let mut newupd = update.clone();
        match newupd { 
          controls::UpdateMsg::Button { ref mut state, .. } => 
            { *state = Some(controls::ButtonState::Unpressed); },
          controls::UpdateMsg::Slider { ref mut state, .. } => 
            { *state = Some(controls::SliderState::Unpressed); },
          _ => (),
          };
        parseOscSliderUpdate(om, 
                             argIndex + 1, newupd)
        },
      osc::Argument::s("location") => {
        if (om.arguments.len() <= argIndex + 1)
        {
          Err(Box::new(Error::new(ErrorKind::Other, "location should be followed by a number between 0.0 and 1.0!")))
        }
        else
        {
          match &om.arguments[argIndex + 1] {
            &osc::Argument::f(loc) => { 
              let mut newupd = update.clone();
              match newupd { 
                controls::UpdateMsg::Slider { ref mut location, .. } => 
                  { *location = Some(loc as f64); },
                _ => (),
                };
              parseOscSliderUpdate(om, argIndex + 2, newupd)
              },
            _ => Err(Box::new(Error::new(ErrorKind::Other, "location should be followed by a number between 0.0 and 1.0!"))),
            }
          }
        },
      osc::Argument::s("label") => {
        if (om.arguments.len() <= argIndex + 1)
        {
          Err(Box::new(Error::new(ErrorKind::Other, "'label' should be followed by a string!")))
        }
        else
        {
          match &om.arguments[argIndex + 1] {
            &osc::Argument::s(txt) => { 
              let mut newupd = update.clone();
              match newupd { 
                controls::UpdateMsg::Button { ref mut label, .. } => 
                  { *label= Some(txt.to_string()); },
                controls::UpdateMsg::Slider { ref mut label, .. } => 
                  { *label= Some(txt.to_string()); },
                controls::UpdateMsg::Label { ref mut label, .. } => 
                  { *label= txt.to_string(); },
                };
              parseOscSliderUpdate(om, argIndex + 2, newupd)
              },
            _ => Err(Box::new(Error::new(ErrorKind::Other, "location should be followed by a number between 0.0 and 1.0!"))),
            }
          }
        },
      _ => Err(Box::new(Error::new(ErrorKind::Other, "unknown keyword!"))),
      }
  }
  
}
                        

fn oscToCtrlUpdate(om: &osc::Message, 
                   cnm: &controls::controlNameMap,
                   cm: &controls::controlMap) 
   -> Result<controls::UpdateMsg, Box<std::error::Error> >
{
  // find the control by name.  
  let cid = try_opt_resbox!(cnm.get(om.path), "control name not found!");
  
  let control = try_opt_resbox!(cm.get(cid), "control not found!");

  match &(*control.controlType()) {
   "slider" => parseOscSliderUpdate(om, 0, controls::UpdateMsg::Slider 
            { controlId: cid.clone(), 
              state: None,
              location: None,
              label: None }),  
   _ => Err(Box::new(Error::new(ErrorKind::Other, "unknown type"))),
   }

/*
  match **control {
   controls::Slider {..} => println!("slider"),
   controls::Button {..} => println!("button"),
   controls::Label {..} => println!("label"),
   };
*/


  /*
  match om.arguments.len() {
    1 => {
      let evt = &om.arguments[0];
      match evt {
        &osc::Argument::s("b_pressed") => {
          // blah
          Ok(controls::UpdateMsg::Button 
            { controlId: cid.clone()
            , state: Some(controls::ButtonState::Pressed) 
            , label: Some(controls::ButtonState::Pressed) 
            })
        }, 
        &osc::Argument::s("b_unpressed") => {
          Ok(controls::UpdateMsg::Button 
            { controlId: cid.clone()
            , state: Some(controls::ButtonState::Unpressed) })
        },
        _ => Err(Box::new(Error::new(ErrorKind::Other, "invalid button event"))),
      }
    },
    2 => {
      match (&om.arguments[0], &om.arguments[1]) {
        (&osc::Argument::s("s_pressed"), &osc::Argument::f(loc))  => {
          Ok(controls::UpdateMsg::Slider 
            { controlId: cid.clone(), 
              state: Some(controls::SliderState::Pressed),
              location: Some(loc as f64) })
        },
        (&osc::Argument::s("s_moved"), &osc::Argument::f(loc))  => {
          Ok(controls::UpdateMsg::Slider 
            { controlId: cid.clone(), 
              state: Some(controls::SliderState::Moved),
              location: Some(loc as f64) })
        },
        (&osc::Argument::s("s_unpressed"), &osc::Argument::f(loc))  => {
          Ok(controls::UpdateMsg::Slider 
            { controlId: cid.clone(), 
              state: Some(controls::SliderState::Unpressed),
              location: Some(loc as f64) })
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
  */
}

fn oscmain( recvsocket: UdpSocket, 
            mut bc: broadcaster::Broadcaster, 
            ci: Arc<Mutex<ControlInfo>>)
              -> Result<String, Box<std::error::Error> >
{ 
  let mut buf = [0; 10000];

  let mut cnm = {
    let sci  = ci.lock().unwrap(); 
    controls::controlMapToNameMap(&sci.cm)
    };

  loop { 
    let (amt, src) = try!(recvsocket.recv_from(&mut buf));

    println!("length: {}", amt);
    match osc::Message::deserialize(&buf[.. amt]) {
      Err(e) => {
          println!("invalid osc messsage: {:?}", e)
        },
      Ok(inmsg) => {
        let sci  = ci.lock().unwrap(); 

        match oscToCtrlUpdate(&inmsg, &cnm, &sci.cm) {
          Ok(updmsg) => { 
            let mut sci  = ci.lock().unwrap(); 
            match sci.cm.get_mut(controls::getUmId(&updmsg)) {
              Some(ctl) => {
                (*ctl).update(&updmsg);
              
                let val = controls::encodeUpdateMessage(&updmsg); 
                println!("sending control update {:?}", val);
                match serde_json::ser::to_string(&val) { 
                  Ok(s) => bc.broadcast(Message::text(s)), 
                  Err(_) => ()
                }
                ()
               },
              None => (),
            }
          },
          Err(e) => {
            if inmsg.path == "guiconfig" && inmsg.arguments.len() > 0 {
              // is this a control config update instead?
              match &inmsg.arguments[0] {
                &osc::Argument::s(guistring) => {
                  // build a new control map with this.
                  // deserialize the gui string into json.
                  match serde_json::from_str(&guistring[..]) { 
                    Ok(guival) => { 
                      match controls::deserializeRoot(&guival) {
                        Ok(controltree) => { 
                          println!("new control layout recieved!");

                          println!("title: {} count: {} ", 
                            controltree.title, controltree.rootControl.controlType());
                          println!("controls: {:?}", controltree.rootControl);

                          // from control tree, make a map of ids->controls.
                          let mapp = controls::makeControlMap(&*controltree.rootControl);
                          cnm = controls::controlMapToNameMap(&mapp);

                          let mut sci = ci.lock().unwrap(); 
                          sci.cm = mapp;
                          sci.guijson = guistring.to_string();
                          bc.broadcast(Message::text(guistring.to_string()));
                        },
                        Err(e) => println!("error reading guiconfig from json: {:?}", e),
                      }
                    },
                    Err(e) => println!("error reading guiconfig json: {:?}", e),
                  }
                },
                _ => println!("osc decode error: {:?}", e),
              }
            }
          }
        };
     
        println!("osc message received {} {:?}", inmsg.path, inmsg.arguments);
      }    
    }
  }

  // Ok("ok".to_string())
}


