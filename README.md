# Build HTTP server from scratch in Rust

## Overview

HTTP is the backbone of the web, facilitating communication between clients and servers. Building an HTTP server from scratch in Rust is not only a fantastic learning experience but also an opportunity to delve deep into the intricacies of web protocols and server-side programming.

This project aims to guide you through the process of creating a simple yet robust HTTP server in Rust, empowering you to understand the inner workings of web servers and gain hands-on experience with Rust's powerful features.

## Getting Started

### Prerequisites

Before getting started, ensure that you have [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/stable/cargo/) installed on your system.

### Running the Project

1. Clone this repository to your local machine:

```bash
git clone https://github.com/tduyng/rhttp.git && cd rhttp
```

2. Run server

```bash
cargo run --bin server
```

## Testing benchmarks

To evaluate the performance of your HTTP server, you can use the `wrk` tool, which is a modern HTTP benchmarking tool capable of generating significant load.

Install `wrk` using your system's package manager or by following the instructions on the official GitHub repository: `wrk`.

Example command to test your server with wrk:

```bash
wrk -t6 -c200 -d5s http://localhost:2024
```

Adjust the parameters (-t, -c, -d) according to your testing requirements.

## Features

- Simple implementation: The HTTP server is implemented from scratch, providing a clear understanding of its internal workings.
- Concurrency: Use tokio async to handle multiple client connections efficiently.
- Extensibility: Designed to be easily extensible, allowing you to add custom features and middleware.

## License

This project is licensed under the terms of the MIT license. See the [LICENSE](./LICENCE) file for details.

Feel free to customize and expand upon this README to suit your project's specific features, goals, and audience. Good luck with your HTTP server project! 🚀
