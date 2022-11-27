# TCP proxy

In this task, we'll implement a simple TCP proxy.

## Implementation

The proxy starts using `run_proxy(port, destination)`, where `port` is port to bind, host must be localhost, and `destination` is the address of the server to which the proxy will go.

```rust
pub fn run_proxy(port: u32, destination: String);
```

The proxy must be able to handle an unlimited number of simultaneous connections. You shouldn't join threads here, as we're writing just a simple proxy. You may need the documentation for [TcpStream](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html). Also check the `Shutdown` method.

After implementation, you can check your proxy:

1. Run a proxy for some HTTP site: `cargo run -- -d www.lamport.org:80 -p 8000`
2. Set up our service in the browser as HTTP-proxy. The documentation for [Firefox](https://support.mozilla.org/en-US/kb/connection-settings-firefox).
3. Go to the site and make sure everything works. You're also allowed to use `log` crate, and with good logging you can observe how browser interacts with the connection.

### Note

There is no IPv6 interface in docker containers where tests for your solution are running. So if you use explicit IP addresses, use IPv4.
