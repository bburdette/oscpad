module SvgButton where

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

-- how to specify a button in json.
type alias Spec = 
  { name: String
  }

jsSpec : JD.Decoder Spec
jsSpec = JD.object1 Spec ("name" := JD.string)

-- MODEL

type alias Model =
  { name : String
  , cid: SvgThings.ControlId 
  , srect: SvgThings.SRect
  , pressed: Bool
  , sendf : (String -> Task.Task Never ())
  }


init: (String -> Task.Task Never ()) -> SvgThings.Rect -> SvgThings.ControlId -> Spec
  -> (Model, Effects Action)
init sendf rect cid spec =
  ( Model (spec.name) 
          cid 
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
    = SvgPress 
    | SvgUnpress 
    | UselessCrap 
    | Reply String
    | SvgUpdate UpdateMessage

type UpdateType 
  = Press
  | Unpress

type alias UpdateMessage = 
  { controlId: SvgThings.ControlId
  , updateType: UpdateType
  }

encodeUpdateMessage: UpdateMessage -> JD.Value
encodeUpdateMessage um = 
  JE.object [ ("controlType", JE.string "button") 
            , ("controlId", SvgThings.encodeControlId um.controlId) 
            , ("updateType", encodeUpdateType um.updateType) 
            ]
  
encodeUpdateType: UpdateType -> JD.Value
encodeUpdateType ut = 
  case ut of 
    Press -> JE.string "Press"
    Unpress -> JE.string "Unpress"


jsUpdateMessage : JD.Decoder UpdateMessage
jsUpdateMessage = JD.object2 UpdateMessage 
  ("controlId" := SvgThings.decodeControlId) 
  (("updateType" := JD.string) `JD.andThen` jsUpdateType)
  
jsUpdateType : String -> JD.Decoder UpdateType 
jsUpdateType ut = 
  case ut of 
    "Press" -> JD.succeed Press
    "Unpress" -> JD.succeed Unpress 



update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    SvgPress -> 
      let um = JE.encode 0 (encodeUpdateMessage (UpdateMessage model.cid Press)) in
      ({ model | pressed <- True}, Effects.task 
        ((model.sendf um) `Task.andThen` (\_ -> Task.succeed UselessCrap)))
    SvgUnpress ->
      case model.pressed of
        True ->  
          let um = JE.encode 0 (encodeUpdateMessage (UpdateMessage model.cid Unpress)) in
          ({ model | pressed <- False}, Effects.task 
            ((model.sendf um) `Task.andThen` (\_ -> Task.succeed UselessCrap)))
        False -> (model, Effects.none)
    UselessCrap -> (model, Effects.none)
    Reply s -> ({model | name <- s}, Effects.none)
    SvgUpdate um -> 
      -- sanity check for ids?  or don't.
      let pressedup = case um.updateType of 
                      Press -> True
                      Unpress -> False
        in
      ({ model | pressed <- pressedup }
       , Effects.none )


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


