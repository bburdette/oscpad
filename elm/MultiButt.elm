module MultiButt where

import Effects exposing (Effects, Never)
import Html exposing (..)
import BlahButton
import Task
import List exposing (..)
import Json.Decode as Json exposing ((:=))
import Util exposing (..)

-- json spec
type alias Spec = 
  {
    title: String,
    buttons: List BlahButton.Spec
  }

jsSpec : Json.Decoder Spec
jsSpec = Json.object2 Spec 
  ("name" := Json.string)
  ("buttons" := Json.list BlahButton.jsSpec)


type alias Model =
    {
      title: String, 
      butts: List (ID, BlahButton.Model),
      nextID: ID,
      mahsend : (String -> Task.Task Never ())
    }

type alias ID = Int

-- UPDATE

type Action
    = JsonMsg String 
    | BAction ID BlahButton.Action 

update : Action -> Model -> (Model, Effects Action)
update action model =
  case action of
    JsonMsg s -> 
      let t = Json.decodeString jsSpec s
       in case t of 
          Ok spec -> init model.mahsend spec 
          Err e -> (model, Effects.none)

init: (String -> Task.Task Never ()) -> Spec -> (Model, Effects Action)
init sendf spec = 
  let blist = map (BlahButton.init sendf) spec.buttons
      idxs = [0..(length spec.buttons)]  
      buttz = zip idxs (map fst blist) 
      fx = Effects.batch 
             (map (\(i,a) -> Effects.map (BAction i) a)
                  (zip idxs (map snd blist)))
    in
     (Model spec.title buttz (length spec.buttons) sendf, fx)
      

-- VIEW

(=>) = (,)


view : Signal.Address Action -> Model -> Html
view address model =
  div [] ([text "meh", text model.title, text (toString (length model.butts))] ++ (map (viewBlahButton address) model.butts))


viewBlahButton : Signal.Address Action -> (ID, BlahButton.Model) -> Html
viewBlahButton address (id, model) =
  BlahButton.view (Signal.forwardTo address (BAction id)) model
