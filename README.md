# arcturus
A STUN server and client implementation in Rust.
It is named after a [star](https://en.wikipedia.org/wiki/Arcturus) used for navigation.

Ask arcturus to shine at the default loopback address and 7969 port:
```
$ cargo run -- shine
Started shining at V4(127.0.0.1:7969)
```
or at a specific address:
```
$ cargo run -- shine -a 127.0.0.1:7969
Started shining at V4(127.0.0.1:7969)
```
or at an IPv6 address:
```
$ cargo run -- shine -a ::1:7969
Started shining at V6([::1]:7969)
```


Ask arcturus where you are and it gives you your address as a `u32`:
```
$ cargo run -- whereami --rh 127.0.0.1:7969
Your IPv4 addr is 2130706433
```
or query arcturus if it is listening at an IPv6 address:
```
$ cargo run -- whereami --rh ::1:7969
Your IPv6 addr is 1
```
