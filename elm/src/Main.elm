module Main where

{-
import Effects exposing (Never)
import SpinSquarePair exposing (init, update, view)
import StartApp
import Task exposing (Task)
import WebSocket exposing (WebSocket)
import Keyboard

socket : Task x WebSocket
socket = WebSocket.create "ws://localhost:1234"

listen : Signal.Mailbox String
listen = Signal.mailbox ""

inputKeyboard : Signal String
inputKeyboard = Signal.map (\c -> toString c) Keyboard.presses

app =
  StartApp.start
    { init = init
    , update = update
    , view = view
    , inputs = []
    }

main =
  app.html


port tasks : Signal (Task.Task Never ())
port tasks =
  app.tasks

-}

import Graphics.Collage exposing (..)
import Graphics.Element exposing (..)
import Signal exposing (Signal)
import Task exposing (Task)
import Keyboard
import Char
import String
import WebSocket exposing (WebSocket)

socket : Task x WebSocket
socket = WebSocket.create "ws://localhost:1234"

listen : Signal.Mailbox String
listen = Signal.mailbox ""

--main = Signal.map2 (\a b -> show ("Sending: " ++ a ++ ", Receiving: " ++ b))
--         inputKeyboard listen.signal
main = Signal.map3 (\a b c -> show ("Sending: " ++ a ++ ", Receiving: " ++ b ++ ", Connected: " ++ c))
         inputKeyboard listen.signal (Signal.map toString connected.signal)

port listening : Task x ()
port listening = socket `Task.andThen` WebSocket.listen listen.address

connected : Signal.Mailbox Bool
connected = Signal.mailbox False

port connection : Task x ()
port connection = socket `Task.andThen` WebSocket.connected connected.address

send : String -> Task x ()
send message = socket `Task.andThen` WebSocket.send message

port sending : Signal (Task x ())
port sending = Signal.map send inputKeyboard

inputKeyboard : Signal String
-- inputKeyboard = Signal.map (\c -> Char.fromCode c |> String.fromChar) Keyboard.presses
inputKeyboard = Signal.map (\c -> toString c) Keyboard.presses
