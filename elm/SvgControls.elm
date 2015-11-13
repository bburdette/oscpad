module SvgControls where

import Effects exposing (Effects, Never)
import Html 
import SvgButton
import SvgSlider
import Task
import List exposing (..)
import Dict exposing (..)
import Json.Decode as JD exposing ((:=))
import Svg 
import Svg.Attributes as SA 
import SvgThings
import Controls

-- json spec
type alias Spec = 
  { title: String
  , rootControl: Controls.Spec
  }

jsSpec : JD.Decoder Spec
jsSpec = JD.object2 Spec 
  ("title" := JD.string)
  ("rootControl" := Controls.jsSpec) 

type alias Model =
  { title: String  
  , mahrect: SvgThings.Rect 
  , srect: SvgThings.SRect 
  , spec: Spec
  , control: Controls.Model
  , mahsend : (String -> Task.Task Never ())
  }

type alias ID = Int

-- UPDATE

type Action
    = JsonMsg String 
    | CAction Controls.Action 
    | WinDims (Int, Int)

type JsMessage 
  = JmSpec Spec
  | JmUpdate Action

jsMessage: JD.Decoder JsMessage
jsMessage = JD.oneOf
  [ jsSpec `JD.andThen` (\x -> JD.succeed (JmSpec x))
  , Controls.jsUpdateMessage `JD.andThen` 
      (\x -> JD.succeed (JmUpdate (CAction x)))
  ] 


update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    JsonMsg s -> 
      case (JD.decodeString jsMessage s) of 
        Ok (JmSpec spec) -> 
          init model.mahsend model.mahrect spec 
        Ok (JmUpdate jmact) -> 
          update jmact model
        Err e -> ({model | title <- e}, Effects.none)
    CAction act -> 
      let wha = Controls.update act model.control 
          newmod = { model | control <- fst wha }
        in
          (newmod, Effects.map CAction (snd wha))
    WinDims (x,y) -> 
      init model.mahsend (SvgThings.Rect 0 0 x y) model.spec 

init: (String -> Task.Task Never ()) -> SvgThings.Rect -> Spec 
  -> (Model, Effects Action)
init sendf rect spec = 
  let (conmod, conevt) = Controls.init sendf rect [] spec.rootControl
      fx = Effects.map CAction conevt
    in
     (Model spec.title rect (SvgThings.toSRect rect) spec conmod sendf, fx)
      

-- VIEW

(=>) = (,)

view : Signal.Address Action -> Model -> Html.Html
view address model =
  Html.div [] (
    [Html.text "meh", 
     Html.br [] [],
     Html.text model.title, 
     Html.br [] []] 
    ++ 
    [Svg.svg
      [ SA.width model.srect.w
      , SA.height model.srect.h
      , SA.viewBox (model.srect.x ++ " " 
                 ++ model.srect.y ++ " " 
                 ++ model.srect.w ++ " "
                 ++ model.srect.h)
      ]
      [(viewSvgControl address model.control)]
    ])

viewSvgControl : Signal.Address Action -> Controls.Model -> Svg.Svg 
viewSvgControl address conmodel =
  Controls.view (Signal.forwardTo address CAction) conmodel


