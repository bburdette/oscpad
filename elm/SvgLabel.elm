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
  , fontSize: Int
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
          20          -- default to whatever
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
    | SvgTextSize TextBounds 
    | SvgInt Int
    | SvgTime Time

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
      ({ model | label = um.label }
       , Effects.none )
    SvgTouch touches -> (model, Effects.none)
    SvgTextSize tb -> 
      ({ model | label = String.concat [toString tb.w, " ", toString tb.h] }, Effects.none)
      -- (model, Effects.none)
    SvgInt tw -> 
      ({ model | label = String.concat ["width: ", toString tw] }, Effects.none)
    SvgTime tm -> 
      ({ model | label = toString tm }, Effects.none)

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
  , Effects.task 
    (Task.andThen 
      (SvgTextSize.getTextWidth model.label "20px sans-serif")
      (\tb -> Task.succeed (SvgInt tb))))

{-
      (SvgTextSize.getTextWidth (TextSizeRequest model.label "20px sans-serif"))
      (\tb -> Task.succeed (SvgInt tb))))

      (SvgTextSize.getTextWidth model.label "20px sans-serif")
      (\tw -> Task.succeed (SvgTextSize tw))))

      (SvgTextSize.getTextWidth model.label "20px sans-serif")
      (\tw -> Task.succeed (SvgInt tw))))

      (SvgTextSize.getTextSize (TextSizeRequest model.label "20px sans-serif"))
      (\tb -> Task.succeed (SvgTextSize tb))))

      (SvgTextSize.getTb)
      (\tb -> Task.succeed (SvgTextSize tb))))

      (SvgTextSize.getTInt)
      (\tb -> Task.succeed (SvgInt tb))))

      (SvgTextSize.getCurrentTime)
      (\tb -> Task.succeed (SvgTime tb))))

      (Task.succeed (TextBounds 100 100))
  , Effects.task (Task.succeed (SvgTextSize (TextBounds 100 100))))
  , Effects.none)
-}

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
            -- , lengthAdjust "glyphs"
            -- , textLength model.srect.w 
            , fontSize model.srect.h
            ] 
            [ text model.label ]
    ]

