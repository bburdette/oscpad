# oscpad
Configurable web controls with shared state, which send OSC messages.  

This is my 'getting to know you' project for elm and rust.  

The idea is to have a simple way to configure a set of touch enabled buttons, sliders, and labels, which are presented on a web page.  When the user manipulates the controls, websocket messages are sent to the server, which in turn sends out OSC messages to one or more target IP addresses.  OSC is an easy protocol to support, good for 'internet of things' activities, such as robots, lamps, electronic instruments, etc.

Currently the controls are functional, but they aren't touch enabled yet.  The control configuration is read from json and send up to the elm client, which draws them.  Shared state isn't quite working yet, but its getting close. 

Ultimately I'll probably split this into a rust library for hosting the shared controls and use that library to make the oscpad app.  
