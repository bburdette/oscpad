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
import NoDragEvents exposing (onClick, onMouseUp, onMouseDown, onMouseOut)
import SvgThings

-- how to specify a button in json.
type alias Spec = 
  { name: String
  }

jsSpec : Json.Decoder Spec
jsSpec = Json.object1 Spec ("name" := Json.string)

-- MODEL

type alias Model =
  { name : String
  , srect: SvgThings.SRect
  , pressed: Bool
  , sendf : (String -> Task.Task Never ())
  }


init : (String -> Task.Task Never ()) -> Spec -> SvgThings.Rect 
  -> (Model, Effects Action)
init sendf spec rect =
  ( Model (spec.name) 
          (SvgThings.SRect (toString (rect.x + 5)) 
                           (toString (rect.y + 5))
                           (toString (rect.w - 5))
                           (toString (rect.h - 5)))
          False sendf
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
  g [ onMouseDown (Signal.message address SvgPress)
    , onMouseUp (Signal.message address SvgUnpress)
    , onMouseOut (Signal.message address SvgUnpress)
    ]
    [ rect
        [ x model.srect.x
        , y model.srect.y 
        , width model.srect.w
        , height model.srect.h
        , rx "15"
        , ry "15"
        , style ("fill: " ++ buttColor(model.pressed) ++ ";")
        ]
        []
    , text' [ fill "white", textAnchor "middle" ] [ text model.name ]
    ]


