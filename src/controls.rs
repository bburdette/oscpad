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
  fn toUpdate(&self) -> Option<UpdateMsg>;
  fn oscname(&self) -> &str;
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
  fn cloneTrol(&self) -> Box<Control> { 
    Box::new( 
      Slider { controlId: self.controlId.clone(), 
               name: self.name.clone(), 
               pressed: self.pressed.clone(), 
               location: self.location.clone() } ) }
  fn subControls(&self) -> Option<&Vec<Box<Control>>> { None } 
  fn update(&mut self, um: &UpdateMsg) {
    match um { 
      &UpdateMsg::Slider { controlId: _, updateType: ref ut, location: l} => {
        self.pressed = match ut { &SliderUpType::Moved => true, &SliderUpType::Pressed => true, &SliderUpType::Unpressed => false };
        self.location = l as f32;
        ()
        }
      _ => ()
      }
    }
  fn toUpdate(&self) -> Option<UpdateMsg> {
    let ut = if self.pressed { SliderUpType::Pressed  } else { SliderUpType::Moved };
    Some(UpdateMsg::Slider { controlId: self.controlId.clone(), updateType: ut, location: self.location as f64 })
  }
  fn oscname(&self) -> &str { &self.name[..] }
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
  fn update(&mut self, um: &UpdateMsg) {
    match um { 
      &UpdateMsg::Button { controlId: _, updateType: ref ut } => {
        self.pressed = match ut { &ButtonUpType::Pressed => true, &ButtonUpType::Unpressed => false };
        ()
        }
      _ => ()
      }
    }
  fn toUpdate(&self) -> Option<UpdateMsg> {
    let ut = if self.pressed { ButtonUpType::Pressed  } else { ButtonUpType::Unpressed };
    Some(UpdateMsg::Button { controlId: self.controlId.clone(), updateType: ut })
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
      let name = try_opt_resbox!(try_opt_resbox!(obj.get("name"), "'name' not found!").as_string(), "'name' is not a string!");
      Ok(Box::new(Button { controlId: aVId.clone(), name: String::from(name), pressed: false }))
    },
    "slider" => { 
      let name = try_opt_resbox!(try_opt_resbox!(obj.get("name"), "'name' not found!").as_string(), "'name' is not a string!");
      Ok(Box::new(Slider { controlId: aVId.clone(), name: String::from(name), pressed: false, location: 0.5 }))
    },
    "label" => { 
      let name = try_opt_resbox!(try_opt_resbox!(obj.get("name"), "'name' not found!").as_string(), "'name' is not a string!");
      let label = try_opt_resbox!(try_opt_resbox!(obj.get("label"), "'label' not found!").as_string(), "'label' is not a string!");
      Ok(Box::new(Label { controlId: aVId.clone(), name: String::from(name), label: label.to_string() }))
    },
    "sizer" => { 
      let name = try_opt_resbox!(try_opt_resbox!(obj.get("name"), "name not found!").as_string(), "name is not a string!");
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

pub enum ButtonUpType { 
  Pressed,
  Unpressed
  }

pub enum SliderUpType { 
  Pressed,
  Moved,
  Unpressed
  }

pub enum UpdateMsg { 
  Button  { controlId: Vec<i32>
          , updateType: ButtonUpType
          },
  Slider  { controlId: Vec<i32>
          , updateType: SliderUpType 
          , location: f64
          },
  Label   { controlId: Vec<i32>
          , label: String 
          },
}

pub fn getUmId(um: &UpdateMsg) -> &Vec<i32> {
  match um { 
    &UpdateMsg::Button { controlId: ref cid, updateType: _ } => &cid,
    &UpdateMsg::Slider { controlId: ref cid, updateType: _, location: _ } => &cid, 
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
    &UpdateMsg::Button { controlId: ref cid, updateType: ref ut } => {
      let mut btv = BTreeMap::new();
      btv.insert(String::from("controlType"), Value::String(String::from("button")));
      btv.insert(String::from("controlId"), Value::Array(convi32array(cid)));
      btv.insert(String::from("updateType"), 
        Value::String(String::from( 
          (match ut { &ButtonUpType::Pressed => "Press", 
                      &ButtonUpType::Unpressed => "Unpress" }))));
      Value::Object(btv)
    }, 
    &UpdateMsg::Slider { controlId: ref cid, updateType: ref ut, location: ref loc } => {
      let mut btv = BTreeMap::new();
      btv.insert(String::from("controlType"), Value::String(String::from("slider")));
      btv.insert(String::from("controlId"), Value::Array(convi32array(cid)));
      btv.insert(String::from("updateType"), 
        Value::String(String::from( 
          (match ut { &SliderUpType::Pressed => "Press",
                      &SliderUpType::Moved => "Move", 
                      &SliderUpType::Unpressed => "Unpress" }))));
      btv.insert(String::from("location"), Value::F64(*loc));
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
  let conid = convarrayi32(try_opt!(try_opt!(obj.get("controlId")).as_array()));
  let utstring = try_opt!(try_opt!(obj.get("updateType")).as_string());
 
  match contype {
    "slider" => {
      let location = try_opt!(try_opt!(obj.get("location")).as_f64());
      let ut = try_opt!(match utstring 
        { "Press" => Some( SliderUpType::Pressed ) 
        , "Move" => Some( SliderUpType::Moved ) 
        , "Unpress" => Some( SliderUpType::Unpressed )
        , _ => None
        });
      Some( UpdateMsg::Slider { controlId: conid, updateType: ut, location: location } )
      },
    "button" => {
      let ut = try_opt!(match utstring 
        { "Press" => Some( ButtonUpType::Pressed ) 
        , "Unpress" => Some( ButtonUpType::Unpressed ) 
        , _ => None
        });
        
      Some( UpdateMsg::Button { controlId: conid, updateType: ut } )
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
