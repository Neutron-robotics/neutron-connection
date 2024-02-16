# neutron-connection

Blazing fast and highly scalable server component written in Rust for handling interaction from a robotic system to one or multiple clients with a low memory footprint.

This software proxies data from the robot to connected clients in order to handle a huge amount of clients. Currently, only the WebSocket protocol is supported. There is an additional protocol layer for authorizing connections, listing connected clients, and retrieving robotic system health information.

## Features

- Establishes a "neutron connection" between a robotic system and multiple clients.
- Proxies data from the robot to connected clients.
- Supports the WebSocket protocol.
- Implements an additional protocol layer for authorization, listing connected clients, and retrieving robotic system health information.

## Installation

### Prerequisites

- Rust programming language installed. If not, you can download it from [rust-lang.org](https://www.rust-lang.org/).

### Running for development

```sh
cargo run -- --id thisConnectionId --robot-host localhost --robot-port 3000 --application-port 3030
```

### Command-line Arguments

The `neutron-connection` server accepts the following command-line arguments:

- `--id`: The identifier of the connection. This is used to uniquely identify each connection to the server.

- `-c, --robot-host`: The hostname of the robot to be connected to. This specifies the address of the robotic system that the server will connect to.

- `-d, --robot-port`: The port of the robot to be connected to. This specifies the port number on which the robotic system is listening for connections.

- `-p, --application-port`: The port for the application to run on. This specifies the port number on which the Neutron connection server itself will listen for incoming connections from clients.

- `-t, --application-timeout`: (Optional) The timeout (in seconds) before closing the application if no clients are connected. If not specified, the server will continue running indefinitely, waiting for client connections.

- `-r, --redis-connection-string`: (Optional) The optional Redis connection string for creating the database client. This allows the server to connect to a Redis database if needed for additional functionality or data storage.

These arguments can be passed when starting the `neutron-connection` server to configure its behavior and connection parameters.


### Building from Source

1. Clone the repository:

    ```sh
    git clone https://github.com/your_username/neutron-connection.git
    ```

2. Navigate to the project directory:

    ```sh
    cd neutron-connection
    ```

3. Build the project:

    ```sh
    cargo build --release
    ```

## Usage

1. Start the neutron-connection server:

    ```sh
    ./target/release/neutron-connection
    ```

2. Connect your robotic system to the server using the WebSocket protocol.

3. Authorize the connection, list connected clients, and retrieve robotic system health information using the provided additional protocol layer.
