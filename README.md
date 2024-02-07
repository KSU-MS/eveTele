# Scope changes
Oh boy, so in short this will no longer act as an all-in-one DAQ tool, why? Because there is no need for me to re-invent that wheel, instead what this app will do is act as a bridge to the foxglove websocket and save data as MCAP files

# Timeline
It can read a serial port and parse it, and thats about it, thankfully not needing to do any processing in this app will make the last bit of this stupid easy

# Ignorance
The primary thing this project can be defined by, speaking of which who knew its kinda hard to make something that has to constantly serialize and de-serialize data into a bunch of wacky formats would be kinda hard...

N e ways, it's closer than before, last few steps is auto-gen protobuf file, then write to mcap, then publish to foxglove. But that is 90% of what this tool was supposed to do so we shall see if it will really be that shrimple.
