module Main exposing (..) 

import SvgButton 
import SvgSlider 
import SvgControlPage
import SvgControl 
import Task
import Task exposing (Task)
-- import Keyboard
import Char
import String
import WebSocket 
import SvgThings
import Window
-- import SvgTouch
import SvgTextSize
import Html
import Html.App as App

---------------------------------------

wsUrl : String
wsUrl = "ws://localhost:1234"

type Msg 
  = Receive String
  | Send

main =
  App.program
    { init = SvgControlPage.init 
        wsUrl
        (SvgThings.Rect 0 0 500 300)    
        (SvgControlPage.Spec "mehtitle" (SvgControl.CsSlider (SvgSlider.Spec "blah" SvgThings.Vertical)) Nothing)
        -- (SvgControlPage.Spec "mehtitle" (SvgControl.CsButton (SvgButton.Spec "blah" Nothing)) Nothing)
    , update = SvgControlPage.update
    , view = SvgControlPage.view
    , subscriptions = \_ -> WebSocket.listen wsUrl SvgControlPage.JsonMsg
--    , subscriptions = \_ -> Sub.none
--    , inits = [ (Signal.map SvgControlPage.WinDims Window.dimensions)
--              ]
--    , inputs = [ (Signal.map SvgControlPage.JsonMsg listen.signal)
--               , (Signal.map SvgControlPage.WinDims Window.dimensions)
--               , (Signal.map SvgControlPage.Touche SvgTouch.touches)
--               ]
    }


