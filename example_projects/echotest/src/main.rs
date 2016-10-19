
#[macro_use]
mod tryopt;
mod stringerror;

use std::net::UdpSocket;
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
    let (amt, _) = try!(recvsocket.recv_from(&mut buf));

    println!("length: {}", amt);
    let inmsg = match osc::Message::deserialize(&buf[.. amt]) {
       Ok(m) => m,
       Err(e) => return Err(stringerror::string_box_err(
                 &format(format_args!("OSC deserialize error {:?}", e)))),
      };

    println!("message recieved {} {:?}", inmsg.path, inmsg.arguments );

    match inmsg {
      osc::Message { path: ref inpath, arguments: ref args } => {
       
        if &inpath[0..2] == "hs"
          {
            // look for a "location" update, ie
            // an arg "location" followed by a float.
            let mut arg_iter = args.into_iter();
            let mut arg = arg_iter.next();

            while arg.is_some()
            {
              match arg
              {
              Some(&osc::Argument::s("location")) => {
                arg = arg_iter.next();
                match arg {
                  Some(&osc::Argument::f(loc)) => {
                    let outpath = format(format_args!("hs{}", &inpath[2..]));    
                    // let outpath = format(format_args!("lb{}", &inpath[2..]));    
                    let labtext = format(format_args!("{}", loc));
                    let mut arghs = Vec::new();
                    // arghs.push(osc::Argument::f(b * 100.0 - 100.0)); 
                    arghs.push(osc::Argument::s("label")); 
                    arghs.push(osc::Argument::s(&labtext)); 
                    let outmsg = osc::Message { path: &outpath, arguments: arghs };
                    match outmsg.serialize() {
                      Ok(v) => {
                        println!("sending {} {:?}", outmsg.path, outmsg.arguments );
                        match sendsocket.send_to(&v, &sendip[..]) {
                          Ok(_) => (),
                          Err(e) => println!("error sending osc message: {:?}", e),
                          }
                      },
                      Err(e) => { println!("error: {:?}", e) },
                    }
                    break;
                    },
                  _ => {
                    continue;
                  }
                }
              }
              _ => {
                arg = arg_iter.next();
                  continue;
              }
            }
          }
          Ok(0)
          }
        else if &inpath[0..1] == "b"
          {
            // for any kind of button message, update the label.
            // amounts to update the label to the last pressed (or released) button. 
            match &args[0] {
              &osc::Argument::s(_) => {
                  let outpath = "lb3"; 
                  let mut arghs = Vec::new();
                  arghs.push(osc::Argument::s("label")); 
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
                println!("ignore args: {:?}", args);
                // return Err(Error::new(ErrorKind::Other, "unexpected osc args!"));
                Ok(0)
              },
            }
          }
        else
          {
             println!("ignore args 2: {:?}", args);
             // println!("ignore");
             Ok(0)    
          }
        },
      };
  };


  // drop(socket); // close the socket
  // Ok(String::from("meh"))
}

