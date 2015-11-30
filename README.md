# oscpad
Configurable web controls with shared state, which send OSC messages.  [Overview/tutorial here](https://github.com/bburdette/oscpad/wiki/Get-started-with-oscpad!)  

[Arm executable here](https://drive.google.com/file/d/0B0C7MeOQIbZkbGk5WmRkRl9aWlE/view?usp=sharing) Compiled on bananapi arch, hopefully compatible with raspberry pi 2 (armv7).  You'll need to install openssl.

This is my 'getting to know you' project for elm and rust.  

The idea is to have a simple way to configure a set of touch enabled buttons, sliders, and labels, which are presented on a web page.  When the user manipulates the controls, websocket messages are sent to the server, which in turn sends out OSC messages to one or more target IP addresses.  Incoming OSC messages can also change the control state, which is reflected in the clients.  OSC is an easy protocol to support, good for 'internet of things' activities, such as robots, lamps, electronic instruments, etc.

Right now the controls are working for the most part.  There is a label, button, slider, and sizer.  Different web browsers have different behaviors, sometimes gestures interfere with the control operation.  So far the Opera browser is the most responsive to touch events on my nexus tablet.  In firefox it appears to be helpful to access them menu and click the 'request desktop site' option.  

I just added an example project, echotest, which changes numbers in labels based on slider movement.  There's also guisend, which reads a json control configuration file and sends it to oscpad.  

Next up:  mostly everything is working, but the controls aren't as snappy as I'd like.  More importantly, there are issues in some browsers with interference from zooming events and from text selection, where it moves or resizes the display when that isn't desired. Sometimes label text gets selected and after that controls can be selected by mousing over them instead of clicking.  So a few rough edges still.    

Ultimately I'll probably break out the control server into a library separate from oscpad itself.  For now everything is smushed together into a single project.  

### Some notes on elm compiling.

I've already compiled the elm to elm/Main.html, so an elm compiler isn't needed just to run oscpad.  But if you want to build it yourself (as of now you still need 0.15.1) there are a few items to note.

First, the 'official' elm websockets library is actually a socketio library which is not supported on the rust side. So, this project depends on this somewhat fly-by-night elm websockets library:

https://github.com/bburdette/testing-elm-websockets

Also there's this thing where I need the window size on program start.  That capability comes from this pull request, which I just monkey-patched in to my elm-stuff folder.  If this or something like it is merged into startapp trunk I'll convert to that.

https://github.com/evancz/start-app/pull/37

