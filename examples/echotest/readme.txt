this program receives OSC messages from the oscpad server, looks for slider messages from those, and sends back label messages.

The result should be that as you slide the sliders around, label text is updated.  

for example, if oscpad is sending out message on port 7070 and receiving message on 9000, then run echotest like this: 

./target/debug/echotest localhost:7070 localhost:9000
