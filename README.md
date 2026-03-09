# Madbus

A Rust library for **Modbus TCP communication**.  
It provides simple APIs for building both **Modbus clients (masters)** and **servers (slaves)** with automatic encoding/decoding of Modbus TCP frames.

---

# Features

- Modbus TCP **Client and Server utilities**
- Automatic **MBAP header handling**
- Encoding/decoding of **Modbus ADU/PDU**
- Built-in **exception handling**
- Helper utilities for **bit packing/unpacking**
- Supports common Modbus function codes:
  - `0x01` Read Coils
  - `0x02` Read Discrete Inputs
  - `0x03` Read Holding Registers
  - `0x04` Read Input Registers
  - `0x05` Write Single Coil
  - `0x06` Write Single Register
  - `0x0F` Write Multiple Coils
  - `0x10` Write Multiple Registers

---

# Project Structure

```
src/
├── main.rs              # Client/Server implementation and ADU handling
├── function_codes.rs    # Commands and responses
├── exception_codes.rs   # Modbus exception definitions
└── helpers.rs           # Utility helpers
```

---

# Usage

## Client Example

Send a request to a Modbus device:

```rust
use madbus::{Client, Command};
use std::net::TcpStream;

let mut stream = TcpStream::connect("127.0.0.1:502").unwrap();

let cmd = Command::ReadHolding {
    start: 0,
    count: 5
};

Client::send_request(&mut stream, cmd, 1).unwrap();
```

Read the response:

```rust
let (response, unit_id, transaction_id) =
    Client::read_response(&mut stream).unwrap();

println!("{:?}", response);
```

---

## Server Example

Reading a request from a client:

```rust
use madbus::Server;

let (command, unit_id, transaction_id) =
    Server::read_request(&mut stream).unwrap();
```

Sending a response:

```rust
use madbus::Response;

let response = Response::read_holding(&[10, 20, 30]);

Server::send_resp(&mut stream, response, unit_id, transaction_id).unwrap();
```

Sending an exception:

```rust
use madbus::Exception;

Server::send_exception(
    &mut stream,
    Exception::IllegalAddress,
    3,
    unit_id,
    transaction_id
).unwrap();
```

---

# Testing

```
cargo test
```

---

# License

MIT