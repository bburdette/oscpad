module SvgTouch exposing (..)


import Time
import Json.Decode as JD exposing ((:=))
import String
import List
import Dict
import SvgThings

{- Every `Touch` has `xy` coordinates. It also has an identifier
`id` to distinguish one touch from another.

A touch also keeps info about the initial point and time of contact:
`x0`, `y0`, and `t0`. This helps compute more complicated gestures
like taps, drags, and swipes which need to know about timing or direction.
-}

type Msg 
  = SvgTouchStart JD.Value
  | SvgTouchMove JD.Value
  | SvgTouchEnd JD.Value
  | SvgTouchCancel JD.Value
  | SvgTouchLeave JD.Value
   
type alias Touch =
    { x : Float
    , y : Float
    , id : Int
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

extractFirstTouchSE : Msg -> Maybe Touch 
extractFirstTouchSE msg = 
  case msg of 
    SvgTouchStart v -> extractFirstTouch v
    SvgTouchMove v -> extractFirstTouch v
    SvgTouchEnd v -> Nothing
    SvgTouchCancel v -> Nothing
    SvgTouchLeave v -> Nothing

extractFirstRectTouchSE : Msg -> SvgThings.Rect -> Maybe Touch 
extractFirstRectTouchSE msg rect = 
  case msg of 
    SvgTouchStart v -> extractFirstTouchInRect v rect
    SvgTouchMove v -> extractFirstTouchInRect v rect
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

extractFirstTouchInRect: JD.Value -> SvgThings.Rect -> Maybe Touch
extractFirstTouchInRect evt rect = 
  let touches = extractTouches evt in
    List.head (List.filter (\touch -> 
      SvgThings.containsXY rect (truncate touch.x) (truncate touch.y))
                 touches) 

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


