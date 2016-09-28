# oscpad
Configurable web controls with shared state, which send OSC messages.  [Overview/tutorial here](https://github.com/bburdette/oscpad/wiki/Get-started-with-oscpad)  

This is my 'getting to know you' project for elm and rust.  

The idea is to have a simple way to configure a set of touch enabled buttons, sliders, and labels, which are presented on a web page.  When the user manipulates the controls, websocket messages are sent to the server, which in turn sends out OSC messages to one or more target IP addresses.  Incoming OSC messages can also change the control state, which is reflected in the clients.  OSC is an easy protocol to support, good for 'internet of things' activities, such as robots, lamps, electronic instruments, etc.

Right now the controls are working for the most part.  There is a label, button, slider, and sizer.  There's  an example project, echotest, which changes numbers in labels based on slider movement.  There's also guisend, which reads a json control configuration file and sends it to oscpad, replacing whatever controls were there before.    

Ultimately I'll make oscpad into a rust library as well as an application, for single-executable projects. 

### Some notes on elm compiling.

From the project directory, use ./build-elm.sh to build the elm and merge the js into stringDefaults.rs.  Then do cargo build to get the rust server.  Run the rust server with ./runit.sh.  
