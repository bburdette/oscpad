module SvgLabel where

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
import NoDragEvents exposing (onClick, onMouseUp, onMouseDown, onMouseOut)
import SvgThings
import Touch
import SvgTextSize exposing (..)
import Time exposing (..)
import String
import Template exposing (template, render)
import Template.Infix exposing ((<%), (%>))

type alias Spec = 
  { name: String
  , label: String
  }

jsSpec : JD.Decoder Spec
jsSpec = JD.object2 Spec 
  ("name" := JD.string)
  ("label" := JD.string)

-- MODEL

type alias Model =
  { name : String
  , label: String
  , cid: SvgThings.ControlId 
  , rect: SvgThings.Rect
  , srect: SvgThings.SRect
  , textSvg: List Svg
  }

init: SvgThings.Rect -> SvgThings.ControlId -> Spec
  -> (Model, Effects Action)
init rect cid spec =
  let ts = SvgThings.calcTextSvg SvgThings.ff spec.label rect 
  in
  ( Model (spec.name)
          (spec.label)
          cid
          rect 
          (SvgThings.SRect (toString rect.x)
                           (toString rect.y)
                           (toString rect.w)
                           (toString rect.h))
          ts
  , Effects.none)

-- UPDATE

type Action
    = SvgUpdate UpdateMessage
    | SvgTouch (List Touch.Touch)

type alias UpdateMessage = 
  { controlId: SvgThings.ControlId
  , label: String 
  }

jsUpdateMessage : JD.Decoder UpdateMessage
jsUpdateMessage = JD.object2 UpdateMessage 
  ("controlId" := SvgThings.decodeControlId) 
  ("label" := JD.string)
  
update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    SvgUpdate um ->
      let ts = SvgThings.calcTextSvg SvgThings.ff um.label model.rect 
      in
      -- when the text changes, measure it. 
      ({ model | label = um.label, textSvg = ts }
      , Effects.none) 
    SvgTouch touches -> (model, Effects.none)

resize: Model -> SvgThings.Rect -> (Model, Effects Action)
resize model rect = 
  let ts = SvgThings.calcTextSvg SvgThings.ff model.label rect 
  in
  ({ model | rect = rect 
           , srect = (SvgThings.SRect (toString rect.x)
                                      (toString rect.y)
                                      (toString rect.w)
                                      (toString rect.h))
           , textSvg = ts
    }
  , Effects.none)

-- VIEW
(=>) = (,)

view : Signal.Address Action -> Model -> Svg
view address model =
  let lbrect = rect
        [ x model.srect.x
        , y model.srect.y 
        , width model.srect.w
        , height model.srect.h
        , rx "15"
        , ry "15"
        , style "fill: #A1A1A1;"
        ]
        []
      svgl = lbrect :: model.textSvg 
  in
  g [ ] svgl


 
