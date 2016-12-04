
use std::thread;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Read;
use std::io::Write;
use std::io::{Error,ErrorKind};
use std::string::*;
use std::collections::BTreeMap;
use std::fmt::format;

use std::net::UdpSocket;

/*
extern crate websocket;
use websocket::{Server, Message, Sender, Receiver};
use websocket::header::WebSocketProtocol;
use websocket::message::Type;

extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
*/

use touchpage::startserver;
use touchpage::control_updates as cu;
extern crate touchpage;


#[macro_use]
mod tryopt;
mod stringerror;
// mod controls;
// mod broadcaster;
// mod string_defaults; 

extern crate tinyosc;
use tinyosc as osc;

extern crate serde_json;
use serde_json::Value;

fn load_string(file_name: &str) -> Result<String, Box<std::error::Error> >
{
  let path = &Path::new(&file_name);
  let mut inf = try!(File::open(path));
  let mut result = String::new();
  try!(inf.read_to_string(&mut result));
  Ok(result)
}

fn write_string(text: &str, file_name: &str) -> Result<(), Box<std::error::Error> >
{
  let path = &Path::new(&file_name);
  let mut inf = try!(File::create(path));
  match inf.write(text.as_bytes()) { 
    Ok(_) => Ok(()),
    Err(e) => Err(Box::new(e)),
    }
}
      
fn make_default_prefs(guifile: &str) -> BTreeMap<String, Value>
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
      /*
      if file_name == "-mkconfigs" 
      {
        let a = iter.next();
        let b = iter.next();
        match (a,b) { 
          (Some(prefs_filename), Some(gui_filename)) => {
            let value = Value::Object(make_default_prefs(&gui_filename[..]));
           
            if let Ok(sv) = serde_json::to_string_pretty(&value) {
              match write_string(&sv[..], &prefs_filename[..]) {
                Ok(()) => println!("wrote example server config to file: {}", prefs_filename),
                Err(e) => println!("error writing default config file: {:?}", e),
              }
              match write_string(string_defaults::SAMPLE_GUI_CONFIG, &gui_filename[..]) {
                Ok(()) => println!("wrote example gui config to file: {}", gui_filename),
                Err(e) => println!("error writing default config file: {:?}", e),
              }
            }
          }
          (Some(prefs_filename), _) => { 
            let value = Value::Object(make_default_prefs("gui.conf"));
           
            if let Ok(sv) = serde_json::to_string_pretty(&value) {
              match write_string(&sv[..], &prefs_filename[..]) {
                Ok(()) => println!("wrote server config to file: {}", prefs_filename),
                Err(e) => println!("error writing default config file: {:?}", e),
              }
            }
          }
          _ => println!("no config filenames!"),
        }
      }
      else {
      */
        match startserver_w_config(&file_name) {
          Err(e) => println!("error starting server: {:?}", e),
          _ => (),
        }
      // }

    }
    None => {
      println!("oscpad syntax: ");
      println!("oscpad <json config file>");
      println!("oscpad -mkconfigs <example server config file> <example gui config file>");
    }
  }
}

fn printupdatemsg(update: &cu::UpdateMsg) -> ()
{
  println!("update callback called! {:?}", update);
  ()
}

