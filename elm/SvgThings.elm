module SvgThings where

import List exposing (..)
import Json.Decode as JD exposing ((:=))
import Json.Encode as JE

type Orientation = Vertical | Horizontal

jsOrientation : String -> JD.Decoder Orientation 
jsOrientation o = 
  case o of 
    "vertical" -> JD.succeed Vertical
    "horizontal" -> JD.succeed Horizontal

type alias ControlId = List Int

encodeControlId: ControlId -> JD.Value
encodeControlId cid = 
  JE.list (List.map JE.int cid)

decodeControlId: JD.Decoder (List Int) 
decodeControlId = JD.list JD.int

type alias Rect = 
  { x: Int  
  , y: Int
  , w: Int
  , h: Int
  }

type alias SRect = 
  { x: String  
  , y: String
  , w: String
  , h: String
  }

toSRect: Rect -> SRect
toSRect rect = SRect 
  (toString rect.x)
  (toString rect.y)
  (toString rect.w)
  (toString rect.h)

-- make a number of horizontally evenly spaced rects.
hrects: Rect -> Int -> List Rect
hrects rct count = 
  let w: Int
      w = round (toFloat rct.w / count)
      idxs = [0..(count-1)]
   in
     map (mekhr rct w) idxs


mekhr: Rect -> Int -> Int -> Rect
mekhr br w i = Rect (w * i) br.y w br.h 

-- make a number of vertically evenly spaced rects.
vrects: Rect -> Int -> List Rect
vrects rct count = 
  let h: Int
      h = round (toFloat rct.h / count)
      idxs = [0..(count-1)]
   in
     map (mekvr rct h) idxs


mekvr: Rect -> Int -> Int -> Rect
mekvr br h i = Rect br.x (h * i) br.w h 
