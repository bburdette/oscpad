module SvgTouch exposing (..)


import Time
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


type alias Model = { 
  touches: List Touch
  }

type Msg 
  = SvgTouchStart JD.Value
  | SvgTouchMove JD.Value
  | SvgTouchEnd JD.Value
  | SvgTouchCancel JD.Value
  | SvgTouchLeave JD.Value
   
init: Model
init = Model []


type alias Touch =
    { x : Float
    , y : Float
    , id : Int
--    , x0 : Int
--    , y0 : Int
--    , t0 : Time.Time
    }

parseTouch: JD.Decoder Touch
parseTouch = 
  JD.object3 Touch
    ("clientX" := JD.float)
    ("clientY" := JD.float)
    ("identifier" := JD.int)

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
      { model | touches = extractTouches v }
    SvgTouchMove v -> model 
    SvgTouchEnd v -> model
    SvgTouchCancel v -> model
    SvgTouchLeave v -> model

-- should update take a different message for each touch message type, or just one generic one and do the detect?
-- hmm the detect will be done regardless.  
extractFirstTouchSE : Msg -> Maybe Touch 
extractFirstTouchSE msg = 
  -- let _ = Debug.log "meh" msg
  case msg of 
    SvgTouchStart v -> extractFirstTouch v
    SvgTouchMove v -> extractFirstTouch v
    SvgTouchEnd v -> Nothing
    SvgTouchCancel v -> Nothing
    SvgTouchLeave v -> Nothing

extractTouches: JD.Value -> List Touch
extractTouches evt = 
  case JD.decodeValue parseTouchCount evt of 
    Ok touchcount -> 
      let touchresults = List.map 
            (\idx -> JD.decodeValue (JD.at [ "touches", (toString idx) ] parseTouch) evt)
            [0..(touchcount - 1)]
          touches = List.foldr (\rst tl -> 
            case rst of 
              Ok touch -> touch :: tl
              Err e -> Debug.log e tl) [] touchresults
        in
        touches 
    Err str_msg -> 
      Debug.log str_msg [] 

extractTouchDict: JD.Value -> Dict.Dict Int Touch
extractTouchDict evt = 
  case JD.decodeValue parseTouchCount evt of 
    Ok touchcount -> 
      let touchresults = List.map 
            (\idx -> JD.decodeValue (JD.at [ "touches", (toString idx) ] parseTouch) evt)
            [0..(touchcount - 1)]
          touches = List.foldr (\rst tl -> 
            case rst of 
              Ok touch -> touch :: tl
              Err e -> Debug.log e tl) [] touchresults
        in
        makeTd touches 
    Err str_msg -> 
      Debug.log str_msg Dict.empty 

extractFirstTouch: JD.Value -> Maybe Touch
extractFirstTouch evt = 
  case JD.decodeValue (JD.at [ "touches", "0" ] parseTouch) evt of
    Ok touch -> Just touch
    Err e -> Debug.log e Nothing

