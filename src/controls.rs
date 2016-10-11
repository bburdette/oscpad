//use serde;
// use serde::de::Deserialize;
// #![feature(custom_derive, plugin)]
// #![plugin(serde_macros)]

extern crate serde_json;
extern crate serde;

use serde_json::Value;

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::error::Error;
use stringerror;

// --------------------------------------------------------
// root obj.  contains controls.
// --------------------------------------------------------

// root is not itself a Control!  although maybe it should be?
#[derive(Debug)]
pub struct Root {
  pub title: String,
  pub rootControl: Box<Control>,
}

pub fn deserializeRoot(data: &Value) -> Result<Box<Root>, Box<Error> >
{
  let obj = try_opt_resbox!(data.as_object(), "json value is not an object!");
  let title = 
    try_opt_resbox!(
      try_opt_resbox!(obj.get("title"), "'title' not found").as_string(), "title is not a string!");

  let rc = try_opt_resbox!(obj.get("rootControl"), "rootControl not found");

  let rootcontrol = try!(deserializeControl(Vec::new(), rc)); 

  Ok(Box::new(Root { title: String::from(title), rootControl: rootcontrol }))
}

// --------------------------------------------------------
// controls.
// --------------------------------------------------------

pub trait Control : Debug + Send {
  fn controlType(&self) -> &'static str; 
  fn controlId(&self) -> &Vec<i32>;
  fn cloneTrol(&self) -> Box<Control>;
  fn subControls(&self) -> Option<&Vec<Box<Control>>>; 
  fn update(&mut self, &UpdateMsg); 
  // build full update message of current state.
  fn toUpdate(&self) -> Option<UpdateMsg>;
  fn oscname(&self) -> &str;
}

#[derive(Debug)]
pub struct Slider {
  controlId: Vec<i32>,
  name: String,
  label: Option<String>,
  pressed: bool,
  location: f32,
}

impl Control for Slider {
  fn controlType(&self) -> &'static str { "slider" } 
  fn controlId(&self) -> &Vec<i32> { &self.controlId }
  fn cloneTrol(&self) -> Box<Control> { 
    Box::new( 
      Slider { controlId: self.controlId.clone(), 
               name: self.name.clone(), 
               label: self.label.clone(), 
               pressed: self.pressed.clone(), 
               location: self.location.clone() } ) }
  fn subControls(&self) -> Option<&Vec<Box<Control>>> { None } 
  fn update(&mut self, um: &UpdateMsg) {
    match um { 
      &UpdateMsg::Slider { controlId: _
                         , state: ref optState
                         , location: ref optLoc
                         , label: ref optLabel
                         } => {
        if let &Some(ref st) = optState {
          self.pressed = match st { &SliderState::Pressed => true
                                  , &SliderState::Unpressed => false };
          };
        if let &Some(ref loc) = optLoc { 
          self.location = *loc as f32;
        };
        if let &Some(ref t) = optLabel {
          self.label = Some(t.clone());
        };
        ()
        }
      _ => ()
      }
    }
  fn toUpdate(&self) -> Option<UpdateMsg> {
    let state = if self.pressed { SliderState::Pressed  } 
                else { SliderState::Unpressed };
    Some(UpdateMsg::Slider  { controlId: self.controlId.clone()
                            , state: Some(state)
                            , location: Some(self.location as f64) 
                            , label: self.label.clone()
                            })
  }
  fn oscname(&self) -> &str { &self.name[..] }
}

#[derive(Debug)]
pub struct Button { 
  controlId: Vec<i32>,
  name: String,
  label: Option<String>,
  pressed: bool,
}

