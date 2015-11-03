//use serde;
// use serde::de::Deserialize;

extern crate serde_json;
extern crate serde;

// create from initial json spec file.
// update individual controls from state update msgs.
// create a json message containing the state of all ctrls.

/*
trait Control : Deserialize {
  fn controlType(&self) -> String;
  // tojson
  // fromjson
  // updatefromjson
}

*/

#[derive(Deserialize)]
pub struct Slider {
  controlid: String,
  name: String,
  pressed: bool,
}

/*
impl Control for Slider {
  // tojson
  // fromjson
  // updatefromjson
}
*/

#[derive(Deserialize)]
pub struct Button { 
  controlid: String,
  name: String,
  pressed: bool,
}

/*
#[derive(Deserialize)]
pub struct Sizer { 
  controlid: String,
  controls: Vec<Control>,
}

#[derive(Deserialize)]
pub struct Root {
  title: String,
  rootControl: Control,
}
*/
