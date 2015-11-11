module Controls where 

import SvgButton
import SvgSlider
import Json.Decode as JD exposing ((:=))
import Effects exposing (Effects, Never)
import Task
import Dict exposing (..)
import List exposing (..)
import Svg exposing (Svg)
import Svg.Attributes as SA 
import SvgThings
import Signal
import Html

----------------------------------------------------------
-- Two things (objects?) in this file; control container 
-- and sizer.  They are mutually recursive so they have to
-- both be in a single file.  

-------------------- control container -------------------

type Spec = CsButton SvgButton.Spec 
          | CsSlider SvgSlider.Spec
          | CsSizer SzSpec

type Model = CmButton SvgButton.Model
           | CmSlider SvgSlider.Model
           | CmSizer SzModel

type Action = CaButton SvgButton.Action
            | CaSlider SvgSlider.Action
            | CaSizer SzAction

jsSpec : JD.Decoder Spec 
jsSpec = 
  ("type" := JD.string) `JD.andThen` jsCs

jsCs : String -> JD.Decoder Spec
jsCs t = 
  case t of 
    "button" -> SvgButton.jsSpec `JD.andThen` (\a -> JD.succeed (CsButton a))
    "slider" -> SvgSlider.jsSpec `JD.andThen` (\a -> JD.succeed (CsSlider a))
    "sizer" -> jsSzSpec `JD.andThen` (\a -> JD.succeed (CsSizer a))

jsUpdateMessage: JD.Decoder Action
jsUpdateMessage = 
  ("controlType" := JD.string) `JD.andThen` jsUmType

jsUmType: String -> JD.Decoder Action 
jsUmType wat = 
  case wat of 
    "slider" -> SvgSlider.jsUpdateMessage `JD.andThen` 
                  (\x -> JD.succeed (CaSlider (SvgSlider.SvgUpdate x)))
    "button" -> SvgButton.jsUpdateMessage `JD.andThen` 
                  (\x -> JD.succeed (CaButton (SvgButton.SvgUpdate x)))

myTail: List a -> List a
myTail lst = 
  let tl = tail lst in 
  case tl of 
    Just l -> l
    Nothing -> []

toCtrlAction: SvgThings.ControlId -> Action -> Action
toCtrlAction id action = 
  case (head id) of 
    Nothing -> action
    Just x -> CaSizer (SzCAction x (toCtrlAction (myTail id) action))


{-
type UpdateMessage 
  = SliderUp SvgSlider.UpdateMessage
  | ButtonUp SvgButton.UpdateMessage

jsUpdateMessage: JD.Decoder UpdateMessage
jsUpdateMessage = 
  ("controlType" := JD.string) `JD.andThen` jsUmType

jsUmType: String -> JD.Decoder UpdateMessage
jsUmType wat = 
  case wat of 
    "slider" -> SvgSlider.jsUpdateMessage `JD.andThen` 
                  (\x -> JD.succeed (SliderUp x))
    "button" -> SvgButton.jsUpdateMessage `JD.andThen` 
                  (\x -> JD.succeed (ButtonUp x))
-}


update : Action -> Model -> (Model, Effects Action)
update action model =
  case (action,model) of 
    (CaButton act, CmButton m) -> 
      let (a,b) = (SvgButton.update act m) in
        (CmButton a, Effects.map CaButton b)
    (CaSlider act, CmSlider m) -> 
      let (a,b) = (SvgSlider.update act m) in
        (CmSlider a, Effects.map CaSlider b)
    (CaSizer act, CmSizer m) -> 
      let (a,b) = (szupdate act m) in
        (CmSizer a, Effects.map CaSizer b)

init : (String -> Task.Task Never ()) -> SvgThings.Rect -> SvgThings.ControlId -> Spec
  -> (Model, Effects Action)
init sendf rect cid spec =
  case spec of 
    CsButton s -> 
      let (a,b) = (SvgButton.init sendf rect cid s) in
        (CmButton a, Effects.map CaButton b)
    CsSlider s -> 
      let (a,b) = (SvgSlider.init sendf rect cid s) in
        (CmSlider a, Effects.map CaSlider b)
    CsSizer s -> 
      let (a,b) = (szinit sendf rect cid s) in
        (CmSizer a, Effects.map CaSizer b)


view : Signal.Address Action -> Model -> Svg
view address model = 
  case model of 
    CmButton m -> SvgButton.view (Signal.forwardTo address CaButton)  m
    CmSlider m -> SvgSlider.view (Signal.forwardTo address CaSlider)  m
    CmSizer m -> szview (Signal.forwardTo address CaSizer)  m

-------------------- sizer -------------------

-- json spec
type alias SzSpec = 
  { name: String
  , orientation: SvgThings.Orientation
  , controls: List Spec
  }

jsSzSpec : JD.Decoder SzSpec
jsSzSpec = JD.object3 SzSpec 
  ("name" := JD.string)
  (("orientation" := JD.string) `JD.andThen` SvgThings.jsOrientation)
  ("controls" := JD.list (lazy (\_ -> jsSpec)))

-- Hack because recursion is sort of broked I guess
-- have to use this above instead of plain jsSpec.
lazy : (() -> JD.Decoder a) -> JD.Decoder a
lazy thunk =
  JD.customDecoder JD.value
      (\js -> JD.decodeValue (thunk ()) js)

type alias SzModel =
  { name: String  
  , cid: SvgThings.ControlId
  , controls: Dict ID Model 
  , szspec: SzSpec
  }

type alias ID = Int

-- UPDATE

type SzAction
    = SzCAction ID Action 

zip = List.map2 (,)

szupdate : SzAction -> SzModel -> (SzModel, Effects SzAction)
szupdate action model =
  case action of
    SzCAction id act -> 
      let bb = get id model.controls in
      case bb of 
        Just bm -> 
          let wha = update act bm 
              updcontrols = insert id (fst wha) model.controls
              newmod = { model | controls <- updcontrols }
            in
              (newmod, Effects.map (SzCAction id) (snd wha))
        Nothing -> (model, Effects.none) 
        
szinit: (String -> Task.Task Never ()) -> SvgThings.Rect -> SvgThings.ControlId -> SzSpec
  -> (SzModel, Effects SzAction)
szinit sendf rect cid szspec = 
  let rlist = case szspec.orientation of 
        SvgThings.Horizontal -> SvgThings.hrects rect (List.length szspec.controls)
        SvgThings.Vertical -> SvgThings.vrects rect (List.length szspec.controls)
      blist = List.map 
                (\(spec, rect, idx) -> init sendf rect (cid ++ [idx]) spec) 
                (map3 (,,) szspec.controls rlist idxs)
      idxs = [0..(length szspec.controls)]  
      controlz = zip idxs (List.map fst blist) 
      fx = Effects.batch 
             (List.map (\(i,a) -> Effects.map (SzCAction i) a)
                  (zip idxs (List.map snd blist)))
    in
     (SzModel szspec.name cid (Dict.fromList controlz) szspec, fx)
      

-- VIEW

(=>) = (,)

szview : Signal.Address SzAction -> SzModel -> Svg
szview address model =
  let controllst = Dict.toList model.controls in 
  Svg.g [] (List.map (viewSvgControls address) controllst)

viewSvgControls : Signal.Address SzAction -> (ID, Model) -> Svg.Svg 
viewSvgControls address (id, model) =
  view (Signal.forwardTo address (SzCAction id)) model

