# oscpad
Configurable web controls with shared state, which send OSC messages.  

This is my 'getting to know you' project for elm and rust.  

The idea is to have a simple way to configure a set of touch enabled buttons, sliders, and labels, which are presented on a web page.  When the user manipulates the controls, websocket messages are sent to the server, which in turn sends out OSC messages to one or more target IP addresses.  Incoming OSC messages can also change the control state, which is reflected in the clients.  OSC is an easy protocol to support, good for 'internet of things' activities, such as robots, lamps, electronic instruments, etc.

Currently the button and slider controls are functional, but they aren't touch enabled yet.  The control configuration is read from json and sent up to the elm client, which draws the controls.  Shared state is working now.  Next up is sending and receiving OSC messages.  

Once the basic controls are working I'll put some nifty examples up, like a control config that makes control configs for instance.  Meta!  

Ultimately I'll probably break out the control server into a library separate from oscpad itself.  For now everything is smushed together into a single project.  

### Some notes on elm compiling.

The 'official' elm websockets library is actually a socketio library which is not supported on the rust side. So, this project depends on this somewhat fly-by-night elm websockets library:

https://github.com/bburdette/testing-elm-websockets

Also there's this thing where I need the window size on program start.  That capability comes from this pull request, which I just monkey-patched in to my elm-stuff folder.  

https://github.com/evancz/start-app/pull/37

