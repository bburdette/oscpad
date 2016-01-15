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
  , fontScaling: Float
  , labelMeasuredWidth: Int 
  , cid: SvgThings.ControlId 
  , rect: SvgThings.Rect
  , srect: SvgThings.SRect
  }

containsXY: Model -> Int -> Int -> Bool
containsXY mod x y = SvgThings.containsXY mod.rect x y

ff: String
ff = "sans-serif"

init: SvgThings.Rect -> SvgThings.ControlId -> Spec
  -> (Model, Effects Action)
init rect cid spec =
  let fw = SvgTextSize.getTextWidth spec.label ("20px " ++ ff)
      fs = computeFontScaling (toFloat fw) 20.0 (toFloat rect.w) (toFloat rect.h) 
  in
  ( Model (spec.name)
          (spec.label)
          fs
          fw
          cid
          rect 
          (SvgThings.SRect (toString rect.x)
                           (toString rect.y)
                           (toString rect.w)
                           (toString rect.h))
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
      let w = SvgTextSize.getTextWidth um.label ("20px " ++ ff)
          fs = computeFontScaling (toFloat w) 20.0 (toFloat model.rect.w) (toFloat model.rect.h) 
      in
      -- when the text changes, measure it. 
      ({ model | label = um.label, fontScaling = fs, labelMeasuredWidth = w }
      , Effects.none) 
    SvgTouch touches -> (model, Effects.none)

computeFontScaling: Float -> Float -> Float -> Float -> Float 
computeFontScaling fw fh rw rh = 
  let wr = rw / fw
      hr = rh / fh in 
  if wr < hr then wr else hr
    

resize: Model -> SvgThings.Rect -> (Model, Effects Action)
resize model rect = 
  let w = SvgTextSize.getTextWidth model.label ("20px " ++ ff)
      fs = computeFontScaling (toFloat w) 20.0 (toFloat rect.w) (toFloat rect.h) 
  in
  ({ model | rect = rect 
           , srect = (SvgThings.SRect (toString rect.x)
                                      (toString rect.y)
                                      (toString rect.w)
                                      (toString rect.h))
           , fontScaling = fs
           , labelMeasuredWidth = w
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
      svgl = lbrect :: calcText model
  in
  g [ ] svgl

calcText : Model -> List Svg
calcText model = 
  let width = model.labelMeasuredWidth
      scale = model.fontScaling
      xc = ((toFloat model.rect.x) + (toFloat model.rect.w) / 2)
      yc = ((toFloat model.rect.y) + (toFloat model.rect.h) / 2)
      xt = xc - ((toFloat width) * scale * 0.5)
      yt = yc + 20.0 * scale * 0.5
      tmpl = template "matrix(" <% .scale %> ", 0, 0, " <% .scale %> ", "<% .xt %>", "<% .yt %>")"
      xf = render tmpl { scale = toString scale, xt = toString xt, yt = toString yt  }
  in 
    [ text' [ fill "black"  
            -- , textAnchor "middle" 
            -- , x model.middlex 
            -- , y fonty
            -- , lengthAdjust "glyphs"
            -- , textLength model.srect.w 
            -- , fontSize "20" -- model.srect.h
            , fontSize "20px"
            , fontFamily ff
            , transform xf 
            ] 
            [ text model.label ]
    ]

        
