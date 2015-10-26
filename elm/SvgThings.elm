module SvgThings where

import List exposing (..)

type Orientation = Vertical | Horizontal

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
     map (mekr rct w) idxs


mekr: Rect -> Int -> Int -> Rect
mekr br w i = Rect (w * i) br.y w br.h 
