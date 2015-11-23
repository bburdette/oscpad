module SvgSlider where

import Effects exposing (Effects, Never)
import Html exposing (Html)
import Http
import Json.Decode as JD exposing ((:=))
import Json.Encode as JE 
import Task
import Svg exposing (Svg, svg, rect, g, text, text', Attribute)
import Svg.Attributes exposing (..)
import NoDragEvents exposing (onClick, onMouseUp, onMouseMove, onMouseDown, onMouseOut)
import SvgThings
import VirtualDom as VD
import Touch


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
  , cid: SvgThings.ControlId 
  , rect: SvgThings.Rect
  , srect: SvgThings.SRect
  , orientation: SvgThings.Orientation
  , pressed: Bool
  , location: Float
  , sendf : (String -> Task.Task Never ())
  }

containsXY: Model -> Int -> Int -> Bool
containsXY mod x y = SvgThings.containsXY mod.rect x y

init: (String -> Task.Task Never ()) -> SvgThings.Rect -> SvgThings.ControlId -> Spec
  -> (Model, Effects Action)
init sendf rect cid spec =
  ( Model (spec.name) 
          cid 
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
    = SvgPress JE.Value
    | SvgUnpress JE.Value 
    | UselessCrap 
    | Reply String 
    | SvgMoved JE.Value
    | SvgUpdate UpdateMessage
    | SvgTouch (List Touch.Touch)

getX : JD.Decoder Int
getX = "offsetX" := JD.int 

getY : JD.Decoder Int
getY = "offsetY" := JD.int 

type UpdateType 
  = Press
  | Move
  | Unpress

type alias UpdateMessage = 
  { controlId: SvgThings.ControlId
  , updateType: UpdateType
  , location: Float
  }

encodeUpdateMessage: UpdateMessage -> JD.Value
encodeUpdateMessage um = 
  JE.object [ ("controlType", JE.string "slider") 
            , ("controlId", SvgThings.encodeControlId um.controlId) 
            , ("updateType", encodeUpdateType um.updateType) 
            , ("location", (JE.float um.location))
            ]
  
encodeUpdateType: UpdateType -> JD.Value
encodeUpdateType ut = 
  case ut of 
    Press -> JE.string "Press"
    Move -> JE.string "Move"
    Unpress -> JE.string "Unpress"

jsUpdateMessage : JD.Decoder UpdateMessage
jsUpdateMessage = JD.object3 UpdateMessage 
  ("controlId" := SvgThings.decodeControlId) 
  (("updateType" := JD.string) `JD.andThen` jsUpdateType)
  ("location" := JD.float)
  
jsUpdateType : String -> JD.Decoder UpdateType 
jsUpdateType ut = 
  case ut of 
    "Press" -> JD.succeed Press
    "Move" -> JD.succeed Move
    "Unpress" -> JD.succeed Unpress 



-- get mouse/whatever location from the json message, 
-- compute slider location from that.
getLocation: Model -> JD.Value -> Result String Float
getLocation model v = 
  case model.orientation of 
    SvgThings.Horizontal ->
      case (JD.decodeValue getX v) of 
        Ok i -> Ok ((toFloat (i - model.rect.x)) 
                    / toFloat model.rect.w)
        Err e -> Err e
    SvgThings.Vertical -> 
      case (JD.decodeValue getY v) of 
        Ok i -> Ok ((toFloat (i - model.rect.y)) 
                    / toFloat model.rect.h)
        Err e -> Err e

{-
updLoc: Model -> JD.Value -> (Model, Effects Action) 

      case (getLocation model v) of 
        Ok l -> ({model | location <- l}, Effects.none)
        _ -> (model, Effects.none)
-}


update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    SvgPress v -> 
      case (getLocation model v) of 
        Ok l -> 
          let um = JE.encode 0 (encodeUpdateMessage (UpdateMessage model.cid Press l)) in
            ( {model | location <- l, pressed <- True}
            , Effects.task 
                ((model.sendf um) `Task.andThen` 
                (\_ -> Task.succeed UselessCrap)))
        _ -> (model, Effects.none)
    SvgUnpress v -> 
      case model.pressed of 
        True -> 
          let um = JE.encode 0 (encodeUpdateMessage 
                    (UpdateMessage model.cid Unpress model.location)) in
            ( { model | pressed <- False }
            , Effects.task 
                ((model.sendf um) `Task.andThen` 
                (\_ -> Task.succeed UselessCrap)))
        False -> (model, Effects.none)
    UselessCrap -> (model, Effects.none)
    Reply s -> ({model | name <- s}, Effects.none)
    SvgMoved v ->
      case model.pressed of 
        True -> 
          case (getLocation model v) of 
            Ok l -> 
              let um = JE.encode 0 (encodeUpdateMessage (UpdateMessage model.cid Move l)) in
                ( {model | location <- l}
                , Effects.task 
                    ((model.sendf um) `Task.andThen` 
                    (\_ -> Task.succeed UselessCrap)))
            _ -> (model, Effects.none)
        False -> (model, Effects.none)
    SvgUpdate um -> 
      -- sanity check for ids?  or don't.
      let mod = case um.updateType of 
          Press -> { model | pressed <- True, location <- um.location }
          Move -> { model | location <- um.location }
          Unpress -> { model | pressed <- False, location <- um.location }
        in
      (mod, Effects.none )
    SvgTouch touches -> 
      if List.isEmpty touches then
        ({ model | pressed <- False }, Effects.none )
      else
        case model.orientation of 
          SvgThings.Horizontal -> 
            let locsum = List.foldl (+) 0 (List.map (\t -> t.x) touches)
                locavg = (toFloat locsum) / (toFloat (List.length touches))
                loc = (locavg - (toFloat model.rect.x)) 
                       / toFloat model.rect.w in 
            ({ model | pressed <- True, location <- loc }, Effects.none )
          SvgThings.Vertical -> 
            let locsum = List.foldl (+) 0 (List.map (\t -> t.y) touches)
                locavg = (toFloat locsum) / (toFloat (List.length touches))
                loc = (locavg - (toFloat model.rect.y)) 
                       / toFloat model.rect.h in 
            ({ model | pressed <- True, location <- loc }, Effects.none )


-- VIEW

(=>) = (,)

-- try VD.onWithOptions for preventing scrolling on touchscreens and 
-- etc. See virtualdom docs.


sliderEvt: String -> (JD.Value -> Action) -> Signal.Address Action -> VD.Property
sliderEvt evtname mkaction address =
    VD.onWithOptions evtname (VD.Options True True) JD.value (\v -> Signal.message address (mkaction v))

-- onClick = sliderEvt "click" SvgMoved
--  , onClick address
onMouseMove = sliderEvt "mousemove" SvgMoved
onMouseLeave = sliderEvt "mouseleave" SvgUnpress
onMouseDown = sliderEvt "mousedown" SvgPress
onMouseUp = sliderEvt "mouseup" SvgUnpress

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
  g [ onMouseDown address 
    , onMouseUp address 
    , onMouseLeave address 
    , onMouseMove address 
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


