module SvgSlider where

import Effects exposing (Effects, Never)
import Html exposing (Html)
import Http
import Json.Decode as JD exposing ((:=))
import Json.Encode as JE 
import Task
import Svg exposing (Svg, svg, rect, g, text, text', Attribute)
import Svg.Attributes exposing (..)
import NoDragEvents exposing (onClick, onMouseUp, onMouseMove, onMouseDown, onMouseOut)
import SvgThings
import VirtualDom as VD

type alias Spec = 
  { name: String
  , orientation: SvgThings.Orientation
  }

jsSpec : JD.Decoder Spec
jsSpec = JD.object2 Spec 
  ("name" := JD.string)
  (("orientation" := JD.string) `JD.andThen` SvgThings.jsOrientation)

-- MODEL

type alias Model =
  { name : String
  , rect: SvgThings.Rect
  , srect: SvgThings.SRect
  , orientation: SvgThings.Orientation
  , pressed: Bool
  , location: Float
  , sendf : (String -> Task.Task Never ())
  }

init : (String -> Task.Task Never ()) -> Spec -> SvgThings.Rect 
  -> (Model, Effects Action)
init sendf spec rect =
  ( Model (spec.name) 
          rect
          (SvgThings.SRect (toString (rect.x + 5)) 
                           (toString (rect.y + 5))
                           (toString (rect.w - 5))
                           (toString (rect.h - 5)))
          spec.orientation
          False 0.5 sendf
  , Effects.none
  )

buttColor: Bool -> String
buttColor pressed = 
  case pressed of 
    True -> "#f000f0"
    False -> "#60B5CC"

-- UPDATE

type Action
    = SvgPress JE.Value
    | SvgUnpress JE.Value 
    | UselessCrap 
    | Reply String 
    | SvgMoved JE.Value

getX : JD.Decoder Int
getX = "offsetX" := JD.int 

getY : JD.Decoder Int
getY = "offsetY" := JD.int 

type UpdateType 
  = Press
  | Move
  | Unpress

type alias UpdateMessage = 
  { updateType: UpdateType
  , location: Float
  }

encodeUpdateMessage: UpdateMessage -> JD.Value
encodeUpdateMessage um = 
  JE.object [ ("updateType", encodeUpdateType um.updateType) 
            , ("location", (JE.float um.location))
            ]
  
encodeUpdateType: UpdateType -> JD.Value
encodeUpdateType ut = 
  case ut of 
    Press -> JE.string "Press"
    Move -> JE.string "Move"
    Unpress -> JE.string "Unpress"

-- get mouse/whatever location from the json message, 
-- compute slider location from that.
getLocation: Model -> JD.Value -> Result String Float
getLocation model v = 
  case model.orientation of 
    SvgThings.Horizontal ->
      case (JD.decodeValue getX v) of 
        Ok i -> Ok ((toFloat (i - model.rect.x)) 
                    / toFloat model.rect.w)
        Err e -> Err e
    SvgThings.Vertical -> 
      case (JD.decodeValue getY v) of 
        Ok i -> Ok ((toFloat (i - model.rect.y)) 
                    / toFloat model.rect.h)
        Err e -> Err e

{-
updLoc: Model -> JD.Value -> (Model, Effects Action) 

      case (getLocation model v) of 
        Ok l -> ({model | location <- l}, Effects.none)
        _ -> (model, Effects.none)
-}


update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    SvgPress v -> 
      case (getLocation model v) of 
        Ok l -> 
          let um = JE.encode 0 (encodeUpdateMessage (UpdateMessage Press l)) in
            ( {model | location <- l, pressed <- True}
            , Effects.task 
                ((model.sendf um) `Task.andThen` 
                (\_ -> Task.succeed UselessCrap)))
        _ -> (model, Effects.none)
    SvgUnpress v -> 
      let um = JE.encode 0 (encodeUpdateMessage 
                (UpdateMessage Unpress model.location)) in
        ( { model | pressed <- False }
        , Effects.task 
            ((model.sendf um) `Task.andThen` 
            (\_ -> Task.succeed UselessCrap)))
    UselessCrap -> (model, Effects.none)
    Reply s -> ({model | name <- s}, Effects.none)
    SvgMoved v ->
      case model.pressed of 
        True -> 
          case (getLocation model v) of 
            Ok l -> 
              let um = JE.encode 0 (encodeUpdateMessage (UpdateMessage Move l)) in
                ( {model | location <- l}
                , Effects.task 
                    ((model.sendf um) `Task.andThen` 
                    (\_ -> Task.succeed UselessCrap)))
            _ -> (model, Effects.none)
        False -> (model, Effects.none)


-- VIEW

(=>) = (,)

-- try VD.onWithOptions for preventing scrolling on touchscreens and 
-- etc. See virtualdom docs.


sliderEvt: String -> (JD.Value -> Action) -> Signal.Address Action -> VD.Property
sliderEvt evtname mkaction address =
    VD.onWithOptions evtname (VD.Options True True) JD.value (\v -> Signal.message address (mkaction v))

-- onClick = sliderEvt "click" SvgMoved
--  , onClick address
onMouseMove = sliderEvt "mousemove" SvgMoved
onMouseLeave = sliderEvt "mouseleave" SvgUnpress
onMouseDown = sliderEvt "mousedown" SvgPress
onMouseUp = sliderEvt "mouseup" SvgUnpress

view : Signal.Address Action -> Model -> Svg
view address model =
  let (sx, sy, sw, sh) = case model.orientation of 
     SvgThings.Vertical -> 
        (model.srect.x
        ,toString ((round (model.location * toFloat (model.rect.h))) + model.rect.y)
        ,model.srect.w
        ,"3")
     SvgThings.Horizontal -> 
        (toString ((round (model.location * toFloat (model.rect.w))) + model.rect.x)
        ,model.srect.y
        ,"3"
        ,model.srect.h)
   in
  g [ onMouseDown address 
    , onMouseUp address 
    , onMouseLeave address 
    , onMouseMove address 
    ]
    [ rect
        [ x model.srect.x
        , y model.srect.y 
        , width model.srect.w
        , height model.srect.h
        , rx "2"
        , ry "2"
        , style "fill: #F1F1F1;"
        ]
        []
    , rect
        [ x sx 
        , y sy 
        , width sw
        , height sh 
        , rx "2"
        , ry "2"
        , style ("fill: " ++ buttColor(model.pressed) ++ ";")
        ]
        []
    , text' [ fill "white", textAnchor "middle" ] [ text model.name ]
    ]


