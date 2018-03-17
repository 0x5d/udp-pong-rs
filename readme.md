# udp-pong-rs

A basic UDP pong server to learn Rust.

I wanted to see what a very basic UDP server looks like in Rust, so I made this.
Maybe it would be useful to implement a gossip protocol in Rust? :)

## Run it
```sh
cargo run <response> <port>
```
```sh
nc -u 127.0.0.1 <port>
# Type something and it will respond with <response>
```
