//use serde;
// use serde::de::Deserialize;
// #![feature(custom_derive, plugin)]
// #![plugin(serde_macros)]

extern crate serde_json;
extern crate serde;


use serde_json::Value;

use std::collections;
use std::collections::BTreeMap;
use std::fmt::Debug;

// --------------------------------------------------------
// root obj.  contains controls.
// --------------------------------------------------------

// root is not itself a Control!  although maybe it should be?
#[derive(Debug)]
pub struct Root {
  pub title: String,
  pub rootControl: Box<Control>,
}

pub fn deserializeRoot(data: &Value) -> Option<Box<Root>>
{
  let obj = data.as_object().unwrap();
  let title = obj.get("title").unwrap().as_string().unwrap();

  let rc = obj.get("rootControl").unwrap();

  let rootcontrol = deserializeControl(Vec::new(), rc).unwrap();

  Some(Box::new(Root { title: String::new() + title, rootControl: rootcontrol }))
}

// --------------------------------------------------------
// controls.
// --------------------------------------------------------

pub trait Control : Debug {
  fn controlType(&self) -> &'static str; 
  fn controlId(&self) -> &Vec<i32>;
  fn cloneTrol(&self) -> Box<Control>;
  fn subControls(&self) -> Option<&Vec<Box<Control>>>; 
}

#[derive(Debug)]
pub struct Slider {
  controlId: Vec<i32>,
  name: String,
  pressed: bool,
  location: f32,
}

impl Control for Slider {
  fn controlType(&self) -> &'static str { "slider" } 
  fn controlId(&self) -> &Vec<i32> { &self.controlId }
  // fn cloneTrol(&self) -> Box<Control> { Box::new( Slider { *self } ) }
  fn cloneTrol(&self) -> Box<Control> { 
    Box::new( 
      Slider { controlId: self.controlId.clone(), 
              name: self.name.clone(), 
              pressed: self.pressed.clone(), 
              location: self.location.clone() } ) }
  fn subControls(&self) -> Option<&Vec<Box<Control>>> { None } 
}

#[derive(Debug)]
pub struct Button { 
  controlId: Vec<i32>,
  name: String,
  pressed: bool,
}

impl Control for Button { 
  fn controlType(&self) -> &'static str { "button" } 
  fn controlId(&self) -> &Vec<i32> { &self.controlId }
  fn cloneTrol(&self) -> Box<Control> { 
    Box::new( 
      Button { controlId: self.controlId.clone(), 
              name: self.name.clone(), 
              pressed: self.pressed.clone() } ) }
  fn subControls(&self) -> Option<&Vec<Box<Control>>> { None } 
}

//#[derive(Debug, Clone)]
#[derive(Debug)]
pub struct Sizer { 
  controlId: Vec<i32>,
  controls: Vec<Box<Control>>,
}

impl Control for Sizer { 
  fn controlType(&self) -> &'static str { "sizer" } 
  fn controlId(&self) -> &Vec<i32> { &self.controlId }
  fn cloneTrol(&self) -> Box<Control> { 
    Box::new( 
      Sizer { controlId: self.controlId.clone(), 
              controls: Vec::new() } ) } 
  fn subControls(&self) -> Option<&Vec<Box<Control>>> { None } 
}

fn deserializeControl(aVId: Vec<i32>, data: &Value) -> Option<Box<Control>>
{
  // what's the type?
  let obj = data.as_object().unwrap();
  let objtype = 
    obj.get("type").unwrap().as_string().unwrap();

  match objtype {
    "slider" => { 
      let name = obj.get("name").unwrap().as_string().unwrap();
      Some(Box::new(Slider { controlId: aVId.clone(), name: String::new() + name, pressed: false, location: 0.5 }))
      },
    "button" => { 
      let name = obj.get("name").unwrap().as_string().unwrap();
      Some(Box::new(Button { controlId: aVId.clone(), name: String::new() + name, pressed: false }))
      },
    "sizer" => { 
      let name = obj.get("name").unwrap().as_string().unwrap();
      let controls = obj.get("controls").unwrap().as_array().unwrap();  

      let mut controlv = Vec::new();

      for (i, v) in controls.into_iter().enumerate() {
          let mut id = aVId.clone();
          id.push(i as i32); 
          let c = deserializeControl(id, v).unwrap();
          controlv.push(c);
          }
      // loop through controls, makin controls.
      Some(Box::new(Sizer { controlId: aVId.clone(), controls: controlv }))
      },
    _ => None,
    }
}

// --------------------------------------------------------
// control state map.  copies all the controls.
// --------------------------------------------------------

pub fn makeControlMap (control: &Control) -> BTreeMap<Vec<i32>,Box<Control>> {
  //.let mut btm = BTreeMap<Vec<i32>,Box<Control>>::new();
  let mut btm = BTreeMap::new();

  makeControlMap_impl(control, btm)
}

fn makeControlMap_impl (control: &Control, mut map: BTreeMap<Vec<i32>,Box<Control>>) -> BTreeMap<Vec<i32>,Box<Control>> {

  map.insert(control.controlId().clone(), control.cloneTrol());

  match control.subControls() {
    Some(trols) => {
      let mut item = trols.into_iter();

      loop {
          match item.next() {
            Some(x) => {
              map = makeControlMap_impl(&**x, map)
              },
            None => { break },
          }
        }
      }
    None => {} 
    }

  map
}
