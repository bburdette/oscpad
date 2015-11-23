module SvgControlPage where

import Effects exposing (Effects, Never)
import Html 
import SvgButton
import SvgSlider
import SvgControl
import SvgThings
import Task
import List exposing (..)
import Dict exposing (..)
import Json.Decode as JD exposing ((:=))
import Svg 
import Svg.Attributes as SA 
import Touch

-- json spec
type alias Spec = 
  { title: String
  , rootControl: SvgControl.Spec
  }

jsSpec : JD.Decoder Spec
jsSpec = JD.object2 Spec 
  ("title" := JD.string)
  ("rootControl" := SvgControl.jsSpec) 

type alias Model =
  { title: String  
  , mahrect: SvgThings.Rect 
  , srect: SvgThings.SRect 
  , spec: Spec
  , control: SvgControl.Model
  , prevtouches: Dict SvgThings.ControlId SvgControl.ControlTam
  , mahsend : (String -> Task.Task Never ())
  }

type alias ID = Int

-- UPDATE

type Action
    = JsonMsg String 
    | CAction SvgControl.Action 
    | WinDims (Int, Int)
    | Touche (List Touch.Touch)

type JsMessage 
  = JmSpec Spec
  | JmUpdate Action

jsMessage: JD.Decoder JsMessage
jsMessage = JD.oneOf
  [ jsSpec `JD.andThen` (\x -> JD.succeed (JmSpec x))
  , SvgControl.jsUpdateMessage `JD.andThen` 
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
      let wha = SvgControl.update act model.control 
          newmod = { model | control <- fst wha }
        in
          (newmod, Effects.map CAction (snd wha))
    WinDims (x,y) -> 
      init model.mahsend (SvgThings.Rect 0 0 x y) model.spec 
    Touche touchlist ->
      let tdict = touchDict model.control touchlist
          curtouches = Dict.map (\_ v -> fst v) tdict
          prevs = Dict.diff model.prevtouches curtouches in 
      ({model | prevtouches <- curtouches}, Effects.batch 
        (
          (List.map (\t -> Effects.task (Task.succeed (CAction t)))
              (List.filterMap (\(cid, (tam, tl)) -> 
                Maybe.map (SvgControl.toCtrlAction cid) (tam tl)) 
                (Dict.toList tdict)))
          ++
          (List.map (\t -> Effects.task (Task.succeed (CAction t)))
              (List.filterMap (\(cid, tam) -> 
                Maybe.map (SvgControl.toCtrlAction cid) (tam [])) 
                (Dict.toList prevs)))))
          

-- build a dict of controls -> touches.


touchDict: SvgControl.Model -> (List Touch.Touch) -> 
    Dict SvgThings.ControlId (SvgControl.ControlTam, (List Touch.Touch))
touchDict root touches = 
  let meh = List.filterMap (\t -> Maybe.andThen (SvgControl.findControl t.x t.y root) (\c -> Just (c,t))) touches in
  List.foldl updict Dict.empty meh 

updict: (SvgControl.Model, Touch.Touch) 
      -> Dict SvgThings.ControlId (SvgControl.ControlTam, (List Touch.Touch)) 
      -> Dict SvgThings.ControlId (SvgControl.ControlTam, (List Touch.Touch))
updict mt dict =
  Dict.update (SvgControl.controlId (fst mt)) 
              (\mbpair -> case mbpair of 
                Nothing -> Just (SvgControl.controlTouchActionMaker (fst mt), [(snd mt)])
                Just (a,b) -> Just (a, (snd mt) :: b))
              dict


{-

    Touche touchlist ->
      case head touchlist of 
        Nothing -> ({model | title <- "no touches" }, Effects.none)
        Just touch -> 
          case SvgControl.findControl touch.x touch.y model.control of
            Just control ->  
              ({model | title <- SvgControl.controlName control }, Effects.none)
            Nothing -> ({model | title = "no touches" }, Effects.none)


-}

-- Now update the controls from the dict?  



-- Ok could make a touch event list.  



init: (String -> Task.Task Never ()) -> SvgThings.Rect -> Spec 
  -> (Model, Effects Action)
init sendf rect spec = 
  let (conmod, conevt) = SvgControl.init sendf rect [] spec.rootControl
      fx = Effects.map CAction conevt
    in
     (Model spec.title rect (SvgThings.toSRect rect) spec conmod Dict.empty sendf, fx)
      

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

viewSvgControl : Signal.Address Action -> SvgControl.Model -> Svg.Svg 
viewSvgControl address conmodel =
  SvgControl.view (Signal.forwardTo address CAction) conmodel


