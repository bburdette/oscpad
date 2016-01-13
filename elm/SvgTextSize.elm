module SvgTextSize
    ( TextSizeRequest, TextBounds, 
      getTextSize, getTb, getTInt, getCurrentTime, getTextWidth, getTextWidthNow
    ) where

import Native.SvgTextSize
import Time exposing (Time)
import Signal exposing (Signal)
import Task exposing (..)

type alias TextBounds =
    { w : Int
    , h : Int
    }

type alias TextSizeRequest = 
  { text : String
  , font : String 
  }

-- getTextWidth <string> <font>  
{- 
getTextWidth : String -> String -> Task x TextBounds 
getTextWidth string font = 
  Native.SvgTextSize.getTextWidth string font

getTextWidth : TextSizeRequest -> Task x Int 
getTextWidth = 
  Native.SvgTextSize.getTextWidth

-}

getTextWidthNow : String -> String -> Int 
getTextWidthNow text font = 
  Native.SvgTextSize.getTextWidthNow text font 

getTextWidth : String -> String -> Task x Int 
getTextWidth text font = 
  Native.SvgTextSize.getTextWidth text font 

getTextSize : TextSizeRequest -> Task x TextBounds
getTextSize = 
  Native.SvgTextSize.getTextSize

getTb : Task x TextBounds
getTb = 
  Native.SvgTextSize.getTb

getTInt : Task x Int
getTInt = 
  Native.SvgTextSize.getTInt

getCurrentTime : Task x Time
getCurrentTime =
  Native.SvgTextSize.getCurrentTime
