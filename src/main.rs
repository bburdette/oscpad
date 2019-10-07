use std::collections::BTreeMap;
use std::env;
use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::string::*;
// use std::thread;

use std::net::UdpSocket;

// use touchpage::startserver;
use touchpage as tp;
use touchpage::control_nexus as cn;
use touchpage::control_updates as cu;
extern crate touchpage;

#[macro_use]
mod tryopt;
mod stringerror;

extern crate tinyosc;
use tinyosc as osc;

extern crate serde_json;
use serde_json::Value;

fn load_string(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = &Path::new(&file_name);
    let mut inf = try!(File::open(path));
    let mut result = String::new();
    try!(inf.read_to_string(&mut result));
    Ok(result)
}

fn write_string(text: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = &Path::new(&file_name);
    let mut inf = try!(File::create(path));
    match inf.write(text.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

fn make_default_prefs(guifile: &str) -> BTreeMap<String, Value> {
    let mut map = BTreeMap::new();
    map.insert(String::from("guifile"), Value::String(guifile.to_string()));
    map.insert(String::from("ip"), Value::String("0.0.0.0".to_string()));
    map.insert(String::from("http-port"), Value::String("3030".to_string()));
    map.insert(
        String::from("websockets-port"),
        Value::String("1234".to_string()),
    );
    map.insert(
        String::from("oscrecvip"),
        Value::String("localhost:9000".to_string()),
    );
    map.insert(
        String::from("oscsendip"),
        Value::String("127.0.0.1:7070".to_string()),
    );

    map
}

fn main() {
    // read in the settings json.
    let args = env::args();
    let mut iter = args.skip(1); // skip the program name
    match iter.next() {
        Some(file_name) => {
            if file_name == "-mkconfigs" {
                let a = iter.next();
                let b = iter.next();
                match (a, b) {
                    (Some(prefs_filename), Some(gui_filename)) => {
                        let value = Value::Object(make_default_prefs(&gui_filename[..]));

                        if let Ok(sv) = serde_json::to_string_pretty(&value) {
                            match write_string(&sv[..], &prefs_filename[..]) {
                                Ok(()) => println!(
                                    "wrote example server config to file: {}",
                                    prefs_filename
                                ),
                                Err(e) => println!("error writing default config file: {:?}", e),
                            }
                            match write_string(
                                touchpage::json::sample_gui_config(),
                                &gui_filename[..],
                            ) {
                                Ok(()) => {
                                    println!("wrote example gui config to file: {}", gui_filename)
                                }
                                Err(e) => println!("error writing default config file: {:?}", e),
                            }
                        }
                    }
                    (Some(prefs_filename), _) => {
                        let value = Value::Object(make_default_prefs("gui.conf"));

                        if let Ok(sv) = serde_json::to_string_pretty(&value) {
                            match write_string(&sv[..], &prefs_filename[..]) {
                                Ok(()) => {
                                    println!("wrote server config to file: {}", prefs_filename)
                                }
                                Err(e) => println!("error writing default config file: {:?}", e),
                            }
                        }
                    }
                    _ => println!("no config filenames!"),
                }
            } else {
                match startserver_w_config(&file_name) {
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

pub struct PrintUpdateMsg {}

impl cn::ControlUpdateProcessor for PrintUpdateMsg {
    fn on_update_received(&mut self, update: &cu::UpdateMsg, ci: &cn::ControlInfo) -> () {
        println!("update callback called! {:?}", update);
    }
}

fn printupdatemsg(update: &cu::UpdateMsg) -> () {
    println!("update callback called! {:?}", update);
    ()
}

pub struct SendOscMsg {
    oscsendsocket: UdpSocket,
    oscsendip: String,
}

impl cn::ControlUpdateProcessor for SendOscMsg {
    fn on_update_received(&mut self, update: &cu::UpdateMsg, ci: &cn::ControlInfo) -> () {
        match ctrl_update_to_osc(update, ci) {
            Ok(v) => match self.oscsendsocket.send_to(&v, &self.oscsendip[..]) {
                Ok(_) => (),
                Err(e) => {
                    println!("oscsendip: {:?}", self.oscsendip);
                    println!("error sending osc message: {:?}", e)
                }
            },
            Err(e) => println!("error building osc message: {:?}", e),
        };

        println!("update callback called! {:?}", update);
    }
}

fn startserver_w_config(file_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("loading config file: {}", file_name);
    let configstring = try!(load_string(&file_name));

    // read config file as json
    let configval: Value = try!(serde_json::from_str(&configstring[..]));

    let oobj = try_opt_resbox!(configval.as_object(), "config file is not a json object!");
    let obj = oobj.clone();

    let guifilename = try_opt_resbox!(
        try_opt_resbox!(obj.get("guifile"), "'guifile' not found in config file").as_string(),
        "failed to convert to string"
    );
    let guistring = try!(load_string(&guifilename[..]));

    let htmlfilename = match obj.get("htmlfile") {
        Some(val) => {
            println!("loading htmlfile: {:?}", val.as_string());
            val.as_string()
        }
        None => None,
    };

    let ip = String::new()
        + try_opt_resbox!(
            try_opt_resbox!(obj.get("ip"), "'ip' not found!").as_string(),
            "'ip' not a string!"
        );

    let http_port = String::new()
        + try_opt_resbox!(
            try_opt_resbox!(obj.get("http-port"), "'http-port' not found!").as_string(),
            "'http-port' not a string!"
        );

    let mut http_ip = String::from(ip.as_str());
    http_ip.push_str(":");
    http_ip.push_str(&http_port);

    let websockets_port = String::new()
        + try_opt_resbox!(
            try_opt_resbox!(obj.get("websockets-port"), "'websockets-port' not found!").as_string(),
            "'websockets-port' not a string!"
        );

    let oscrecvip = String::new()
        + try_opt_resbox!(
            try_opt_resbox!(obj.get("oscrecvip"), "'oscrecvip' not found!").as_string(),
            "'oscrecvip' not a string!"
        );

    let oscsendip = String::new()
        + try_opt_resbox!(
            try_opt_resbox!(obj.get("oscsendip"), "'oscsendip' not found!").as_string(),
            "'oscsendip' not a string!"
        );

    let oscsocket = try!(UdpSocket::bind(&oscrecvip[..]));

    // for sending, bind to this.  if we bind to localhost, we can't
    // send messages to other machines.

    let oscsendsocket = try!(UdpSocket::bind("0.0.0.0:0"));
    let wsos = try!(oscsendsocket.try_clone());
    let wsoscsendip = oscsendip.clone();

    let cup = Box::new(SendOscMsg {
        oscsendsocket: wsos,
        oscsendip: wsoscsendip,
    });

    println!("websockets port: {}", websockets_port);
    let control_server = try!(tp::websocketserver::startserver(
        guistring.as_str(),
        cup,
        ip.as_str(),
        websockets_port.as_str(),
        false,
    ));

    tp::webserver::startwebserver("localhost", "8000", websockets_port.as_str(), None);

    println!("after startwebserver");
    match oscmain(oscsocket, &control_server) {
        Err(e) => println!("oscmain exited with error: {:?}", e),
        Ok(_) => (),
    };

    Ok(())
}

// TODO:
// how should osc messages be structured?
//  - seperate messages for each control attribute?
//  - named pairs, attrib + amount
//  - positional: <location> <state> <labelval>
//  I'm tending towards the named pairs.  But, look at the client code and
// see if that would be an undue hardship.

fn ctrl_update_to_osc(um: &cu::UpdateMsg, ci: &cn::ControlInfo) -> Result<Vec<u8>, Error> {
    match um {
        &cu::UpdateMsg::Button {
            control_id: ref id,
            label: ref opt_lab,
            state: ref st,
        } => {
            // find the control in the map.
            match ci.get_name(&id) {
                Some(oscname) => {
                    let mut arghs = Vec::new();
                    if let &Some(ref state) = st {
                        arghs.push(match state {
                            &cu::ButtonState::Pressed => osc::Argument::s("pressed"),
                            &cu::ButtonState::Unpressed => osc::Argument::s("unpressed"),
                        })
                    };

                    if let &Some(ref lb) = opt_lab {
                        arghs.push(osc::Argument::s("label"));
                        arghs.push(osc::Argument::s(&lb[..]));
                    };

                    let msg = osc::Message {
                        path: oscname.as_str(),
                        arguments: arghs,
                    };
                    msg.serialize()
                }
                None => Err(Error::new(ErrorKind::NotFound, "osc id not found!")),
            }
        }
        &cu::UpdateMsg::Slider {
            control_id: ref id,
            label: ref lb,
            state: ref st,
            location: ref loc,
        } => match ci.get_name(&id) {
            Some(oscname) => {
                let mut arghs = Vec::new();
                if let &Some(ref state) = st {
                    arghs.push(match state {
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

                let msg = osc::Message {
                    path: oscname.as_str(),
                    arguments: arghs,
                };
                msg.serialize()
            }
            None => Err(Error::new(ErrorKind::NotFound, "osc id not found!")),
        },
        &cu::UpdateMsg::Label {
            control_id: ref id,
            label: ref labtext,
        } => match ci.get_name(&id) {
            Some(oscname) => {
                let mut arghs = Vec::new();
                arghs.push(osc::Argument::s("label"));
                arghs.push(osc::Argument::s(&labtext[..]));
                let msg = osc::Message {
                    path: oscname.as_str(),
                    arguments: arghs,
                };
                msg.serialize()
            }
            None => Err(Error::new(ErrorKind::NotFound, "osc id not found!")),
        },
    }
}

// TODO: find the control itself and check its type.
// then build an update message based on that type.

// building the update message based on type:
//  - create an update message with all None
//  - make a function that takes the args array and an index.
//  - check for the various things at the index.
//  - recursively call self with updated update message.

// wow this code is hideous!  do not look!
fn parse_osc_control_update(
    om: &osc::Message,
    arg_index: usize,
    update: cu::UpdateMsg,
) -> Result<cu::UpdateMsg, Box<dyn std::error::Error>> {
    if arg_index >= om.arguments.len() {
        Ok(update)
    } else {
        match om.arguments[arg_index] {
            osc::Argument::s("pressed") => {
                let mut newupd = update.clone();
                match newupd {
                    cu::UpdateMsg::Button { ref mut state, .. } => {
                        *state = Some(cu::ButtonState::Pressed);
                    }
                    cu::UpdateMsg::Slider { ref mut state, .. } => {
                        *state = Some(cu::SliderState::Pressed);
                    }
                    _ => (),
                };
                parse_osc_control_update(om, arg_index + 1, newupd)
            }
            osc::Argument::s("unpressed") => {
                let mut newupd = update.clone();
                match newupd {
                    cu::UpdateMsg::Button { ref mut state, .. } => {
                        *state = Some(cu::ButtonState::Unpressed);
                    }
                    cu::UpdateMsg::Slider { ref mut state, .. } => {
                        *state = Some(cu::SliderState::Unpressed);
                    }
                    _ => (),
                };
                parse_osc_control_update(om, arg_index + 1, newupd)
            }
            osc::Argument::s("location") => {
                if om.arguments.len() <= arg_index + 1 {
                    Err(Box::new(Error::new(
                        ErrorKind::Other,
                        "location should be followed by a number between 0.0 and 1.0!",
                    )))
                } else {
                    match &om.arguments[arg_index + 1] {
                        &osc::Argument::f(loc) => {
                            let mut newupd = update.clone();
                            match newupd {
                                cu::UpdateMsg::Slider {
                                    ref mut location, ..
                                } => {
                                    *location = Some(loc as f64);
                                }
                                _ => (),
                            };
                            parse_osc_control_update(om, arg_index + 2, newupd)
                        }
                        _ => Err(Box::new(Error::new(
                            ErrorKind::Other,
                            "location should be followed by a number between 0.0 and 1.0!",
                        ))),
                    }
                }
            }
            osc::Argument::s("label") => {
                if om.arguments.len() <= arg_index + 1 {
                    Err(Box::new(Error::new(
                        ErrorKind::Other,
                        "'label' should be followed by a string!",
                    )))
                } else {
                    match &om.arguments[arg_index + 1] {
                        &osc::Argument::s(txt) => {
                            let mut newupd = update.clone();
                            match newupd {
                                cu::UpdateMsg::Button { ref mut label, .. } => {
                                    *label = Some(txt.to_string());
                                }
                                cu::UpdateMsg::Slider { ref mut label, .. } => {
                                    *label = Some(txt.to_string());
                                }
                                cu::UpdateMsg::Label { ref mut label, .. } => {
                                    *label = txt.to_string();
                                }
                            };
                            parse_osc_control_update(om, arg_index + 2, newupd)
                        }
                        _ => Err(Box::new(Error::new(
                            ErrorKind::Other,
                            "location should be followed by a number between 0.0 and 1.0!",
                        ))),
                    }
                }
            }
            _ => Err(Box::new(Error::new(ErrorKind::Other, "unknown keyword!"))),
        }
    }
}

fn osc_to_ctrl_update(
    om: &osc::Message,
    cserver: &cn::ControlNexus,
) -> Result<cu::UpdateMsg, Box<dyn std::error::Error>> {
    // find the control by name.

    let updmsg = cserver.make_update_msg(om.path);

    match updmsg {
        Some(updmsg_yeah) => parse_osc_control_update(om, 0, updmsg_yeah),
        None => {
            let msg = format(format_args!("failed to update control: {:?}", om.path));
            Err(Box::new(Error::new(ErrorKind::Other, msg)))
        }
    }
}

fn oscmain(
    recvsocket: UdpSocket,
    control_server: &cn::ControlNexus,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = [0; 10000];

    loop {
        let (amt, _) = try!(recvsocket.recv_from(&mut buf));

        match osc::Message::deserialize(&buf[..amt]) {
            Err(e) => println!("invalid osc messsage: {:?}", e),
            Ok(inmsg) => {
                match osc_to_ctrl_update(&inmsg, control_server) {
                    Ok(updmsg) => control_server.update(&updmsg),
                    Err(e) => {
                        if inmsg.path == "guiconfig" && inmsg.arguments.len() > 0 {
                            // is this a control config update instead?
                            match &inmsg.arguments[0] {
                                &osc::Argument::s(guistring) => {
                                    control_server.load_gui_string(guistring);
                                    println!("new control layout recieved!");
                                }
                                _ => println!("osc decode error: {:?}", e),
                            }
                        }
                    }
                };

                // print received afterwards, I guess for latency savings?
                println!("osc message received {} {:?}", inmsg.path, inmsg.arguments);
            }
        }
    }
}
