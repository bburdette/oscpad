
#[macro_use]
mod tryopt;
mod stringerror;

use std::net::UdpSocket;
use std::string::String;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;

extern crate tinyosc;
use tinyosc as osc;


fn main() {

  match resmain() {
    Ok(s) => println!("ok: {}", s),
    Err(e) => println!("error: {} ", e),
    }
}

fn resmain() -> Result<String, Box<dyn std::error::Error> > {
  let args = env::args();
  let mut iter = args.skip(1); // skip the program name
  
  let syntax = "syntax: \n guisend <ip:port> <gui filename>";

  let sendip = try_opt_resbox!(iter.next(), syntax);
  let guifilename = try_opt_resbox!(iter.next(), syntax);
  let guistring = try!(load_string(&guifilename));

  let socket = try!(UdpSocket::bind("0.0.0.0:0"));
  println!("guisend");

  let mut arghs = Vec::new();
  arghs.push(osc::Argument::s(&guistring));

  let outmsg = osc::Message { path: "guiconfig", arguments: arghs };
  let v = try!(outmsg.serialize());

  println!("sending {} {:?}", outmsg.path, outmsg.arguments );
  try!(socket.send_to(&v, &sendip[..]));

  Ok("success".to_string())
}


fn load_string(file_name: &str) -> Result<String, Box<dyn std::error::Error> >
{
  let path = &Path::new(&file_name);
  let mut inf = try!(File::open(path));
  let mut result = String::new();
  let len = try!(inf.read_to_string(&mut result));
  println!("read {} bytes", len);
  Ok(result)
}

