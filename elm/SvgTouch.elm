module SvgTouch exposing (..)

{-
    exposing ( Touch, Model, init, update, Msg 
         )
         , SvgTouchStart 
         , SvgTouchMove 
         , SvgTouchEnd 
         , SvgTouchCancel 
         , SvgTouchLeave 
         ) 
-}

import Time
import Dict
import Json.Decode as JD
import String

{- Every `Touch` has `xy` coordinates. It also has an identifier
`id` to distinguish one touch from another.

A touch also keeps info about the initial point and time of contact:
`x0`, `y0`, and `t0`. This helps compute more complicated gestures
like taps, drags, and swipes which need to know about timing or direction.
-}

{- A list of ongoing touches. 
touches : Signal (List Touch)
touches =
  Native.SvgTouch.touches
-}

{- The last position that was tapped. Default value is `{x=0,y=0}`.
Updates whenever the user taps the screen.
taps : Signal { x:Int, y:Int }
taps =
  Native.SvgTouch.taps
-}


type alias Model = { 
  touches: Dict.Dict Int Touch
  }

type Msg 
  = SvgTouchStart JD.Value
  | SvgTouchMove JD.Value
  | SvgTouchEnd JD.Value
  | SvgTouchCancel JD.Value
  | SvgTouchLeave JD.Value
   
init: Model
init = Model Dict.empty

-- should update take a different message for each touch message type, or just one generic one and do the detect?
-- hmm the detect will be done regardless.  
update : Msg -> Model -> Model
update msg model = 
  -- let _ = Debug.log "meh" msg
  case msg of 
    SvgTouchStart v -> 
      let kvp = JD.decodeValue (JD.keyValuePairs JD.value) v
          kstr = 
            case kvp of 
              Ok jkv -> let dastrings = List.map fst jkv in 
                          String.join " " dastrings
              Err meh -> meh
        in 
        Debug.log kstr model
      -- does this event only contain new touches???? 
    SvgTouchMove v -> model 
      -- I guess this never containes new touches, only changed ones?  Are all included every time?
    SvgTouchEnd v -> model
      -- time to take out the ids from the map I spose. 
    SvgTouchCancel v -> model
    SvgTouchLeave v -> model
 
type alias Touch =
    { x : Int
    , y : Int
    , id : Int
    , x0 : Int
    , y0 : Int
    , t0 : Time.Time
    }