impl Control for Button { 
  fn controlType(&self) -> &'static str { "button" } 
  fn controlId(&self) -> &Vec<i32> { &self.controlId }
  fn cloneTrol(&self) -> Box<Control> { 
    Box::new( 
      Button { controlId: self.controlId.clone(), 
               name: self.name.clone(), 
               label: self.label.clone(), 
               pressed: self.pressed.clone() } ) }
  fn subControls(&self) -> Option<&Vec<Box<Control>>> { None } 
  fn update(&mut self, um: &UpdateMsg) {
    match um { 
      &UpdateMsg::Button { controlId: _ 
                         , state: ref optState
                         , label: ref optLabel } => {
        if let &Some(ref st) = optState { 
          self.pressed = match st { &ButtonState::Pressed => true
                                  , &ButtonState::Unpressed => false };
          };
        if let &Some(ref t) = optLabel {
          self.label = Some(t.clone());
        };
        ()
        }
      _ => ()
      }
    }
  fn toUpdate(&self) -> Option<UpdateMsg> {
    let ut = if self.pressed { ButtonState::Pressed  } 
                        else { ButtonState::Unpressed };
    Some(UpdateMsg::Button { controlId: self.controlId.clone()
                           , state: Some(ut)
                           , label: self.label.clone() })
  }
  fn oscname(&self) -> &str { &self.name[..] }
}

#[derive(Debug)]
pub struct Label { 
  controlId: Vec<i32>,
  name: String,
  label: String,
}

impl Control for Label { 
  fn controlType(&self) -> &'static str { "Label" } 
  fn controlId(&self) -> &Vec<i32> { &self.controlId }
  fn cloneTrol(&self) -> Box<Control> { 
    Box::new( 
      Label { controlId: self.controlId.clone(), 
              name: self.name.clone(), 
              label: self.label.clone() } ) }
  fn subControls(&self) -> Option<&Vec<Box<Control>>> { None } 
  fn update(&mut self, um: &UpdateMsg) {
    match um { 
      &UpdateMsg::Label { controlId: _, label: ref l } => {
        self.label = l.clone();
        ()
        }
      _ => ()
      }
    }
  fn toUpdate(&self) -> Option<UpdateMsg> {
    Some(UpdateMsg::Label { controlId: self.controlId.clone(), label: self.label.clone() })
  }
  fn oscname(&self) -> &str { &self.name[..] }
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
  fn subControls(&self) -> Option<&Vec<Box<Control>>> { Some(&self.controls) } 
  fn update(&mut self, um: &UpdateMsg) {}
  fn toUpdate(&self) -> Option<UpdateMsg> { None }
  fn oscname(&self) -> &str { "" }
}

fn deserializeControl(aVId: Vec<i32>, data: &Value) -> Result<Box<Control>, Box<Error> >
{
  // what's the type?
  let obj = try_opt_resbox!(data.as_object(), "unable to parse control as json");
  let objtype = 
    try_opt_resbox!(try_opt_resbox!(obj.get("type"), "'type' not found").as_string(), "'type' is not a string");

  match objtype {
    "button" => { 
      let name = try_opt_resbox!(try_opt_resbox!(obj.get("name"), 
                                                 "'name' not found!").as_string(), 
                                 "'name' is not a string!");
      let label =  match obj.get("label") { 
          Some(x) => {
            let s = try_opt_resbox!(x.as_string(), "'label' is not a string!");
            Some(String::from(s))
            },
          None => None,
          };
      Ok(Box::new(Button { controlId: aVId.clone()
                         , name: String::from(name)
                         , label: label // String::from(label)
                         , pressed: false }))
    },
    "slider" => { 
      let name = try_opt_resbox!(try_opt_resbox!(obj.get("name"), 
                                                 "'name' not found!").as_string(), 
                                 "'name' is not a string!");
      let label =  match obj.get("label") { 
          Some(x) => {
            let s = try_opt_resbox!(x.as_string(), "'label' is not a string!");
            Some(String::from(s))
            },
          None => None,
          };
      Ok(Box::new(Slider { controlId: aVId.clone()
                         , name: String::from(name)
                         , label: label 
                         , pressed: false
                         , location: 0.5 }))
    },
    "label" => { 
      let name = try_opt_resbox!(try_opt_resbox!(obj.get("name"), "'name' not found!").as_string(), "'name' is not a string!");
      let label = try_opt_resbox!(try_opt_resbox!(obj.get("label"), "'label' not found!").as_string(), "'label' is not a string!");
      Ok(Box::new(Label { controlId: aVId.clone(), name: String::from(name), label: label.to_string() }))
    },
    "sizer" => { 
      let controls = 
        try_opt_resbox!(try_opt_resbox!(obj.get("controls"), "'controls' not found").as_array(), "'controls' is not an array");

      let mut controlv = Vec::new();

      // loop through array, makin controls.
      for (i, v) in controls.into_iter().enumerate() {
          let mut id = aVId.clone();
          id.push(i as i32); 
          let c = try!(deserializeControl(id, v));
          controlv.push(c);
          }
      
      Ok(Box::new(Sizer { controlId: aVId.clone(), controls: controlv }))
    },
    _ => Err(stringerror::stringBoxErr("objtype not supported!"))
  }
}

