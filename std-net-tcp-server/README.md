# Rust TCP Echo Server Example

A minimal, multi-threaded TCP server implemented in Rust.  
It accepts client connections on `127.0.0.1:7878`, prints received data, and responds with a greeting.

## Features

- Pure Rust, using standard library only (`std::net`, `std::thread`)
- Handles multiple clients concurrently using threads
- Echoes a greeting message to each client
- Simple and self-contained — great for learning and demos

## Usage

### 1. 启动服务器（终端一）

在项目根目录下运行：

```bash
cargo run -p std-net-tcp-server
```

你会看到类似的输出：

```
服务器启动，监听 127.0.0.1:7878
有新客户端连入: 127.0.0.1:50760
```

### 2. 连接服务器（终端二）

打开另一个终端，输入：

```bash
nc 127.0.0.1 7878
```

输入些内容（例如 `hello`），回车。

你会收到服务端响应：

```
Hello from rust server!
```

此时，服务端终端也会输出你输入的数据。例如：

```
有新客户端连入: 127.0.0.1:50760
收到客户端数据: hello
```

## Code Overview

- Uses [`TcpListener`](https://doc.rust-lang.org/std/net/struct.TcpListener.html) to listen for connections.
- Spawns a thread for each incoming connection to handle clients concurrently.
- Demonstrates basic TCP stream reading and writing.

## Requirements

- Rust (edition 2021 or later)
- `nc` (netcat) 客户端，主流类 Unix 系统常自带

## License

MIT

---

Inspired by the official [Rust documentation examples](https://doc.rust-lang.org/std/net/struct.TcpListener.html).