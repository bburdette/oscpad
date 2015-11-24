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
  , middlex: String
  , middley: String
  }

containsXY: Model -> Int -> Int -> Bool
containsXY mod x y = SvgThings.containsXY mod.rect x y

init: SvgThings.Rect -> SvgThings.ControlId -> Spec
  -> (Model, Effects Action)
init rect cid spec =
  ( Model (spec.name) 
          (spec.label)
          cid
          rect 
          (SvgThings.SRect (toString (rect.x + 5)) 
                           (toString (rect.y + 5))
                           (toString (rect.w - 5))
                           (toString (rect.h - 5)))
          (toString ((toFloat rect.x) + (toFloat rect.w) / 2))
          (toString ((toFloat rect.y) + (toFloat rect.h) / 2))
  , Effects.none
  )

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
      -- sanity check for ids?  or don't.
      ({ model | label <- um.label }
       , Effects.none )
    SvgTouch touches -> (model, Effects.none)


-- VIEW
(=>) = (,)

view : Signal.Address Action -> Model -> Svg
view address model =
  let fonty = toString ((toFloat model.rect.y) + (toFloat model.rect.h) * 0.9) in
  g [ ]
    [ rect
        [ x model.srect.x
        , y model.srect.y 
        , width model.srect.w
        , height model.srect.h
        , rx "15"
        , ry "15"
        , style "fill: #A1A1A1;"
        ]
        []
    , text' [ fill "black"  
            , textAnchor "middle" 
            , x model.middlex 
            , y fonty
            , lengthAdjust "glyphs"
            -- , textLength model.srect.w 
            , fontSize model.srect.h
            ] 
            [ text model.label ]
    ]

