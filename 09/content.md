# Networking

### CIS 198 Lecture 9

---
## Sockets

- The basic way to send data over the network.
    - Defined by an IP address and a port number.

---
## Socket Programming

- There are many layers of socket-programming providers:
    - Operating system-provided system calls (C's `read`, `write`).
    - Low-level programming language standard library.
    - Higher-level networking libraries or libraries handling a specific
      protocol (e.g. HTTP).

---
## Datagram Sockets (UDP)

- **U**ser **D**atagram **P**rotocol sockets
- Stateless communication: no ongoing, established connections
    - Send data to a destination IP and port, and assume they're listening
- "At least once" delivery.
    - Packets are not guaranteed to be delivered in order
    - Packets may be received more than once
- Traditionally implement two methods:
    - `send_to(addr)`: sends data over the socket to the address
    - `recv_from()`: listens for data being sent to the socket

---
## `std::net::UdpSocket`

```rust
// Try to bind a UDP socket
let mut socket = try!(UdpSocket::bind("127.0.0.1:34254"));

// Try to receive data from the socket we've bound
let mut buf = [0; 10];
let (amt, src) = try!(socket.recv_from(&mut buf));

// Send a reply to the socket we just received data from
let buf = &mut buf[..amt];
buf.reverse();
try!(socket.send_to(buf, &src));

// The socket is closed when it's `Drop`ed from this scope.
```

???

No exercises for this because it's annoying to manipulate u8 arrays directly.

---
## Stream Sockets (TCP)

- **T**ransmission **C**ontrol **P**rotocol sockets
- Stateful: require a connection to be established and acknowledged between two
    clients (using SYN packet)
    - Connection must also be explicitly closed
- A few types of different packets (SYN, ACK, FIN)
- TCP _listeners_ and _senders_ behave differently.

---
## TCP

1. Server opens a socket which listens on a port.
2. A client sends: SYN `-->` server
3. Server responds: client `<--` SYN-ACK
4. Client responds: ACK `<--` server

- Similar process for shutting down a connection
- Packets are delivered in-order, _exactly once_
    - Packet sequence numbers
- Server responds further messages with ACK
- You never have to implement this part of the protocol

---
### TCP

- TCP communication is asymmetric
- Communication happens in a `TcpStream`, which implements `Read` and `Write`
- Servers (`TcpListener`) bind to a port
    - `TcpListener::incoming()` is an iterator of new connections
    - Use the yielded `TcpStream` to read and write to the connections
- Clients can use `TcpStream::connect` to start a connection to a server

---
### `std::net::TcpStream`

- A TCP stream between a local socket and a remote socket.

```rust
use std::io::{Read, Write};

// Create a TCP connection
let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();

// Try to write a byte to the stream
let write_result = stream.write(&[1]);

// Read from the stream into buf
let mut buf = [0; 128];
let read_result = stream.read(&mut buf);

// Create a BufReader so you can use `read_line`
let mut reader = BufReader::new(try!(stream.try_clone()));
// ...

// Socket gets automatically closed when it goes out of scope
```

---
## Exercise: Chat Client

- Try it out: `telnet <ip> 19800`
- Create a new Cargo project: `cargo new --bin project`
- Then write a chat client:
- Connect with `TcpStream::connect`
- First write your name, then you can write messages
- Read messages from StdIn
    - `try!(io::stdin().read_line(&mut buffer));`
    - (It's kind of derpy because Rust doesn't have an easy non-blocking
      standard in... so you'll only get messages after you send them.)

---
### `std::net::TcpListener`

- A TCP socket server.

```rust
let listener = TcpListener::bind("127.0.0.1:80").unwrap();

// Accept connections and process them,
// spawning a new thread for each one.
for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            // ...
        }
        Err(e) => {
            // ...
        }
    }
}

// Socket gets automatically closed when it goes out of scope
```

---
## Exercise: Echo Server

- Accept connections, read in a line, and write it back to the TcpStream.
    - Use `BufReader::read_line` to read from socket.
- Use Telnet to test.

---
## Exercise: File Server

- Accept file names, then write file contents back to the TcpStream.
- Use `File::open` to open files and `read_to_string` to read data.
    - Remember to import the `Read` trait.

???

Congratulations, you wrote the more interesting part of an HTTP server.

In 555 you get to do the boring parts like parse headers and handle errors.

---
## [Web](http://arewewebyet.org/)

---
## HTTP

- "Hypertext Transfer Protocol" is basically the protocol that defines the
  internet.
- HTTP defines several common actions for interacting with servers over the
  Internet
- Common HTTP verbs are:
    - GET: retrieve data (e.g. access a web page)
    - POST: send data (e.g. submit a login form)
    - PATCH: modify existing data (e.g. modify a user profile)
    - DELETE: delete data (e.g. delete a user)
- Others exist, but are less common

---
### HTTP

- An HTTP _request_ is made by sending some data to a server over HTTP
    containing some data, such as:
    - the URL of the server
    - the method you want to invoke
    - data the server needs to look at (like in a POST)
    - names of data you want back from the server
    - etc.

```
GET /index.html HTTP/1.1
Host: www.google.com
...
```

---
### HTTP

- Once the server processes your request, you get a _response_
- Responses usually contain:
    - a status code (200, 404, 502, etc.)
    - some information pertaining to your request:
        - HTML content
        - JSON-formatted data
        - Error messages
        - etc.

```
200 OK
Date: [...]
Last-Modified: [...]

...
```

---
### [Hyper](http://hyper.rs)

- "A Modern HTTP library for Rust"
- Provides a relatively low-level wrapper over raw HTTP.
- Because you never want to implement the HTTP protocol yourself.

---
### `hyper::client`

- An HTTP client.
- Designed for most people to make HTTP requests, using the `client::Request`
    API.

```rust
let client = Client::new();

// GET
let res = client.get("http://www.google.com")
                .send().unwrap();
assert_eq!(res.status, hyper::Ok);

// POST
let res = client.post("http://www.google.com")
                .body("user=me")
                .send().unwrap();
assert_eq!(res.status, hyper::Ok);

// PUT, PATCH, DELETE, etc. are all legal verbs too.
```

- `Client` is shareable between threads, so you can make requests _in parallel by default_!

---
### `Client` Requests

- With some proper error handling.
- A GET request that reads out the body of a web page:

```rust
extern crate hyper;

use std::io::Read;
use hyper::client::Client;

let url = "http://www.google.com";
let client = Client::new();
let mut response = try!(client.get(url).send());
let mut buf = String::new();
try!(response.read_to_string(&mut buf));
println!("{}", buf)
```

---
### Exercise: random.org

- To use Hyper, add to Cargo.toml:

```
[dependencies]
hyper = "0.9"
```

- Random.org supports an HTTP API for URLs of the following form:
```
https://www.random.org/integers/
    ?num=10
    &min=1
    &max=6
    &col=1
    &base=10
    &format=plain
```
