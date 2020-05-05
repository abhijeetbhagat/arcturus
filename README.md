# arcturus
A STUN ([RFC5389](https://tools.ietf.org/html/rfc5389)) server and client implementation in Rust.
It is named after a [star](https://en.wikipedia.org/wiki/Arcturus) used for navigation.

Ask arcturus to shine at the default loopback address and 7969 port with UDP as default transport mode:
```
$ cargo run -- shine
Started shining using UDP at V4(127.0.0.1:7969)
```
or at a specific address:
```
$ cargo run -- shine -a 127.0.0.1:7969
Started shining using UDP at V4(127.0.0.1:7969)
```
or at an IPv6 address:
```
$ cargo run -- shine -a ::1:7969
Started shining at V6([::1]:7969)
```
or at an IPv6 address with TCP as transport:
```
$ cargo run -- shine -a ::1:7969 -t tcp
Started shining using TCP at V6([::1]:7969)
```


Ask arcturus where you are and it gives you your address as a `u32`:
```
$ cargo run -- whereami -h 127.0.0.1:7969
Your IPv4 addr is 2130706433
```
or query arcturus if it is listening at an IPv6 address:
```
$ cargo run -- whereami -h ::1:7969
Your IPv6 addr is 1
```
or query arcturus if it is listening at an IPv6 address with UDP as transport:
```
$ cargo run -- whereami -h ::1:7969 -t udp
Your IPv6 addr is 1
```