fn startserver_w_config(file_name: &String) -> Result<(), Box<std::error::Error> >
{
    println!("loading config file: {}", file_name);
    let configstring = try!(load_string(&file_name));

    // read config file as json
    let configval: Value = try!(serde_json::from_str(&configstring[..]));

    let oobj = try_opt_resbox!(configval.as_object(), "config file is not a json object!");
    let obj = oobj.clone();
  
    let guifilename = 
      try_opt_resbox!(
        try_opt_resbox!(obj.get("guifile"), 
                        "'guifile' not found in config file").as_string(), 
        "failed to convert to string");
    let guistring = try!(load_string(&guifilename[..]));
    // let guistring = "wtf";

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


    let oscsocket = try!(UdpSocket::bind(&oscrecvip[..]));

    // for sending, bind to this.  if we bind to localhost, we can't
    // send messages to other machines.  
    let oscsendsocket = try!(UdpSocket::bind("0.0.0.0:0"));
//    let bc = broadcaster::Broadcaster::new();
    // let wsos = try!(oscsendsocket.try_clone());
    // let wsoscsendip = oscsendip.clone();

    // let ci = ControlInfo { cm: mapp, guijson: guijson };
    // let cmshare = Arc::new(Mutex::new(ci));

      match touchpage::startserver(guistring.as_str(), printupdatemsg, ip.as_str(), http_port.as_str(), websockets_port.as_str(), None) {
        Err(e) => println!("startserver returned error: {:?}", e),
        Ok(_) => (),
      };


//    thread::spawn(move || { 
      // match oscmain(oscsocket, bc, cmshare) {
      match oscmain(oscsocket) {
        Err(e) => println!("oscmain exited with error: {:?}", e),
        Ok(_) => (),
      };
 //     }); 


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

/*
fn ctrl_update_to_osc(um: &control_updates::UpdateMsg, ctrl: &controls::Control) -> Result<Vec<u8>, Error>
{
  match um {
    &controls::UpdateMsg::Button { control_id: _
                                 , label: ref opt_lab
                                 , state: ref st
                                 } => { 
      // find the control in the map.
      let mut arghs = Vec::new();
      if let &Some(ref state) = st {
        arghs.push (
          match state { 
            &cu::ButtonState::Pressed => osc::Argument::s("pressed"),
            &cu::ButtonState::Unpressed => osc::Argument::s("unpressed"),
          })
        };

      if let &Some(ref lb) = opt_lab {
        arghs.push(osc::Argument::s("label"));
        // arghs.push(osc::Argument::s(lr));  // &labs[..]));
        arghs.push(osc::Argument::s(&lb[..]));
      };
        
      let msg = osc::Message { path: ctrl.oscname(), arguments: arghs };
      msg.serialize() 
    },
    &controls::UpdateMsg::Slider  { control_id: _
                                  , label: ref lb
                                  , state: ref st
                                  , location: ref loc
                                  } => {
      let mut arghs = Vec::new();
      if let &Some(ref state) = st {
        arghs.push (
          match state { 
            &cu::SliderState::Pressed => osc::Argument::s("pressed"),
            &cu::SliderState::Unpressed => osc::Argument::s("unpressed"),
          });
        };
      if let &Some(ref location) = loc { 
        let l = *location as f32;
        arghs.push(osc::Argument::s("location"));
        arghs.push(osc::Argument::f(l));
        };
      if let &Some(ref label) = lb {
        arghs.push(osc::Argument::s("label"));
        arghs.push(osc::Argument::s(&label[..]));
      };
      
      let msg = osc::Message { path: ctrl.oscname(), arguments: arghs };
      msg.serialize() 
    },
    &controls::UpdateMsg::Label { control_id: _ 
             , label: ref labtext
             } => { 
      // find the control in the map.
      let mut arghs = Vec::new();
      arghs.push(osc::Argument::s("label"));
      arghs.push(osc::Argument::s(&labtext[..]));
      let msg = osc::Message { path: ctrl.oscname(), arguments: arghs };
      msg.serialize() 
    },
   } 
} 
*/



// TODO: find the control itself and check its type.
// then build an update message based on that type.

// building the update message based on type:
//  - create an update message with all None
//  - make a function that takes the args array and an index.
//  - check for the various things at the index.
//  - recursively call self with updated update message.

// wow this code is hideous!  do not look!
fn parse_osc_control_update(om: &osc::Message,
                        arg_index: usize, 
                        update: cu::UpdateMsg )
  -> Result<cu::UpdateMsg, Box<std::error::Error> >
{
  if arg_index >= om.arguments.len() {
    Ok(update)
  }
  else {
    match om.arguments[arg_index] { 
      osc::Argument::s("pressed") => {
        let mut newupd = update.clone();
        match newupd { 
          cu::UpdateMsg::Button { ref mut state, .. } => 
            { *state = Some(cu::ButtonState::Pressed); },
          cu::UpdateMsg::Slider { ref mut state, .. } => 
            { *state = Some(cu::SliderState::Pressed); },
          _ => (),
          };
        parse_osc_control_update(om, 
                             arg_index + 1, newupd)
        },
      osc::Argument::s("unpressed") => {
        let mut newupd = update.clone();
        match newupd { 
          cu::UpdateMsg::Button { ref mut state, .. } => 
            { *state = Some(cu::ButtonState::Unpressed); },
          cu::UpdateMsg::Slider { ref mut state, .. } => 
            { *state = Some(cu::SliderState::Unpressed); },
          _ => (),
          };
        parse_osc_control_update(om, 
                             arg_index + 1, newupd)
        },
      osc::Argument::s("location") => {
        if om.arguments.len() <= arg_index + 1
        {
          Err(Box::new(Error::new(ErrorKind::Other, "location should be followed by a number between 0.0 and 1.0!")))
        }
        else
        {
          match &om.arguments[arg_index + 1] {
            &osc::Argument::f(loc) => { 
              let mut newupd = update.clone();
              match newupd { 
                cu::UpdateMsg::Slider { ref mut location, .. } => 
                  { *location = Some(loc as f64); },
                _ => (),
                };
              parse_osc_control_update(om, arg_index + 2, newupd)
              },
            _ => Err(Box::new(Error::new(ErrorKind::Other, "location should be followed by a number between 0.0 and 1.0!"))),
            }
          }
        },
      osc::Argument::s("label") => {
        if om.arguments.len() <= arg_index + 1
        {
          Err(Box::new(Error::new(ErrorKind::Other, "'label' should be followed by a string!")))
        }
        else
        {
          match &om.arguments[arg_index + 1] {
            &osc::Argument::s(txt) => { 
              let mut newupd = update.clone();
              match newupd { 
                cu::UpdateMsg::Button { ref mut label, .. } => 
                  { *label= Some(txt.to_string()); },
                cu::UpdateMsg::Slider { ref mut label, .. } => 
                  { *label= Some(txt.to_string()); },
                cu::UpdateMsg::Label { ref mut label, .. } => 
                  { *label= txt.to_string(); },
                };
              parse_osc_control_update(om, arg_index + 2, newupd)
              },
            _ => Err(Box::new(Error::new(ErrorKind::Other, "location should be followed by a number between 0.0 and 1.0!"))),
            }
          }
        },
      _ => Err(Box::new(Error::new(ErrorKind::Other, "unknown keyword!"))),
      }
  }
}
                        

/*
fn osc_to_ctrl_update(om: &osc::Message, 
                   cnm: &controls::ControlNameMap,
                   cm: &controls::ControlMap) 
   -> Result<controls::UpdateMsg, Box<std::error::Error> >
{
  // find the control by name.  

  // first the control id
  let cid = try_opt_resbox!(cnm.get(om.path), "control name not found!");
  
  // next the control itself.
  let control = try_opt_resbox!(cm.get(cid), "control not found!");

  // make an update message based on the control type.
  match &(*control.control_type()) {
   "slider" => parse_osc_control_update(om, 0, controls::UpdateMsg::Slider 
            { control_id: cid.clone(), 
              state: None,
              location: None,
              label: None }),  
   "button" => parse_osc_control_update(om, 0, controls::UpdateMsg::Button 
            { control_id: cid.clone(), 
              state: None,
              label: None }),  
   "label" => parse_osc_control_update(om, 0, controls::UpdateMsg::Label 
            { control_id: cid.clone(), 
              label: String::from("") }),  
   x => {
    let msg = format(format_args!("unknown type: {:?}", x));    
      Err(Box::new(Error::new(ErrorKind::Other, msg)))
    },
   }
}
*/

fn oscmain( recvsocket: UdpSocket )
//            mut bc: broadcaster::Broadcaster, 
//            ci: Arc<Mutex<ControlInfo>>)
              -> Result<String, Box<std::error::Error> >
{ 
  let mut buf = [0; 10000];

  /*
  let mut cnm = {
    let sci  = ci.lock().unwrap(); 
    controls::control_map_to_name_map(&sci.cm)
    };
    */

  loop { 
    let (amt, _) = try!(recvsocket.recv_from(&mut buf));

    match osc::Message::deserialize(&buf[.. amt]) {
      Err(e) => {
          println!("invalid osc messsage: {:?}", e)
        },
      Ok(inmsg) => {
        // let mut sci = ci.lock().unwrap(); 

        /*
        match osc_to_ctrl_update(&inmsg, &cnm, &sci.cm) {
          Ok(updmsg) => { 
            match sci.cm.get_mut(controls::get_um_id(&updmsg)) {
              Some(ctl) => {
                (*ctl).update(&updmsg);
                let val = controls::encode_update_message(&updmsg); 
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
                      match controls::deserialize_root(&guival) {
                        Ok(controltree) => { 
                          println!("new control layout recieved!");

                          println!("title: {} count: {} ", 
                            controltree.title, controltree.root_control.control_type());
                          println!("controls: {:?}", controltree.root_control);

                          // from control tree, make a map of ids->controls.
                          let mapp = controls::make_control_map(&*controltree.root_control);
                          cnm = controls::control_map_to_name_map(&mapp);
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
        */
    
        // print received afterwards, I guess for latency savings?
        println!("osc message received {} {:?}", inmsg.path, inmsg.arguments);
      }    
    }
  }
}


