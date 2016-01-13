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
  , fontInfo: Maybe (Float, Int)   -- scaling, width
  , cid: SvgThings.ControlId 
  , rect: SvgThings.Rect
  , srect: SvgThings.SRect
  , middlex: String
  , middley: String
  }

containsXY: Model -> Int -> Int -> Bool
containsXY mod x y = SvgThings.containsXY mod.rect x y

ff: String
ff = "sans-serif"

init: SvgThings.Rect -> SvgThings.ControlId -> Spec
  -> (Model, Effects Action)
init rect cid spec =
  ( Model (spec.name)
          (spec.label)
          Nothing
          cid
          rect 
          (SvgThings.SRect (toString (rect.x + 5)) 
                           (toString (rect.y + 5))
                           (toString (rect.w - 5))
                           (toString (rect.h - 5)))
          (toString ((toFloat rect.x) + (toFloat rect.w) / 2))
          (toString ((toFloat rect.y) + (toFloat rect.h) / 2))
  , Effects.task 
    (Task.andThen 
      (SvgTextSize.getTextWidth spec.label ("20px " ++ ff))
      (\tb -> Task.succeed (SvgTextWidth tb))))

-- UPDATE

type Action
    = SvgUpdate UpdateMessage
    | SvgTouch (List Touch.Touch)
    | SvgTextWidth Int

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
      -- when the text changes, measure it. 
      ({ model | label = um.label }
      , Effects.task 
        (Task.andThen 
          (SvgTextSize.getTextWidth um.label ("20px " ++ ff))
          (\tb -> Task.succeed (SvgTextWidth tb))))
    SvgTouch touches -> (model, Effects.none)
    SvgTextWidth w ->
      let fs = computeFontScaling (toFloat w) 20.0 (toFloat model.rect.w) (toFloat model.rect.h) 
      in
      Debug.log (toString fs)
      ({ model | fontInfo = Just (fs,w) }, Effects.none)
      -- ({ model | label = toString w, fontScale = Just fs }, Effects.none)

computeFontScaling: Float -> Float -> Float -> Float -> Float 
computeFontScaling fw fh rw rh = 
  let wr = rw / fw
      hr = rh / fh in 
  Debug.log (toString (List.map toString [fw,fh,rw,rh])) 
    (if wr < hr then wr else hr)
    

{-

 var width=336, height=107;
 var textNode = document.getElementById("t1");
 var bb = textNode.getBBox();
 var widthTransform = width / bb.width;
 var heightTransform = height / bb.height;
 var value = widthTransform < heightTransform ? widthTransform : heightTransform;
 textNode.setAttribute("transform", "matrix("+value+", 0, 0, "+value+", 0,0)");

-}


resize: Model -> SvgThings.Rect -> (Model, Effects Action)
resize model rect = 
  ({ model | rect = rect  
           , srect = (SvgThings.SRect (toString (rect.x + 5)) 
                                     (toString (rect.y + 5))
                                     (toString (rect.w - 5))
                                     (toString (rect.h - 5))) 
          , middlex = (toString ((toFloat rect.x) + (toFloat rect.w) / 2))
          , middley = (toString ((toFloat rect.y) + (toFloat rect.h) / 2))
    }
  , Effects.none)

-- VIEW
(=>) = (,)

{-
      fs = case model.fontScale of
             Just fs -> fs
             Nothing -> 1.0

-}

view : Signal.Address Action -> Model -> Svg
view address model =
  let -- fonty = toString ((toFloat model.rect.y) + (toFloat model.rect.h) * 0.9)  
      -- fs = 2.0
      lbrect = rect
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
  case model.fontInfo of
    Nothing -> []
    Just (scale,width) -> 
      let xc = ((toFloat model.rect.x) + (toFloat model.rect.w) / 2)
          yc = ((toFloat model.rect.y) + (toFloat model.rect.h) / 2)
          xt = xc - ((toFloat width) * scale * 0.5)
          -- xt = toFloat width * scale
          -- xt = xc
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

        
