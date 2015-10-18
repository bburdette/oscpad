module BlahButton where

import Effects exposing (Effects, Never)
import Html exposing (..)
import Html.Attributes exposing (style)
import Html.Events exposing (onClick)
import Http
import Json.Decode as Json exposing ((:=))
import Task


-- how to specify a button in json.
type alias Spec = 
  {
    name: String
  }

jsSpec : Json.Decoder Spec
jsSpec = Json.object1 Spec ("name" := Json.string)

-- MODEL

type alias Model =
    { name : String,
      mahsend : (String -> Task.Task Never ())
    }


init : (String -> Task.Task Never ()) -> 
  Spec ->  
  (Model, Effects Action)
init sendf spec =
  ( Model (spec.name) sendf
  , Effects.none
  )


-- UPDATE

type Action
    = BlahClick | UselessCrap | Reply String


update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    BlahClick -> (model, Effects.task 
      ((model.mahsend model.name) `Task.andThen` (\_ -> Task.succeed UselessCrap)))
    UselessCrap -> (model, Effects.none)
    Reply s -> (Model s (model.mahsend), Effects.none)

-- VIEW

(=>) = (,)


view : Signal.Address Action -> Model -> Html
view address model =
  div [ style [ "width" => "200px" ] ]
    [ h2 [headerStyle] [text model.name]
    , button [ onClick address BlahClick ] [ text model.name ]
    ]


headerStyle : Attribute
headerStyle =
  style
    [ "width" => "200px"
    , "text-align" => "center"
    ]


imgStyle : String -> Attribute
imgStyle url =
  style
    [ "display" => "inline-block"
    , "width" => "200px"
    , "height" => "200px"
    , "background-position" => "center center"
    , "background-size" => "cover"
    , "background-image" => ("url('" ++ url ++ "')")
    ]


-- EFFECTS

{-
getBlahButton : String -> Effects Action
getBlahButton topic =
  Http.get decodeUrl (randomUrl topic)
    |> Task.toMaybe
    |> Task.map NewGif
    |> Effects.task


randomUrl : String -> String
randomUrl topic =
  Http.url "http://api.giphy.com/v1/gifs/random"
    [ "api_key" => "dc6zaTOxFJmzC"
    , "tag" => topic
    ]


decodeUrl : Json.Decoder String
decodeUrl =
  Json.at ["data", "image_url"] Json.string
-}
