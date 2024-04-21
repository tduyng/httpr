# Build Your Own HTTP Server Challenge

[![progress-banner](https://backend.codecrafters.io/progress/http-server/76effb18-7490-4be4-be46-f3e2a01cd92c)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

Welcome to the "Build Your Own HTTP Server" Challenge repository for Rust solutions!

## Overview

This repository serves as a my Rust solutions to the ["Build Your Own HTTP server" Challenge](https://app.codecrafters.io/courses/http-server/overview).

HTTP is the backbone of the web, enabling communication between clients and servers.
In this challenge, you'll embark on an exciting journey to build your very own HTTP/1.1 server in Rust.
By completing this challenge, you'll gain valuable insights into TCP servers, HTTP request syntax, and more.

>Each first commit that I put here corresponds to each step of the challenge. However, afterwards, I tried to approach the challenge in different ways.

With each step, you'll gradually enhance your server's capabilities and gain a deeper understanding of how web servers operate under the hood.

Here is different stages of this challenge.

## Stages of challenges

<details>
<summary>1. Response with 200</summary>
In this stage, you'll respond to a HTTP request with a 200 OK response.

Your program will need to:

Accept a TCP connection
Read data from the connection (we'll get to parsing it in later stages)
Respond with `HTTP/1.1 200 OK\r\n\r\n` (there are two `\r\ns` at the end)
`HTTP/1.1 200 OK` is the HTTP Status Line.
`\r\n`, also known as CRLF, is the end-of-line marker that HTTP uses.
The first `\r\n` signifies the end of the status line.
The second `\r\n` signifies the end of the response headers section (which is empty in this case).
It's okay to ignore the data received from the connection for now. We'll get to parsing it in later stages.

For more details on the structure of a HTTP response, view the MDN docs.
</details>


<details>
<summary>2. Respond with 404</summary>
In this stage, your program will need to extract the path from the HTTP request.

Here's what the contents of a HTTP request look like:
```
GET /index.html HTTP/1.1
Host: localhost:4221
User-Agent: curl/7.64.1
```
GET `/index.html` HTTP/1.1 is the start line.
`GET` is the HTTP method.
`/index.html` is the path.
`HTTP/1.1` is the HTTP version.
`Host: localhost:4221` and `User-Agent: curl/7.64.1` are HTTP headers.
Note that all of these lines are separated by `\r\n`, not just `\n`.
In this stage, we'll only focus on extracting the path from the request.

If the path is /, you'll need to respond with a 200 OK response. Otherwise, you'll need to respond with a 404 Not Found response.
</details>


<details>
<summary>3. Respond with content</summary>
In this stage, your program will need to respond with a body. In the previous stages we were only sending a status code, no body.

The task here is to parse the path from the HTTP request. We will send a random string in the url path you will need to parse that string and then respond with the parsed string (only) in the response body.

The tester will send you a request of the form `GET /echo/<a-random-string>`.

Your program will need to respond with a 200 OK response. The response should have a content type of text/plain, and it should contain the random string as the body.

As an example, here's a request you might receive:
```
GET /echo/abc HTTP/1.1
Host: localhost:4221
User-Agent: curl/7.64.1
```
And here's the response you're expected to send back:
```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 3
```

abc
Remember, lines in the response are separated by `\r\n`, not just `\n`.

For more details on the structure of a HTTP response, view the MDN docs.
</details>

<details>
<summary>4. Parse headers</summary>
In this stage, your program will need to parse HTTP request headers.

The tester will send you a request of the form `GET /user-agent`, and it'll include a User-Agent header.

Your program will need to respond with a 200 OK response. The response should have a content type of text/plain, and it should contain the user agent value as the body.

For example, here's a request you might receive:
```
GET /user-agent HTTP/1.1
Host: localhost:4221
User-Agent: curl/7.64.1
```
and here's the response you're expected to send back:

```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 11

curl/7.64.1
```
</details>

<details>
<summary>5. Concurrent connections</summary>
Up until now, we've only tested your program against a single connection in each stage.

In this stage, your server will need to handle multiple concurrent connections.

The tester will send you multiple requests at the same time. Your server will need to respond to all of them.
</details>


<details>
<summary>6. Get a file</summary>
In this stage, your server will need to return the contents of a file.

The tester will execute your program with a `--directory` flag like this:
```
./your_server.sh --directory <directory>
```
It'll then send you a request of the form `GET /files/<filename>`.

If <filename> exists in <directory>, you'll need to respond with a 200 OK response. The response should have a content type of `application/octet-stream`, and it should contain the contents of the file as the body.

If the file doesn't exist, return a 404.

We pass in absolute path to your program using the `--directory` flag.
</details>

<details>
<summary>7. Post a file</summary>
In this stage, your server will need to accept the contents of a file in a POST request and save it to a directory.

Just like in the previous stage, the tester will execute your program with a `--directory` flag like this:
```
./your_server.sh --directory <directory>
```
It'll then send you a request of the form `POST /files/<filename>`. The request body will contain the contents of the file.

You'll need to fetch the contents of the file from the request body and save it to `<directory>/<filename>`. The response code returned should be 201.

We pass in absolute path to your program using the `--directory` flag.
</details>