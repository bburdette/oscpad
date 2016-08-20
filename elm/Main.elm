module Main exposing (..) 

import SvgButton 
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
-- see this:
-- http://guide.elm-lang.org/architecture/effects/web_sockets.html

{-
subscriptions : Model -> Sub Msg
subscriptions model =
  WebSocket.listen "ws://localhost:1234" NewMessage

socket : Task x WebSocket
socket = WebSocket.createToHost "ws1234"

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
-}

---------------------------------------

-- { init = init, update = update, view = view, subscriptions = \_ -> Sub.none }

wsUrl : String
wsUrl = "ws://localhost:3000"

type Msg 
  = Receive String
  | Send

main =
  App.program
    { init = SvgControlPage.init 
        wsUrl
        (SvgThings.Rect 0 0 500 300)    
        (SvgControlPage.Spec "mehtitle" (SvgControl.CsButton (SvgButton.Spec "blah" Nothing)) Nothing)
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

-- main =
--   app.html

-- port tasks : Signal (Task.Task Never ())
-- port tasks =
--  app.tasks