// --------------------------------------------------------
// control update messages.
// --------------------------------------------------------

/*

  JE.object [ ("controlType", JE.string "button")
            , ("controlId", SvgThings.encodeControlId um.controlId)
            , ("updateType", encodeUpdateType um.updateType)
            ]

  JE.object [ ("controlType", JE.string "slider")
            , ("controlId", SvgThings.encodeControlId um.controlId)
            , ("updateType", encodeUpdateType um.updateType)
            , ("location", (JE.float um.location))

*/

#[derive(Clone)]
pub enum ButtonState { 
  Pressed,
  Unpressed
  }

#[derive(Clone)]
pub enum SliderState { 
  Pressed,
  Unpressed
  }

#[derive(Clone)]
pub enum UpdateMsg { 
  Button  { controlId: Vec<i32>
          , state: Option<ButtonState>
          , label: Option<String>
          },
  Slider  { controlId: Vec<i32>
          , state: Option<SliderState>
          , location: Option<f64>
          , label: Option<String>
          },
  Label   { controlId: Vec<i32>
          , label: String 
          },
}

pub fn getUmId(um: &UpdateMsg) -> &Vec<i32> {
  match um { 
    &UpdateMsg::Button { controlId: ref cid, state: _, label: _ } => &cid,
    &UpdateMsg::Slider { controlId: ref cid, state: _, label: _, location: _ } => &cid, 
    &UpdateMsg::Label { controlId: ref cid, label: _ } => &cid, 
    }
}

fn convi32array(inp: &Vec<i32>) -> Vec<Value> {
  inp.into_iter().map(|x|{Value::I64(*x as i64)}).collect()
}

fn convarrayi32(inp: &Vec<Value>) -> Vec<i32> {
  inp.into_iter().map(|x|{x.as_i64().unwrap() as i32}).collect()
}

