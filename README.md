# Build Your Own HTTP Server Challenge

[![progress-banner](https://backend.codecrafters.io/progress/http-server/76effb18-7490-4be4-be46-f3e2a01cd92c)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

Welcome to the "Build Your Own HTTP Server" Challenge repository for Rust solutions!

## Overview

This repository serves as a my Rust solutions to the ["Build Your Own HTTP server" Challenge](https://app.codecrafters.io/courses/http-server/overview).

HTTP is the backbone of the web, enabling communication between clients and servers.
In this challenge, you'll embark on an exciting journey to build your very own HTTP/1.1 server in Rust.
By completing this challenge, you'll gain valuable insights into TCP servers, HTTP request syntax, and more.

>Each commit that I put here corresponds to each step of the challenge.
With each step, you'll gradually enhance your server's capabilities and gain a deeper understanding of how web servers operate under the hood.

Note: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try this challenge.

## Structure of project:
```bash
src
├── error.rs
├── lib.rs
├── main.rs
├── request.rs
├── response.rs
├── routes
│  ├── echo.rs
│  ├── files.rs
│  ├── mod.rs
│  ├── root.rs
│  └── user_agent.rs
└── server.rs
tests
├── echo.rs
├── files.rs
├── root.rs
├── support
│  ├── mod.rs
│  └── setup.rs
└── user_agent.rs
```