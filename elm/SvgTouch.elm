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
import Dict

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

parseTouchCount: JD.Decoder Int 
parseTouchCount =
  JD.at [ "touches", "length" ] JD.int

makeTd: List Touch -> Dict.Dict Int Touch
makeTd touchlist = 
  Dict.fromList <| List.map (\t -> (t.id, t)) touchlist

-- should update take a different message for each touch message type, or just one generic one and do the detect?
-- hmm the detect will be done regardless.  
update : Msg -> Model -> Model
update msg model = 
  -- let _ = Debug.log "meh" msg
  case msg of 
    SvgTouchStart v -> 
      case JD.decodeValue parseTouchCount v of 
        Ok touchcount -> 
          let touchresults = List.map 
                (\idx -> JD.decodeValue (JD.at [ "touches", (toString idx) ] parseTouch) v)
                [0..(touchcount - 1)]
              touches = List.foldr (\rst tl -> 
                case rst of 
                  Ok touch -> touch :: tl
                  Err _ -> tl) [] touchresults
              newmodel = { model | touches = makeTd touches } 
            in
              Debug.log "newmodel: " newmodel
        Err str_msg -> 
          Debug.log str_msg model 

    SvgTouchMove v -> model 
      -- I guess this never contains new touches, only changed ones?  Are all included every time?
    SvgTouchEnd v -> model
      -- time to take out the ids from the map I spose. 
    SvgTouchCancel v -> model
    SvgTouchLeave v -> model
 

