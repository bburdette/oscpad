module SvgLabel exposing (..) 

import Html exposing (Html)
-- import Html.Attributes exposing (style)
-- import Html.Events exposing (onClick)
import Http
import Json.Decode as JD exposing ((:=))
import Json.Encode as JE
import Task
import Svg exposing (Svg, svg, rect, g, text, text', Attribute)
import Svg.Attributes exposing (..)
-- import NoDragEvents exposing (onClick, onMouseUp, onMouseDown, onMouseOut)
import SvgThings
-- import SvgTouch
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
  , textSvg: List (Svg ())
  }

init: SvgThings.Rect -> SvgThings.ControlId -> Spec
  -> (Model, Cmd Msg)
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
  , Cmd.none)

-- UPDATE

type Msg
    = SvgUpdate UpdateMessage
--    | SvgTouch (List Touch.Touch)

type alias UpdateMessage = 
  { controlId: SvgThings.ControlId
  , label: String 
  }

jsUpdateMessage : JD.Decoder UpdateMessage
jsUpdateMessage = JD.object2 UpdateMessage 
  ("controlId" := SvgThings.decodeControlId) 
  ("label" := JD.string)
  
update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    SvgUpdate um ->
      let ts = SvgThings.calcTextSvg SvgThings.ff um.label model.rect 
      in
      -- when the text changes, measure it. 
      ({ model | label = um.label, textSvg = ts }
      , Cmd.none) 
--    SvgTouch touches -> (model, Cmd.none)

resize: Model -> SvgThings.Rect -> (Model, Cmd Msg)
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
  , Cmd.none)

-- VIEW
(=>) = (,)

view : Model -> Svg ()
view model =
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


 
