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

pub trait Control : Debug + Send {
  fn controlType(&self) -> &'static str; 
  fn controlId(&self) -> &Vec<i32>;
  fn cloneTrol(&self) -> Box<Control>;
  fn subControls(&self) -> Option<&Vec<Box<Control>>>; 
  fn update(&mut self, &UpdateMsg); 
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
  fn oscname(&self) -> &str { "" }
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
// control update messages.
// --------------------------------------------------------

/*

i  JE.object [ ("controlType", JE.string "button")
            , ("controlId", SvgThings.encodeControlId um.controlId)
            , ("updateType", encodeUpdateType um.updateType)
            ]

  JE.object [ ("controlType", JE.string "slider")
            , ("controlId", SvgThings.encodeControlId um.controlId)
            , ("updateType", encodeUpdateType um.updateType)
            , ("location", (JE.float um.location))

*/
macro_rules! try_opt { 
  ($e: expr) => { 
    match $e { 
      Some(x) => x, 
      None => return None 
      } 
  } 
}

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
          }
}

pub fn getUmId(um: &UpdateMsg) -> &Vec<i32> {
  match um { 
    &UpdateMsg::Button { controlId: ref cid, updateType: _ } => &cid,
    &UpdateMsg::Slider { controlId: ref cid, updateType: _, location: _ } => &cid, 
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
        let s = String::new() + &*val.oscname(); 
        cnm.insert(s, key.clone());
        ()
      }, 
      None => break
    }
  }

  cnm
}

