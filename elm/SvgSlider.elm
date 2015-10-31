module SvgSlider where

import Effects exposing (Effects, Never)
import Html exposing (Html)
-- import Html.Attributes exposing (style)
-- import Html.Events exposing (onClick)
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
    = SvgPress 
    | SvgUnpress 
    | UselessCrap 
    | Reply String 
    | ArbJson JE.Value

getX : JD.Decoder Int
getX = "offsetX" := JD.int 

getY : JD.Decoder Int
getY = "offsetY" := JD.int 

update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    SvgPress -> ({ model | pressed <- True}, Effects.task 
      ((model.sendf model.name) `Task.andThen` (\_ -> Task.succeed UselessCrap)))
    SvgUnpress -> ({ model | pressed <- False}, Effects.none)
    UselessCrap -> (model, Effects.none)
    Reply s -> ({model | name <- s}, Effects.none)
    ArbJson v -> 
      case model.orientation of 
        SvgThings.Horizontal ->
          case (JD.decodeValue getX v) of 
            Ok i ->  
              ({model | location <- (toFloat (i - model.rect.x)) / toFloat model.rect.w }, Effects.none)
            Err e -> 
              ({model | name <- (JE.encode 2 v)}, Effects.none)
        SvgThings.Vertical -> 
          case (JD.decodeValue getY v) of 
            Ok i ->  
              ({model | location <- (toFloat (i - model.rect.y)) / toFloat model.rect.h }, Effects.none)
            Err e -> 
              ({model | name <- (JE.encode 2 v)}, Effects.none)

-- VIEW

(=>) = (,)

-- try VD.onWithOptions for preventing scrolling on touchscreens and 
-- etc. See virtualdom docs.

onClick : Signal.Address Action -> VD.Property
onClick address =
    VD.onWithOptions "click" (VD.Options True True) JD.value (\v -> Signal.message address (ArbJson v))

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
  g [ onMouseDown (Signal.message address SvgPress)
    , onMouseMove (Signal.message address SvgPress)
    , onMouseUp (Signal.message address SvgUnpress)
    , onMouseOut (Signal.message address SvgUnpress)
    , onClick address
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


