module SvgControl where 

import SvgButton
import SvgSlider
import SvgLabel
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
import Touch

----------------------------------------------------------
-- Two things (objects?) in this file; control container 
-- and sizer.  They are mutually recursive so they have to
-- both be in a single file.  

border: Int
border = 1

-------------------- control container -------------------

type Spec = CsButton SvgButton.Spec 
          | CsSlider SvgSlider.Spec
          | CsLabel SvgLabel.Spec
          | CsSizer SzSpec

type Model = CmButton SvgButton.Model
           | CmSlider SvgSlider.Model
           | CmLabel SvgLabel.Model
           | CmSizer SzModel

type Action = CaButton SvgButton.Action
            | CaSlider SvgSlider.Action
            | CaLabel SvgLabel.Action
            | CaSizer SzAction

findControl: Int -> Int -> Model -> Maybe Model
findControl x y mod = 
  case mod of 
    CmButton bmod -> 
      if (SvgThings.containsXY bmod.rect x y) then
        Just mod
      else
        Nothing
    CmSlider smod -> 
      if (SvgThings.containsXY smod.rect x y) then
        Just mod
      else
        Nothing
    CmLabel smod -> Nothing 
    CmSizer szmod -> szFindControl szmod x y
      
controlId: Model -> SvgThings.ControlId 
controlId mod = 
  case mod of 
    CmButton bmod -> bmod.cid
    CmSlider smod -> smod.cid
    CmLabel smod -> smod.cid
    CmSizer szmod -> szmod.cid
      
controlName: Model -> Maybe String
controlName mod = 
  case mod of 
    CmButton bmod -> Just bmod.name
    CmSlider smod -> Just smod.name
    CmLabel smod -> Just smod.name
    CmSizer szmod -> Nothing

tupMap2: (a -> c) -> (b -> d) -> (a,b) -> (c,d)
tupMap2 fa fb ab = (fa (fst ab), fb (snd ab))

resize: Model -> SvgThings.Rect -> (Model, Effects Action)
resize model rect = 
  case model of 
    CmButton mod -> tupMap2 CmButton (Effects.map CaButton) (SvgButton.resize mod (SvgThings.shrinkRect border rect)) 
    CmSlider mod -> tupMap2 CmSlider (Effects.map CaSlider) (SvgSlider.resize mod (SvgThings.shrinkRect border rect)) 
    CmLabel mod -> tupMap2 CmLabel (Effects.map CaLabel) (SvgLabel.resize mod (SvgThings.shrinkRect border rect)) 
    CmSizer mod -> tupMap2 CmSizer (Effects.map CaSizer) (szresize mod rect) 

type alias ControlTam = ((List Touch.Touch) -> Maybe Action)
    
controlTouchActionMaker: Model -> ControlTam 
controlTouchActionMaker ctrl = 
  case ctrl of
    CmButton _ -> (\t -> Just (CaButton (SvgButton.SvgTouch t)))
    CmSlider _ -> (\t -> Just (CaSlider (SvgSlider.SvgTouch t)))
    CmLabel _ -> (\t -> Nothing) 
    CmSizer _ -> (\t -> Nothing)
  
jsSpec : JD.Decoder Spec
jsSpec = 
  ("type" := JD.string) `JD.andThen` jsCs

jsCs : String -> JD.Decoder Spec
jsCs t = 
  case t of 
    "button" -> SvgButton.jsSpec `JD.andThen` (\a -> JD.succeed (CsButton a))
    "slider" -> SvgSlider.jsSpec `JD.andThen` (\a -> JD.succeed (CsSlider a))
    "label" -> SvgLabel.jsSpec `JD.andThen` (\a -> JD.succeed (CsLabel a))
    "sizer" -> jsSzSpec `JD.andThen` (\a -> JD.succeed (CsSizer a))
    _ -> JD.fail ("unkown type: " ++ t)

jsUpdateMessage: JD.Decoder Action
jsUpdateMessage = 
  ("controlType" := JD.string) `JD.andThen` jsUmType

jsUmType: String -> JD.Decoder Action
jsUmType wat = 
  case wat of 
    "button" -> SvgButton.jsUpdateMessage `JD.andThen` 
                  (\x -> JD.succeed (toCtrlAction x.controlId (CaButton (SvgButton.SvgUpdate x))))
    "slider" -> SvgSlider.jsUpdateMessage `JD.andThen` 
                  (\x -> JD.succeed (toCtrlAction x.controlId (CaSlider (SvgSlider.SvgUpdate x))))
    "label" -> SvgLabel.jsUpdateMessage `JD.andThen` 
                  (\x -> JD.succeed (toCtrlAction x.controlId (CaLabel (SvgLabel.SvgUpdate x))))
    _ -> JD.fail ("unknown update type" ++ wat)

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

update : Action -> Model -> (Model, Effects Action)
update action model =
  case (action,model) of 
    (CaButton act, CmButton m) -> 
      let (a,b) = (SvgButton.update act m) in
        (CmButton a, Effects.map CaButton b)
    (CaSlider act, CmSlider m) -> 
      let (a,b) = (SvgSlider.update act m) in
        (CmSlider a, Effects.map CaSlider b)
    (CaLabel act, CmLabel m) -> 
      let (a,b) = (SvgLabel.update act m) in
        (CmLabel a, Effects.map CaLabel b)
    (CaSizer act, CmSizer m) -> 
      let (a,b) = (szupdate act m) in
        (CmSizer a, Effects.map CaSizer b)
    _ -> (model, Effects.none)    -- should probably produce an error.  to the user??

init : (String -> Task.Task Never ()) -> SvgThings.Rect -> SvgThings.ControlId -> Spec
  -> (Model, Effects Action)
