module Main exposing (..) 

-- import Effects exposing (Never)
import SvgButton 
import SvgControlPage
import SvgControl 
-- import StartApp
import Task
-- import Signal exposing (Signal)
import Task exposing (Task)
-- import Keyboard
import Char
import String
import WebSocket exposing (WebSocket)
import SvgThings
import Window
-- import SvgTouch
import SvgTextSize
import Html

---------------------------------------

socket : Task x WebSocket
socket = WebSocket.createToHost "1234"

listen : Signal.Mailbox String
listen = Signal.mailbox ""

port listening : Task x (List ())
port listening = socket `Task.andThen` 
  (\s -> 
    Task.sequence [WebSocket.listen listen.address s, 
                   WebSocket.connected connected.address s])

connected : Signal.Mailbox Bool
connected = Signal.mailbox False

-- send : String -> Task x ()
-- send message = socket `Task.andThen` WebSocket.send message

port sending : Signal (Task x ())
port sending = Signal.map send inputKeyboard

-- not really used yet!  but hopefully will be soon, for a 
-- text entry control or other keyboard oriented controls.  
inputKeyboard : Signal String
inputKeyboard = Signal.map (\c -> toString c) Keyboard.presses

---------------------------------------

{ init = init, update = update, view = view, subscriptions = \_ -> Sub.none }

app =
  Html.program
    { init = SvgControlPage.init 
        (send "todo:www.wheretosend.com")
        (SvgThings.Rect 0 0 500 300)    
        (SvgControlPage.Spec "mehtitle" (SvgControl.CsButton (SvgButton.Spec "blah" Nothing)) Nothing)
    , update = SvgControlPage.update
    , view = SvgControlPage.view
    , subscriptions = \_ -> Sub.none
--    , inits = [ (Signal.map SvgControlPage.WinDims Window.dimensions)
--              ]
--    , inputs = [ (Signal.map SvgControlPage.JsonMsg listen.signal)
--               , (Signal.map SvgControlPage.WinDims Window.dimensions)
--               , (Signal.map SvgControlPage.Touche SvgTouch.touches)
--               ]
    }

main =
  app.html

port tasks : Signal (Task.Task Never ())
port tasks =
  app.tasks


