# Rusty MQTT Event Controller

App written to read specific events from MQTT topics and publish events in response.
These publish events are configured through Lua scripts for easy maintainability and complex functionality without needing
to recompile the core Rust program. To add functionality, simply add an entry to the config, and restart the app.

## Primarily written for 2 reasons:
1. ZwaveJS2MQTT will publish scene events from switches but will not act on those events.
2. Homebridge plugin MQTTThing does not have a way (or an easy way) to listen to these scenes and control other devices

---

## Table of Contents
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Writing Lua Scripts](#writing-lua-scripts)
- [Docker Deployment](#docker-deployment)
- [FAQ](#faq)

---

## Quick Start

1. Create a `config.json` file (see [Configuration](#configuration))
2. Write Lua scripts for your event handlers (see [Writing Lua Scripts](#writing-lua-scripts))
3. Run with Docker or compile locally

---

## Configuration

Create a `config.json` file with the following structure:

```json
{
  "client_id": "rusty-event-controller",
  "host": "localhost",
  "port": 1883,
  "script_dir": "/data/lua_scripts",
  "sub_events": [
    {
      "qos": 2,
      "sub_topic": "home/sensor/motion",
      "lua_script_file": "motion_handler.lua"
    },
    {
      "qos": 1,
      "sub_topic": "home/switch/scene",
      "lua_script_file": "scene_handler.lua"
    }
  ]
}
```

### Configuration Fields

| Field | Type | Description |
|-------|------|-------------|
| `client_id` | String | Unique identifier for this MQTT client |
| `host` | String | MQTT broker hostname or IP address |
| `port` | Number | MQTT broker port (typically 1883) |
| `script_dir` | String | Directory containing Lua script files |
| `sub_events` | Array | List of topic subscriptions and their handlers |

### Sub Event Fields

| Field | Type | Description |
|-------|------|-------------|
| `qos` | Number | Quality of Service: 0 (at most once), 1 (at least once), 2 (exactly once) |
| `sub_topic` | String | MQTT topic to subscribe to (supports wildcards like `home/+/sensor`) |
| `lua_script_file` | String | Lua script filename relative to `script_dir` |

---

## Writing Lua Scripts

### **IMPORTANT: Required Convention**

**Every Lua script MUST define a function named `myFunction`.** This is a required convention enforced by the application.

### Function Signature

```lua
function myFunction(message)
    -- Your logic here
    return true  -- or false
end
```

**Parameters:**
- `message` (string): The MQTT message payload received on the subscribed topic

**Return Value:**
- `true`: Script executed successfully (logs success message)
- `false`: Silent execution (no log output)

### Available Lua Functions

Your Lua scripts have access to the following functions:

#### `publish_message(topic, payload)`

Publishes an MQTT message to the specified topic.

**Parameters:**
- `topic` (string): The MQTT topic to publish to
- `payload` (string): The message payload to publish

**Example:**
```lua
publish_message("home/light/living-room", "ON")
```

**Note:** This function uses QoS 2 (exactly once delivery) and publishes messages asynchronously.

---

## Lua Script Examples

### Example 1: Simple Toggle

```lua
function myFunction(message)
    if message == "1" then
        publish_message("home/light/bedroom", "ON")
        return true
    elseif message == "0" then
        publish_message("home/light/bedroom", "OFF")
        return true
    end
    return false  -- Unknown message, no action taken
end
```

### Example 2: Scene Controller

```lua
function myFunction(message)
    -- Parse JSON or handle scene IDs
    local scene_id = tonumber(message)

    if scene_id == 1 then
        -- "Good Morning" scene
        publish_message("home/light/bedroom", "ON")
        publish_message("home/thermostat/set", "72")
        return true
    elseif scene_id == 2 then
        -- "Good Night" scene
        publish_message("home/light/all", "OFF")
        publish_message("home/lock/front-door", "LOCK")
        return true
    end

    return false
end
```

### Example 3: Conditional Logic

```lua
function myFunction(message)
    -- Handle motion sensor events
    if message == "motion_detected" then
        -- Only turn on lights between sunset and sunrise
        local hour = tonumber(os.date("%H"))

        if hour >= 19 or hour <= 6 then
            publish_message("home/light/hallway", "ON")
            return true
        end
    end

    return false
end
```

### Example 4: Multiple Actions

```lua
function myFunction(message)
    local button_press = tonumber(message)

    if button_press == 1 then
        -- Single press: Toggle main light
        publish_message("home/light/living-room", "TOGGLE")
        return true
    elseif button_press == 2 then
        -- Double press: Turn on all lights
        publish_message("home/light/living-room", "ON")
        publish_message("home/light/kitchen", "ON")
        publish_message("home/light/hallway", "ON")
        return true
    elseif button_press == 3 then
        -- Triple press: Turn off everything
        publish_message("home/light/all", "OFF")
        return true
    end

    return false
end
```

---

## Docker Deployment

### Using Docker Compose

```yaml
version: '3.8'
services:
  rusty-mqtt-controller:
    image: rusty-mqtt-controller:latest
    volumes:
      - ./config.json:/data/config.json
      - ./lua_scripts:/data/lua_scripts
    restart: unless-stopped
    environment:
      - RUST_BACKTRACE=full
```

### Building the Docker Image

```bash
docker build -t rusty-mqtt-controller .
```

### Running the Container

```bash
docker run -d \
  --name mqtt-controller \
  -v $(pwd)/config.json:/data/config.json \
  -v $(pwd)/lua_scripts:/data/lua_scripts \
  rusty-mqtt-controller:latest
```

### Viewing Logs

```bash
docker logs -f mqtt-controller
```

---

## Troubleshooting

### Script Not Executing

**Problem:** Events arrive but nothing happens

**Solutions:**
1. Verify your Lua script defines `myFunction` (case-sensitive)
2. Check that the script file path in `config.json` is correct
3. Ensure the script returns `true` to see success logs
4. Check Docker logs for Lua syntax errors

### MQTT Connection Issues

**Problem:** Application can't connect to broker

**Solutions:**
1. Verify `host` and `port` in `config.json`
2. Ensure MQTT broker is running and accessible
3. Check network connectivity in Docker (use `--network host` for testing)
4. Verify MQTT broker doesn't require authentication (not currently supported)

### Missing Script Files

**Problem:** "Failed to read Lua script" error

**Solutions:**
1. Check that `script_dir` path is correct
2. Verify Lua script files exist at the specified paths
3. In Docker, ensure volumes are mounted correctly
4. Check file permissions (must be readable)

---

## FAQ

### 1. Why not use Home Assistant?
**Answer:** Yes, I know that Home Assistant can do this, but I am not currently using Home Assistant because I am too lazy to migrate from Homebridge but not too lazy to write a small Rust program.

### 2. What is the easiest way to run the program?
**Answer:** Docker. Docker allows you to have a generic config and run on any OS so recompiling is not required.

### 3. Can I ask for X feature?
**Answer:** Asking is free, but I may or may not respond. Feel free to contribute yourself and make a PR.

### 4. Why did you write this when X exists?
**Answer:** I'm sure there is a better way but that's not the point of this. This works for me, and helped me learn. So this meets my requirements.

### 5. Can I use multiple MQTT brokers?
**Answer:** Not currently. The application connects to a single MQTT broker defined in the config.

### 6. Why must the function be named `myFunction`?
**Answer:** This is a convention enforced by the application. All Lua scripts are loaded into a shared Lua runtime, and the application looks for `myFunction` in the global namespace for each script.

### 7. Can Lua scripts share state or communicate?
**Answer:** Yes, technically, since all scripts share the same Lua runtime. However, this is not recommended as it can lead to unexpected behavior. Each script should be independent.

### 8. Does it support MQTT authentication?
**Answer:** Not currently. This is planned for a future release.

---

## License

MIT License