init sendf rect cid spec =
  case spec of 
    CsButton s -> 
      let (a,b) = (SvgButton.init sendf (SvgThings.shrinkRect border rect) cid s) in
        (CmButton a, Effects.map CaButton b)
    CsSlider s -> 
      let (a,b) = (SvgSlider.init sendf (SvgThings.shrinkRect border rect) cid s) in
        (CmSlider a, Effects.map CaSlider b)
    CsLabel s -> 
      let (a,b) = (SvgLabel.init (SvgThings.shrinkRect border rect) cid s) in
        (CmLabel a, Effects.map CaLabel b)
    CsSizer s -> 
      let (a,b) = (szinit sendf rect cid s) in
        (CmSizer a, Effects.map CaSizer b)

view : Signal.Address Action -> Model -> Svg
view address model = 
  case model of 
    CmButton m -> SvgButton.view (Signal.forwardTo address CaButton)  m
    CmSlider m -> SvgSlider.view (Signal.forwardTo address CaSlider)  m
    CmLabel m -> SvgLabel.view (Signal.forwardTo address CaLabel)  m
    CmSizer m -> szview (Signal.forwardTo address CaSizer)  m

-------------------- sizer -------------------

-- json spec
type alias SzSpec = 
  { orientation: SvgThings.Orientation
  , proportions: Maybe (List Float)
  , controls: List Spec
  }

-- proportions should all add up to 1.0
processProps: List Float -> List Float
processProps lst = 
  let s = sum lst in
  List.map (\x -> x / s) lst

jsSzSpec : JD.Decoder SzSpec
jsSzSpec = JD.object3 SzSpec
  (("orientation" := JD.string) `JD.andThen` SvgThings.jsOrientation)
  ((JD.maybe ("proportions" := JD.list JD.float)) `JD.andThen` 
    (\x -> JD.succeed (Maybe.map processProps x)))
  ("controls" := (JD.list (lazy (\_ -> jsSpec))))

-- Hack because recursion is sort of broked I guess
-- have to use this above instead of plain jsSpec.
lazy : (() -> JD.Decoder a) -> JD.Decoder a
lazy thunk =
  JD.customDecoder JD.value
      (\js -> JD.decodeValue (thunk ()) js)

type alias SzModel =
  { 
    cid: SvgThings.ControlId
  , rect: SvgThings.Rect
  , controls: Dict ID Model 
  , orientation: SvgThings.Orientation
  , proportions: Maybe (List Float)
  }

type alias ID = Int

szFindControl: SzModel -> Int -> Int -> Maybe Model
szFindControl mod x y = 
  if (SvgThings.containsXY mod.rect x y) then
    firstJust (findControl x y) (values mod.controls)
  else
    Nothing 
   

firstJust : (a -> Maybe b) -> List a -> Maybe b
firstJust f xs =
  case head xs of 
    Nothing -> Nothing
    Just x -> 
      case f x of 
        Just v -> Just v
        Nothing -> Maybe.andThen (tail xs) (firstJust f) 


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
              newmod = { model | controls = updcontrols }
            in
              (newmod, Effects.map (SzCAction id) (snd wha))
        Nothing -> (model, Effects.none) 
 
szresize : SzModel -> SvgThings.Rect -> (SzModel, Effects SzAction)
szresize model rect = 
  let clist = Dict.toList(model.controls)
      rlist = mkRlist model.orientation rect (List.length clist) model.proportions
      ctlsNeffs = List.map (\((i,c),r) -> (i, resize c r)) (zip clist rlist)
      controls = List.map (\(i,(c,efs)) -> (i,c)) ctlsNeffs
      effs = Effects.batch 
          (List.map 
            (\(i,(c,efs)) -> (Effects.map (\ef -> SzCAction i ef) efs))
            ctlsNeffs)
      cdict = Dict.fromList(controls)
    in
     ( { model | rect = rect, controls = cdict }
     , effs )
         
mkRlist: SvgThings.Orientation -> SvgThings.Rect -> Int -> Maybe (List Float) -> List SvgThings.Rect
mkRlist orientation rect count mbproportions = 
  case orientation of 
    SvgThings.Horizontal -> 
      case mbproportions of 
        Nothing -> SvgThings.hrects rect count  
        Just p -> SvgThings.hrectsp rect count p 
    SvgThings.Vertical -> 
      case mbproportions of 
        Nothing -> SvgThings.vrects rect count 
        Just p -> SvgThings.vrectsp rect count p


        
szinit: (String -> Task.Task Never ()) -> SvgThings.Rect -> SvgThings.ControlId -> SzSpec
  -> (SzModel, Effects SzAction)
szinit sendf rect cid szspec = 
  let rlist = mkRlist szspec.orientation rect (List.length szspec.controls) szspec.proportions
      blist = List.map 
                (\(spec, rect, idx) -> init sendf rect (cid ++ [idx]) spec) 
                (map3 (,,) szspec.controls rlist idxs)
      idxs = [0..(length szspec.controls)]  
      controlz = zip idxs (List.map fst blist) 
      fx = Effects.batch 
             (List.map (\(i,a) -> Effects.map (SzCAction i) a)
                  (zip idxs (List.map snd blist)))
    in
     (SzModel cid rect (Dict.fromList controlz) szspec.orientation szspec.proportions, fx)
      

-- VIEW

(=>) = (,)

szview : Signal.Address SzAction -> SzModel -> Svg
szview address model =
  let controllst = Dict.toList model.controls in 
  Svg.g [] (List.map (viewSvgControls address) controllst)

viewSvgControls : Signal.Address SzAction -> (ID, Model) -> Svg.Svg 
viewSvgControls address (id, model) =
  view (Signal.forwardTo address (SzCAction id)) model

