
#[macro_use]
mod tryopt;
mod stringerror;

use std::net::UdpSocket;
use std::io::{Error,ErrorKind};
use std::string::String;
use std::env;

extern crate tinyosc;
use tinyosc as osc;


use std::fmt::format;

fn main() {

  match rmain() {
    Ok(s) => println!("ok: {}", s),
    Err(e) => println!("error: {} ", e),
    }
}

fn rmain() -> Result<String, Box<std::error::Error> > { 
  let args = env::args();
  let mut iter = args.skip(1); // skip the program name
  
  let syntax = "syntax: \n echotest <recvip:port> <sendip:port>";

  let recvip = try_opt_resbox!(iter.next(), syntax);
  let sendip = try_opt_resbox!(iter.next(), syntax);

  let recvsocket = try!(UdpSocket::bind(&recvip[..]));
  // for sending, bind to this.  if we bind to localhost, we can't
  // send messages to other machines.  for that reason, don't reuse the 
  // recvsocket for sending. 
  let sendsocket = try!(UdpSocket::bind("0.0.0.0:0"));
  let mut buf = [0; 100];
  println!("echotest");

  loop { 
    let (amt, src) = try!(recvsocket.recv_from(&mut buf));

    println!("length: {}", amt);
    let inmsg = match osc::Message::deserialize(&buf[.. amt]) {
       Ok(m) => m,
       Err(e) => return Err(stringerror::stringBoxErr("OSC deserialize error")),
      };

    println!("message recieved {} {:?}", inmsg.path, inmsg.arguments );

    match inmsg {
      osc::Message { path: ref inpath, arguments: ref args } => {
       
        println!("inpath: {} eq: {}", &inpath[0..2], &inpath[0..2] == "hs"); 
 
        if args.len() == 2 && &inpath[0..2] == "hs"
          {
            let q = &args[0];
            let r = &args[1];
       
            match (q,r) {
              (&osc::Argument::s(_), &osc::Argument::f(b)) => {
                  let outpath = format(format_args!("hs{}", &inpath[2..]));    
                  let labtext = format(format_args!("{}", b));
                  let mut arghs = Vec::new();
                  // arghs.push(osc::Argument::f(b * 100.0 - 100.0)); 
                  arghs.push(osc::Argument::s("l_label")); 
                  arghs.push(osc::Argument::s(&labtext)); 
                  let outmsg = osc::Message { path: &outpath, arguments: arghs };
                  match outmsg.serialize() {
                    Ok(v) => {
                      println!("sending {} {:?}", outmsg.path, outmsg.arguments );
              			  sendsocket.send_to(&v, &sendip[..])
                    },
                    Err(e) => return Err(Box::new(e)),
                  }
                },
              _ => { 
                println!("ignore");
                // return Err(Error::new(ErrorKind::Other, "unexpected osc args!"));
                Ok(0)
              },
            }
          }
        else if args.len() == 1 && &inpath[0..1] == "b"
          {
            match &args[0] {
              &osc::Argument::s(evt) => {
                  let outpath = "lb3"; 
                  let mut arghs = Vec::new();
                  arghs.push(osc::Argument::s("l_label")); 
                  arghs.push(osc::Argument::s(&inpath)); 
                  let outmsg = osc::Message { path: &outpath, arguments: arghs };
                  match outmsg.serialize() {
                    Ok(v) => {
                      println!("sending {} {:?}", outmsg.path, outmsg.arguments );
              			  sendsocket.send_to(&v, &sendip[..])
                    },
                    Err(e) => return Err(Box::new(e)),
                  }
                },
              _ => { 
                println!("ignore");
                // return Err(Error::new(ErrorKind::Other, "unexpected osc args!"));
                Ok(0)
              },
            }
          }
        else
          {
             println!("ignore");
             Ok(0)    
          }
        },
      };
  };


  // drop(socket); // close the socket
  // Ok(String::from("meh"))
}

