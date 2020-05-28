# arcturus
A STUN ([RFC5389](https://tools.ietf.org/html/rfc5389)) server and client implementation in Rust.
It is named after a [star](https://en.wikipedia.org/wiki/Arcturus) used for navigation.

Ask arcturus to shine at the default loopback address and 3478 port with UDP as default transport mode:
```
$ cargo run -- shine
Started shining using UDP at V4(127.0.0.1:3478)
```
or at a specific address:
```
$ cargo run -- shine -a 127.0.0.1:3478
Started shining using UDP at V4(127.0.0.1:3478)
```
or at an IPv6 address:
```
$ cargo run -- shine -a ::1:3478
Started shining at V6([::1]:3478)
```
or at an IPv6 address with TCP as transport:
```
$ cargo run -- shine -a ::1:3478 -t tcp
Started shining using TCP at V6([::1]:3478)
```


Ask arcturus where you are and it gives you your address:
```
$ cargo run -- whereami -h 127.0.0.1:3478
Your IPv4 addr is 127.0.0.1
```
or query arcturus if it is listening at an IPv6 address:
```
$ cargo run -- whereami -h ::1:3478
Your IPv6 addr is 1
```
or query arcturus if it is listening at an IPv6 address with UDP as transport:
```
$ cargo run -- whereami -h ::1:3478 -t udp
Your IPv6 addr is 1
```

### Building
1. Install [rust](https://www.rust-lang.org/tools/install).
2. Clone this repo
3. `$ cd arcturus`
4. `$ cargo build`

Executable should be created in the `target` folder. Replace `cargo run` with `arcturus` in the above commands.
