//use serde;
// use serde::de::Deserialize;
// #![feature(custom_derive, plugin)]
// #![plugin(serde_macros)]

extern crate serde_json;
extern crate serde;


use serde_json::Value;

use std::collections;
use std::fmt::Debug;
// extern crate collections;

// use collections::vec;

// create from initial json spec file.
// update individual controls from state update msgs.
// create a json message containing the state of all ctrls.

pub trait Control : Debug {
  fn controlType(&self) -> &'static str; 
}


#[derive(Debug)]
pub struct Slider {
//  controlid: String,
  name: String,
  pressed: bool,
  location: f32,
}

impl Control for Slider {
  fn controlType(&self) -> &'static str { "slider" } 
}

#[derive(Debug)]
pub struct Button { 
//  controlid: String,
  name: String,
  pressed: bool,
}

impl Control for Button { 
  fn controlType(&self) -> &'static str { "button" } 
}

#[derive(Debug)]
pub struct Sizer { 
  // controlid: String,
  controls: Vec<Box<Control>>,
}

impl Control for Sizer { 
  fn controlType(&self) -> &'static str { "sizer" } 
}

// root is not a Control!
// #[derive(Debug)]
pub struct Root {
  pub title: String,
  pub rootControl: Box<Control>,
}

pub fn deserializeRoot(data: &Value) -> Option<Box<Root>>
{
  let obj = data.as_object().unwrap();
  let title = obj.get("title").unwrap().as_string().unwrap();

  let rc = obj.get("rootControl").unwrap();


  let rootcontrol = deserializeControl(rc).unwrap();

  Some(Box::new(Root { title: String::new() + title, rootControl: rootcontrol }))
}

fn deserializeControl(data: &Value) -> Option<Box<Control>>
{
  // what's the type?
  let obj = data.as_object().unwrap();
  let objtype = 
    obj.get("type").unwrap().as_string().unwrap();

  let mut x = 0;  

  // match objtype&[..] {
  match objtype {
    "slider" => { 
      let name = obj.get("name").unwrap().as_string().unwrap();
      Some(Box::new(Slider { name: String::new() + name, pressed: false, location: 0.5 }))
      },
    "button" => { 
      let name = obj.get("name").unwrap().as_string().unwrap();
      Some(Box::new(Button { name: String::new() + name, pressed: false }))
      },
    "sizer" => { 
      let name = obj.get("name").unwrap().as_string().unwrap();
      let controls = obj.get("controls").unwrap().as_array().unwrap();  

      let mut controlv = Vec::new();

      for (i, v) in controls.into_iter().enumerate() {
          let c = deserializeControl(v).unwrap();
          controlv.push(c);
      }
      // loop through controls, makin controls.
      Some(Box::new(Sizer { controls: controlv }))
      },
    _ => None,
    }
  
}


