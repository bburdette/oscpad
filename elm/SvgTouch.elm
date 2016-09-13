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
import Json.Decode as JD exposing ((:=))
import String
import List

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


type alias Touch =
    { x : Int
    , y : Int
    , id : Int
--    , x0 : Int
--    , y0 : Int
--    , t0 : Time.Time
    }

parseTouch: JD.Decoder Touch
parseTouch = 
  JD.object3 Touch
    ("clientX" := JD.int)
    ("clientY" := JD.int)
    ("identifier" := JD.int)
--    ("t0" := JD.int)

-- parseTouchList: JD.Value -> Result String (List Touch)

-- parseTouchList: JD.Decoder (Dict.Dict String Touch)
-- parseTouchList = JD.dict parseTouch 

-- parseTouchList: JD.Decoder (Dict.Dict String String)
-- parseTouchList = JD.dict JD.string

parseTouchEvt: JD.Decoder Touch 
parseTouchEvt =
  JD.andThen 
    (JD.at [ "touches", "length" ] JD.int)
    parseTouches 

-- parseTouches: Int -> JD.Decoder (Dict.Dict Int Touch)
parseTouches: Int -> JD.Decoder Touch 
parseTouches c = 
  JD.at [ "touches", (toString (c - 1)) ] parseTouch
--  (\c -> JD.at [ (toString c) ] parseTouch)

-- should update take a different message for each touch message type, or just one generic one and do the detect?
-- hmm the detect will be done regardless.  
update : Msg -> Model -> Model
update msg model = 
  -- let _ = Debug.log "meh" msg
  case msg of 
    SvgTouchStart v -> 
      let touchies = JD.decodeValue parseTouchEvt v
        in 
        Debug.log (toString touchies) model
{-
      let dcr = JD.decodeValue (JD.dict JD.value) v
          kstr = 
            case dcr of 
              Ok kvd -> let dastrings = Dict.keys kvd
                            _ = case (Dict.get "touches" kvd) of 
                                    Just x -> 
                                      let _ = Debug.log "decoding" " x"
                                          tl = JD.decodeValue parseTouchList x 
                                          _ = Debug.log "touches" tl
                                        in Nothing
                                    Nothing -> Nothing
                in 
                          String.join " " dastrings
              Err meh -> meh
        in 
        Debug.log kstr model
-}
      -- does this event only contain new touches???? 
    SvgTouchMove v -> model 
      -- I guess this never contains new touches, only changed ones?  Are all included every time?
    SvgTouchEnd v -> model
      -- time to take out the ids from the map I spose. 
    SvgTouchCancel v -> model
    SvgTouchLeave v -> model
 

