module Controls where 

import SvgButton
import SvgSlider
import Json.Decode as JD exposing ((:=))
import Effects exposing (Effects, Never)
import Task
import Svg exposing (Svg)
import SvgThings
import Signal

type Spec = CsButton SvgButton.Spec 
          | CsSlider SvgSlider.Spec

type Model = CmButton SvgButton.Model
           | CmSlider SvgSlider.Model

type Action = CaButton SvgButton.Action
            | CaSlider SvgSlider.Action

jsSpec : JD.Decoder Spec 
jsSpec = 
  ("type" := JD.string) `JD.andThen` jsCs

jsCs : String -> JD.Decoder Spec
jsCs t = 
  case t of 
    "button" -> SvgButton.jsSpec `JD.andThen` (\a -> JD.succeed (CsButton a))
    "slider" -> SvgSlider.jsSpec `JD.andThen` (\a -> JD.succeed (CsSlider a))

update : Action -> Model -> (Model, Effects Action)
update action model =
  case (action,model) of 
    (CaButton act, CmButton m) -> 
      let (a,b) = (SvgButton.update act m) in
        (CmButton a, Effects.map CaButton b)
    (CaSlider act, CmSlider m) -> 
      let (a,b) = (SvgSlider.update act m) in
        (CmSlider a, Effects.map CaSlider b)

init : (String -> Task.Task Never ()) -> Spec -> SvgThings.Rect 
  -> (Model, Effects Action)
init sendf spec rect =
  case spec of 
    CsButton s -> 
      let (a,b) = (SvgButton.init sendf s rect) in
        (CmButton a, Effects.map CaButton b)
    CsSlider s -> 
      let (a,b) = (SvgSlider.init sendf s rect) in
        (CmSlider a, Effects.map CaSlider b)


view : Signal.Address Action -> Model -> Svg
view address model = 
  case model of 
    CmButton m -> SvgButton.view (Signal.forwardTo address CaButton)  m
    CmSlider m -> SvgSlider.view (Signal.forwardTo address CaSlider)  m