pub fn encodeUpdateMessage(um: &UpdateMsg) -> Value { 
  match um { 
    &UpdateMsg::Button { controlId: ref cid 
                       , state: ref optState
                       , label: ref optLabel } => {
      let mut btv = BTreeMap::new();
      btv.insert(String::from("controlType"), Value::String(String::from("button")));
      btv.insert(String::from("controlId"), Value::Array(convi32array(cid)));
      if let &Some(ref st) = optState { 
        btv.insert(String::from("state"), 
          Value::String(String::from( 
            (match st { &ButtonState::Pressed => "Press", 
                        &ButtonState::Unpressed => "Unpress", }))));
        };
      Value::Object(btv)
    }, 
    &UpdateMsg::Slider { controlId: ref cid
                       , state: ref optState
                       , label: ref lb
                       , location: ref loc } => 
    {
      let mut btv = BTreeMap::new();
      btv.insert(String::from("controlType"), 
                 Value::String(String::from("slider")));
      btv.insert(String::from("controlId"), 
                 Value::Array(convi32array(cid)));
      if let &Some(ref st) = optState { 
        btv.insert(String::from("state"), 
          Value::String(String::from( 
            (match st { &SliderState::Pressed => "Press",
                        &SliderState::Unpressed => "Unpress" }))));
      };
      if let &Some(loc) = loc { 
        btv.insert(String::from("location"), Value::F64(loc));
      };
      
      Value::Object(btv)
    },
    &UpdateMsg::Label { controlId: ref cid, label: ref labtext } => {
      let mut btv = BTreeMap::new();
      btv.insert(String::from("controlType"), Value::String(String::from("label")));
      btv.insert(String::from("controlId"), Value::Array(convi32array(cid)));
      btv.insert(String::from("label"), Value::String(labtext.clone()));
      Value::Object(btv)
    }, 
   } 
}
 
pub fn decodeUpdateMessage(data: &Value) -> Option<UpdateMsg> {
  let obj = try_opt!(data.as_object());
 
  let contype = try_opt!(try_opt!(obj.get("controlType")).as_string());
  println!("contype {}", contype);
  let conid = convarrayi32(try_opt!(try_opt!(obj.get("controlId")).as_array()));
  println!("conid {:?}", conid);
  // TODO: MAKE THESE OPTIONAL
  let ststring = try_opt!(try_opt!(obj.get("state")).as_string());
  println!("ststring {:?}", ststring);
 
  match contype {
    "slider" => {
      let location = try_opt!(try_opt!(obj.get("location")).as_f64());
      let st = try_opt!(match ststring 
        { "Press" => Some( SliderState::Pressed ) 
      //  , "Move" => Some( SliderState::Moved ) 
        , "Unpress" => Some( SliderState::Unpressed )
        , _ => None
        });
      let lab = obj.get("label").and_then(|s| s.as_string()).map(|s| String::from(s));
      Some( UpdateMsg::Slider { controlId: conid
                              , state: Some(st)
                              , location: Some(location)
                              , label: lab
                              } )
      },
    "button" => {
      let st = try_opt!(match ststring 
        { "Press" => Some( ButtonState::Pressed ) 
        , "Unpress" => Some( ButtonState::Unpressed ) 
        , _ => None
        });
      let lab = obj.get("label").and_then(|s| s.as_string()).map(|s| String::from(s));
        
      Some( UpdateMsg::Button { controlId: conid
                              , state: Some(st)
                              , label: lab } )
      },
    _ => None
    }
}

// --------------------------------------------------------
// control state map.  copies all the controls.
// --------------------------------------------------------

pub type controlMap = BTreeMap<Vec<i32>,Box<Control>>;

pub fn makeControlMap (control: &Control) -> controlMap {
  let mut btm = BTreeMap::new();

  makeControlMap_impl(control, btm)
}

fn makeControlMap_impl (control: &Control, mut map: controlMap) 
  -> controlMap 
{ 
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

pub type controlNameMap = BTreeMap<String, Vec<i32>>;

pub fn controlMapToNameMap(cmap: &controlMap) -> controlNameMap 
{
  let mut iter = cmap.iter();
  let mut cnm = BTreeMap::new();

  loop {
    match iter.next() {
      Some((key,val)) => { 
        let s = String::from(&*val.oscname()); 
        cnm.insert(s, key.clone());
        ()
      }, 
      None => break
    }
  }

  cnm
}

pub fn cmToUpdateArray(cm: &controlMap) -> Vec<UpdateMsg>
{
  let mut iter = cm.iter();
  let mut result = Vec::new();
  
  loop {
    match iter.next() {
      Some((key,val)) => { 
        match val.toUpdate() { 
          Some(updmsg) => result.push(updmsg),
          None => (),
        }
      },
      None => return result,
    }
  }
}
