# shutdown_hooks [![Build Status](https://travis-ci.org/Techern/shutdown_hooks_rs.svg)](https://travis-ci.org/Techern/shutdown_hooks_rs) [![Crates.io](https://img.shields.io/crates/v/shutdown_hooks.svg)](https://crates.io/crates/shutdown_hooks) 
Shutdown hook functionality for Rust. Basically a wrapper around atexit for now

An unspecified future release will feature the ability to remove shutdown hooks

#Notes

So far, I have successfully tested an eight-second long shutdown hook under Windows. I haven't tested any longer than that, but you should be fine. If not, you would have faced the same issue with atexit anyway :)
## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
