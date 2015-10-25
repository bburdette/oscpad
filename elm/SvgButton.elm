module SvgButton where

import Effects exposing (Effects, Never)
import Html exposing (Html)
-- import Html.Attributes exposing (style)
-- import Html.Events exposing (onClick)
import Http
import Json.Decode as Json exposing ((:=))
import Task
import Svg exposing (Svg, svg, rect, g, text, text', Attribute)
import Svg.Attributes exposing (..)
import Svg.Events exposing (onClick, onMouseUp, onMouseDown, onMouseOut)

-- how to specify a button in json.
type alias Spec = 
  { name: String
  }

{-
  , x: Int
  , y: Int
  , w: Int
  , h: Int  
-}

jsSpec : Json.Decoder Spec
jsSpec = Json.object1 Spec ("name" := Json.string)

-- MODEL

type alias Model =
  { name : String
  , pressed: Bool
  , sendf : (String -> Task.Task Never ())
  }


init : (String -> Task.Task Never ()) -> Spec ->  
  (Model, Effects Action)
init sendf spec =
  ( Model (spec.name) False sendf
  , Effects.none
  )

buttColor: Bool -> String
buttColor pressed = 
  case pressed of 
    True -> "#f000f0"
    False -> "#60B5CC"


-- UPDATE

type Action
    = SvgPress | SvgUnpress | UselessCrap | Reply String


update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    SvgPress -> ({ model | pressed <- True}, Effects.task 
      ((model.sendf model.name) `Task.andThen` (\_ -> Task.succeed UselessCrap)))
    SvgUnpress -> ({ model | pressed <- False}, Effects.none)
    UselessCrap -> (model, Effects.none)
    Reply s -> ({model | name <- s}, Effects.none)

-- VIEW

(=>) = (,)

view : Signal.Address Action -> Model -> Svg
view address model =
  g [ transform ("translate(100, 100)")
    , onMouseDown (Signal.message address SvgPress)
    , onMouseUp (Signal.message address SvgUnpress)
    , onMouseOut (Signal.message address SvgUnpress)
    ]
    [ rect
        [ x "-50"
        , y "-50"
        , width "100"
        , height "100"
        , rx "15"
        , ry "15"
        , style ("fill: " ++ buttColor(model.pressed) ++ ";")
        ]
        []
    , text' [ fill "white", textAnchor "middle" ] [ text model.name ]
    ]


