module Main where

import Effects exposing (Never)
import SvgButton 
import SvgControls
import Controls
import StartApp
import Task
import Signal exposing (Signal)
import Task exposing (Task)
import Keyboard
import Char
import String
import WebSocket exposing (WebSocket)
import SvgThings
import Window
import Touch

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

send : String -> Task x ()
send message = socket `Task.andThen` WebSocket.send message

port sending : Signal (Task x ())
port sending = Signal.map send inputKeyboard

inputKeyboard : Signal String
inputKeyboard = Signal.map (\c -> toString c) Keyboard.presses

---------------------------------------

app =
  StartApp.start
    { init = SvgControls.init send 
        (SvgThings.Rect 0 0 500 300)    
        (SvgControls.Spec "mehtitle" (Controls.CsButton (SvgButton.Spec "blah")))
    , update = SvgControls.update
    , view = SvgControls.view
    , inits = [ (Signal.map SvgControls.WinDims Window.dimensions)
              ]
    , inputs = [ (Signal.map SvgControls.JsonMsg listen.signal)
               , (Signal.map SvgControls.WinDims Window.dimensions)
               , (Signal.map SvgControls.Touche Touch.touches)
               ]
    }

main =
  app.html

port tasks : Signal (Task.Task Never ())
port tasks =
  app.tasks


