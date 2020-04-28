# arcturus
A STUN server and client implementation in Rust

Ask arcturus to shine:
```
$ cargo run -- shine
Started shining at V4(127.0.0.1:7969)
```

Ask arcturus where you are and it gives you your address as a `u32`:
```
$ cargo run -- whereami --rh 127.0.0.1 --rp 7969
Your IP is 2130706433
```
