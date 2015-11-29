module Main where

import Effects exposing (Never)
import SvgButton 
import SvgLabel
import SvgSlider
import SvgControlPage
import SvgControl 
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
    { init = SvgControlPage.init send 
        (SvgThings.Rect 0 0 500 300)    
        (SvgControlPage.Spec 
          "mehtitle" 
          (SvgControl.CsSizer 
            (SvgControl.SzSpec 
              "" 
              SvgThings.Vertical
              [(SvgControl.CsLabel (SvgLabel.Spec "test" "test")) 
              ,(SvgControl.CsButton (SvgButton.Spec "blah"))
              ,(SvgControl.CsSlider (SvgSlider.Spec "sl1" SvgThings.Horizontal))
              ,(SvgControl.CsSlider (SvgSlider.Spec "sl2" SvgThings.Horizontal))
              ,(SvgControl.CsSlider (SvgSlider.Spec "sl3" SvgThings.Horizontal))
              ,(SvgControl.CsSlider (SvgSlider.Spec "sl4" SvgThings.Horizontal))
              ,(SvgControl.CsSlider (SvgSlider.Spec "sl5" SvgThings.Horizontal))
              ,(SvgControl.CsSlider (SvgSlider.Spec "sl6" SvgThings.Horizontal))
              ,(SvgControl.CsSlider (SvgSlider.Spec "sl7" SvgThings.Horizontal))
              ]
            )
          )
          Nothing)
    , update = SvgControlPage.update
    , view = SvgControlPage.view
    , inits = [ (Signal.map SvgControlPage.WinDims Window.dimensions)
              ]
    , inputs = [ (Signal.map SvgControlPage.JsonMsg listen.signal)
               , (Signal.map SvgControlPage.WinDims Window.dimensions)
               , (Signal.map SvgControlPage.Touche Touch.touches)
               ]
    }

main =
  app.html

port tasks : Signal (Task.Task Never ())
port tasks =
  app.tasks


