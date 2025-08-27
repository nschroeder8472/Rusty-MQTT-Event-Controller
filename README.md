Rusty Mqtt Event Controller

App written to read specific events from MQTT topics and publish events in response

Primarily written for 2 reasons:
1. ZwaveJS2MQTT will publish scene events from switches but will not act on those events.
2. Homebridge plugin MQTTThing does not have a way (or an easy way) to listen to these scenes and control other devices
3. I want to learn some Rust :)

FAQ:
1. Why not use Home Assistant? \
   Ans: Yes. I know that Home Assistant can do this, but I am not currently using Home Assistant because I am too lazy
to migrate from Homebridge but not too lazy to write a small rust program.
