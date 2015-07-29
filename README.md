# shutdown_hooks [![Build Status](https://travis-ci.org/Techern/shutdown_hooks_rs.svg)](https://travis-ci.org/Techern/shutdown_hooks_rs) [![Crates.io](https://img.shields.io/crates/v/shutdown_hooks.svg)](https://crates.io/crates/shutdown_hooks) 
Shutdown hook functionality for Rust. Basically a wrapper around atexit for now

An unspecified future release will feature the ability to remove shutdown hooks

#Notes

So far, I have successfully tested an eight-second long shutdown hook under Windows. I haven't tested any longer than that, but you should be fine. If not, you would have faced the same issue with atexit anyway :)