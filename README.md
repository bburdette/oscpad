# oscpad
Configurable web controls with shared state, which send OSC messages.  [Overview/tutorial here](https://github.com/bburdette/oscpad/wiki/Get-started-with-oscpad)  

[Arm executables here](https://drive.google.com/open?id=0B0C7MeOQIbZkQmJCZE5RSDY5Q2M) Compiled on bananapi arch, hopefully compatible with raspberry pi 2 (armv7).  You'll need to install openssl.

This is my 'getting to know you' project for elm and rust.  

The idea is to have a simple way to configure a set of touch enabled buttons, sliders, and labels, which are presented on a web page.  When the user manipulates the controls, websocket messages are sent to the server, which in turn sends out OSC messages to one or more target IP addresses.  Incoming OSC messages can also change the control state, which is reflected in the clients.  OSC is an easy protocol to support, good for 'internet of things' activities, such as robots, lamps, electronic instruments, etc.

Right now the controls are working for the most part.  There is a label, button, slider, and sizer.  Different web browsers have different behaviors, sometimes gestures interfere with the control operation.  So far the Opera browser is the most responsive to touch events on my nexus tablet.  In firefox it appears to be helpful to access the menu and click the 'request desktop site' option.  

There's  an example project, echotest, which changes numbers in labels based on slider movement.  There's also guisend, which reads a json control configuration file and sends it to oscpad, replacing whatever controls were there before.    

Ultimately I'll probably break out the control server into a library separate from oscpad itself.  For now everything is smushed together into a single project.  

### Some notes on elm compiling.

use elm/build-elm.sh to build the elm into elm.js.  Then open index.html from the web browser.
Inside index.html is the address of the rust server, for now hardcoded to ws://localhost:1234.  

TODO: have the rust server edit index.html and replace the hardcoded address with its actual address.
