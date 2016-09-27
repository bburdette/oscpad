module SvgTextSize
   exposing ( getTextWidth
    ) 

import Native.SvgTextSize

getTextWidth : String -> String -> Int 
getTextWidth text font = 
  Native.SvgTextSize.getTextWidth text font 


