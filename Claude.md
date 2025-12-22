# Rusty MQTT Event Controller

## Project Overview

This is a Rust-based MQTT event controller that subscribes to MQTT topics, processes incoming events, and publishes responses based on Lua script configurations. The project was designed to bridge the gap between ZwaveJS2MQTT scene events and Homebridge MQTTThing plugin.

## Purpose

**Primary Use Cases:**
1. ZwaveJS2MQTT publishes scene events from switches but doesn't act on those events
2. Homebridge plugin MQTTThing lacks easy ways to listen to these scenes and control other devices

**Design Philosophy:**
- Configuration via Lua scripts (no recompilation needed for functionality changes)
- Docker-friendly for cross-platform deployment
- Lightweight alternative to full home automation systems like Home Assistant

## Technology Stack

- **Language:** Rust (Edition 2024)
- **MQTT Client:** rumqttc (v0.24.0)
- **Async Runtime:** Tokio (v1.47.1) with full features
- **Scripting:** Lua 5.4 via mlua (v0.11.2)
- **Serialization:** serde + serde_json
- **CLI:** clap (v4.5.32)
- **Containerization:** Docker

## Architecture

### Core Components

1. **MQTT Client** (`src/mqtt/`)
   - Handles connection to MQTT broker
   - Subscribes to configured topics
   - Manages event loop for incoming messages

2. **Lua Integration** (`src/lua/`)
   - Loads and manages Lua scripts
   - Maps topics to their corresponding Lua script functions
   - Executes Lua handlers asynchronously when events arrive

3. **Configuration** (`src/structs/`)
   - Parses JSON configuration file
   - Defines MQTT broker connection parameters
   - Maps subscription topics to Lua script files

4. **Main Event Loop** (`src/main.rs`)
   - Initializes configuration, MQTT, and Lua subsystems
   - Continuously polls MQTT events
   - Dispatches incoming messages to appropriate Lua handlers

### Data Flow

```
MQTT Broker → Event Loop → Topic Matcher → Lua Script → Response Actions
```

1. MQTT event received on subscribed topic
2. Payload extracted and topic matched against configuration
3. Corresponding Lua script function called with payload
4. Lua script processes event and can publish MQTT responses

## Configuration Structure

**config.json** (default location: `/data/config.json`)

```json
{
  "client_id": "rusty-event-controller",
  "host": "localhost",
  "port": 1883,
  "script_dir": "/sample_files/lua_scripts",
  "sub_events": [{
    "qos": 2,
    "sub_topic": "example/sub/topic",
    "lua_script_file": "example.lua"
  }]
}
```

**Fields:**
- `client_id`: MQTT client identifier
- `host`: MQTT broker hostname/IP
- `port`: MQTT broker port
- `script_dir`: Directory containing Lua scripts
- `sub_events`: Array of subscription configurations
  - `qos`: Quality of Service (0, 1, or 2)
  - `sub_topic`: MQTT topic to subscribe to
  - `lua_script_file`: Lua script file to execute for this topic

## Lua Script Interface

Lua scripts receive the MQTT payload as a parameter and should return a boolean:
- `true`: Script executed successfully
- `false`: Silent failure (no log output)

Scripts have access to MQTT publishing functions to send response events.

## Key Files

- **`src/main.rs`**: Entry point, event loop orchestration
- **`src/mqtt/mqtt_client.rs`**: MQTT connection and subscription setup
- **`src/lua/lua_config.rs`**: Lua runtime initialization and script loading
- **`src/structs/config.rs`**: Configuration parsing and structures
- **`Cargo.toml`**: Rust dependencies and project metadata
- **`Dockerfile`**: Container build configuration
- **`sample_files/config.json`**: Example configuration
- **`sample_files/lua_scripts/`**: Example Lua event handlers

## Development Setup

### Local Development

```bash
# Build the project
cargo build --release

# Run with custom config
cargo run -- --config /path/to/config.json
```

### Docker Deployment

```bash
# Build image
docker build -t rusty-mqtt-controller .

# Run container
docker run -v /path/to/config:/data rusty-mqtt-controller
```

## Adding New Functionality

To add new event handling:

1. Create a new Lua script in your scripts directory
2. Add an entry to `sub_events` in `config.json` with:
   - The MQTT topic to listen to
   - The Lua script filename
   - Desired QoS level
3. Restart the application

**No Rust code changes or recompilation needed!**

## Common Tasks

### Debugging Event Handlers

The application logs when Lua scripts execute successfully. Check logs for:
- "Successfully ran script for topic {topic}" - Normal operation
- "Lua script failed to execute" - Script error
- "Error polling event loop" - MQTT connection issues

### Modifying MQTT Connection

Edit `config.json` to change:
- Broker host/port
- Client ID
- Topic subscriptions

### Testing Lua Scripts

Lua scripts can be tested independently by:
1. Loading them in a Lua interpreter
2. Calling the function with sample JSON payloads
3. Verifying the return value and any MQTT publish calls

## Project Structure

```
.
├── Cargo.toml              # Rust project configuration
├── Dockerfile              # Container build instructions
├── README.md               # User-facing documentation
├── sample_files/           # Example configurations
│   ├── config.json         # Sample config
│   └── lua_scripts/        # Example Lua handlers
└── src/                    # Rust source code
    ├── main.rs             # Application entry point
    ├── mqtt/               # MQTT client module
    ├── lua/                # Lua integration module
    └── structs/            # Data structures and config
```

## Notes for AI Assistants

- This is a personal project built for learning and specific home automation needs
- The author prefers simplicity over feature completeness
- Lua scripting is the primary extension mechanism - avoid suggesting Rust code changes for new event handlers
- Docker is the preferred deployment method
- Configuration changes don't require recompilation
