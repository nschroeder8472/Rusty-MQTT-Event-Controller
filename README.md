Rusty Mqtt Event Controller

App written to read specific events from MQTT topics and publish events in response.
These publish events are configured through a lua script for easy maintainability and complex functionality without needing
to recompile the core rust program. To add functionality, simply add an entry to the config, and restart the app.

Primarily written for 2 reasons:
1. ZwaveJS2MQTT will publish scene events from switches but will not act on those events.
2. Homebridge plugin MQTTThing does not have a way (or an easy way) to listen to these scenes and control other devices

FAQ:
1. Why not use Home Assistant? \
   Ans: Yes. I know that Home Assistant can do this, but I am not currently using Home Assistant because I am too lazy
to migrate from Homebridge but not too lazy to write a small rust program.
2. What is the easiest way to run the program? \
   Ans: Docker. Docker allows you to have a generic config and run on *any* OS so recompiling is not required.
3. Can I ask for X feature? \
   Ans: Asking is free, but I may or may not respond. Feel free to contribute yourself and make a PR.
4. Don't you know that X exists or can't you do Y, or whatever question you might have as to why I wrote this.
   Ans: I'm sure there is a better way but that's not the point of this. This works for me, and helped me learn. So this meets my requirements.