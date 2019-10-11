this program sends a gui config file to the oscpad server in an osc message.  

make sure the osc buffer in oscpad is large enough to accomodate the file size, otherwise you'll get an 'invalid osc message' error.

./target/debug/guisend 127.0.0.1:9000 ../samplegui.json
