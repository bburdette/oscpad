module Main where

import Graphics.Collage exposing (..)
import Graphics.Element exposing (..)
import Signal exposing (Signal)
import WebSocket exposing (WebSocket)
import Task exposing (Task)
import Keyboard
import Char
import String

socket : Task x WebSocket
socket = WebSocket.create "ws://localhost:1234"

--main = Signal.map2 (\a b -> show ("Sending: " ++ a ++ ", Receiving: " ++ b))
--         inputKeyboard listen.signal
main = Signal.map3 (\a b c -> show ("Sending: " ++ a ++ ", Receiving: " ++ b ++ ", Connected: " ++ c))
         inputKeyboard listen.signal (Signal.map toString connected.signal)

listen : Signal.Mailbox String
listen = Signal.mailbox ""

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

