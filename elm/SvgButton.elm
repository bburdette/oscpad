module SvgButton exposing (..)

-- import Effects exposing (Effects, Never)
import Html exposing (Html)
-- import Html.Attributes exposing (style)
-- import Html.Events exposing (onClick)
import Http
import Json.Decode as JD exposing ((:=))
import Json.Encode as JE
import Task
import Svg exposing (Svg, svg, rect, g, text, text', Attribute)
import Svg.Attributes exposing (..)
import Html.Events exposing (onClick, onMouseUp, onMouseDown, onMouseOut)
-- import NoDragEvents exposing (onClick, onMouseUp, onMouseDown, onMouseOut)
import SvgThings
-- import SvgTouch

-- how to specify a button in json.
type alias Spec = 
  { name: String
  , label: Maybe String
  }

jsSpec : JD.Decoder Spec
jsSpec = JD.object2 Spec
  ("name" := JD.string)
  (JD.maybe ("label" := JD.string)) 

-- MODEL

type alias Model =
  { name : String
  , label: String
  , cid: SvgThings.ControlId 
  , rect: SvgThings.Rect
  , srect: SvgThings.SRect
  , pressed: Bool
  , sendf : (String -> Cmd Msg) 
  , textSvg: List (Svg ())
  }

init: (String -> Cmd Msg) -> SvgThings.Rect -> SvgThings.ControlId -> Spec
  -> (Model, Cmd Msg)
init sendf rect cid spec =
  let ts = case spec.label of 
        Just lbtext -> SvgThings.calcTextSvg SvgThings.ff lbtext rect 
        Nothing -> []
  in
  ( Model (spec.name) 
          (Maybe.withDefault "" (spec.label))
          cid
          rect 
          (SvgThings.SRect (toString rect.x)
                           (toString rect.y)
                           (toString rect.w)
                           (toString rect.h))
          False 
          sendf
          ts
  , Cmd.none
  )

buttColor: Bool -> String
buttColor pressed = 
  case pressed of 
    True -> "#f000f0"
    False -> "#60B5CC"

-- UPDATE

type Msg
    = SvgPress 
    | SvgUnpress 
    | UselessCrap 
    | Reply String
    | SvgUpdate UpdateMessage
--    | SvgTouch (List Touch.Touch)

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
    _ -> JD.succeed Unpress 



update : Msg -> Model -> (Model, Cmd Msg) 
update msg model =
  case msg of
    SvgPress -> pressup model Press
    SvgUnpress ->
      case model.pressed of
        True ->  pressup model Unpress
        False -> (model, Cmd.none)
    UselessCrap -> (model, Cmd.none)
    Reply s -> ({model | name = s}, Cmd.none)
    SvgUpdate um -> 
      -- sanity check for ids?  or don't.
      let pressedupdate = case um.updateType of 
                      Press -> True
                      Unpress -> False
        in
      ({ model | pressed = pressedupdate }
       , Cmd.none )
{-    SvgTouch touches -> 
      if List.isEmpty touches then
        if model.pressed == True then 
          pressup model Unpress
        else
          (model , Cmd.none )
      else
        if model.pressed == False then 
          pressup model Press
        else
          (model , Cmd.none )
-}

pressup: Model -> UpdateType -> (Model, Cmd Msg)
pressup model ut = 
  let um = JE.encode 0 (encodeUpdateMessage (UpdateMessage model.cid ut)) in
  ({ model | pressed = (ut == Press)}
    , (model.sendf um) )

-- Effects.task ((model.sendf um) 
--         `Task.andThen` (\_ -> Task.succeed UselessCrap)))


resize: Model -> SvgThings.Rect -> (Model, Cmd Msg)
resize model rect = 
  let ts = SvgThings.calcTextSvg SvgThings.ff model.label rect 
  in
  ({ model | rect = rect, 
            srect = (SvgThings.SRect (toString rect.x)
                                     (toString rect.y)
                                     (toString rect.w)
                                     (toString rect.h))}
  , Cmd.none)


-- VIEW
(=>) = (,)

view : Model -> Svg Msg
view model =
  g [ onMouseDown SvgPress
    , onMouseUp SvgUnpress
    , onMouseOut SvgUnpress
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


