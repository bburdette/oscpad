# oscpad
Configurable web controls with shared state, which send OSC messages.  

This is my 'getting to know you' project for elm and rust.  

The idea is to have a simple way to configure a set of touch enabled buttons, sliders, and labels, which are presented on a web page.  When the user manipulates the controls, websocket messages are sent to the server, which in turn sends out OSC messages to one or more target IP addresses.  Incoming OSC messages can also change the control state, which is reflected in the clients.  OSC is an easy protocol to support, good for 'internet of things' activities, such as robots, lamps, electronic instruments, etc.

Basically the controls and etc are working now.  Pressing a button or a slider results in an outgoing OSC message on the server, and updates the state of any other clients connected to the same server.  Incoming OSC messages update the control state on the server and send updates to all clients.  Controls are now touch enabled, although there's still a bit to be done there.

Next up: a label control, uploading the current state to newly connected clients.  Also code cleanup, sending to multiple OSC ip addresses, accepting a new control configuration as an OSC message.  Also hopefully some cool examples.  

Ultimately I'll probably break out the control server into a library separate from oscpad itself.  For now everything is smushed together into a single project.  

### Some notes on elm compiling.

I've already compiled the elm to elm/Main.html, so an elm compiler isn't needed just to run oscpad.  But if you want to build it yourself (I used elm 0.16) there are a few items to note.

First, the 'official' elm websockets library is actually a socketio library which is not supported on the rust side. So, this project depends on this somewhat fly-by-night elm websockets library:

https://github.com/bburdette/testing-elm-websockets

Also there's this thing where I need the window size on program start.  That capability comes from this pull request, which I just monkey-patched in to my elm-stuff folder.  If this or something like it is merged into startapp trunk I'll convert to that.

https://github.com/evancz/start-app/pull/37

