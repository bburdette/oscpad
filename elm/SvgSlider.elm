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
import Svg.Events exposing (onClick, onMouseUp, onMouseMove, onMouseDown, onMouseOut)
import SvgThings
import VirtualDom as VD

-- how to specify a button in json.
type alias Spec = 
  { name: String
  }

jsSpec : JD.Decoder Spec
jsSpec = JD.object1 Spec ("name" := JD.string)

-- MODEL

type alias Model =
  { name : String
  , rect: SvgThings.Rect
  , srect: SvgThings.SRect
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
    = SvgPress | SvgUnpress | UselessCrap | Reply String | ArbJson JE.Value

update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    SvgPress -> ({ model | pressed <- True}, Effects.task 
      ((model.sendf model.name) `Task.andThen` (\_ -> Task.succeed UselessCrap)))
    SvgUnpress -> ({ model | pressed <- False}, Effects.none)
    UselessCrap -> (model, Effects.none)
    Reply s -> ({model | name <- s}, Effects.none)
    ArbJson v -> ({model | name <- (JE.encode 2 v)}, Effects.none)

-- VIEW

(=>) = (,)


-- try VD.onWithOptions for preventing scrolling on touchscreens and 
-- etc. See virtualdom docs.

onClick : Signal.Address Action -> VD.Property
onClick address =
    VD.on "click" JD.value (\v -> Signal.message address (ArbJson v))


view : Signal.Address Action -> Model -> Svg
view address model =
  let ly = (round (model.location * toFloat (model.rect.h))) + model.rect.y
      sly = toString ly
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
        , style "fill: #010101;"
        ]
        []
    , rect
        [ x model.srect.x
        , y sly 
        , width model.srect.w
        , height "3"
        , rx "2"
        , ry "2"
        , style ("fill: " ++ buttColor(model.pressed) ++ ";")
        ]
        []
    , text' [ fill "white", textAnchor "middle" ] [ text model.name ]
    ]


