module SvgTextSize
    ( getTextWidth
    ) where

import Native.SvgTextSize
import Time exposing (Time)
import Signal exposing (Signal)
import Task exposing (..)

getTextWidth : String -> String -> Int 
getTextWidth text font = 
  Native.SvgTextSize.getTextWidth text font 


