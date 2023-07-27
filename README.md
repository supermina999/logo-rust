# Logo Rust
This repo contains a full implementation of Logo programming language interpreter and IDE written in Rust. 

[Online Demo](https://logo-worlds.pages.dev/)

## Crates

### Logo-interp
Implements parsing and interpreting of Logo programming language. Use this crate directly if you want to interpret some abstract Logo code.

### Logo-runtime
Builds on top of `logo-interp`, adds the runtime for graphic functionality of the Logo language. You can use this crate if you want to integrate Logo with a custom renderer.

### Logo-renderer
Implements a simple CPU renderer for `logo-runtime`. You can use this crate to run Logo code and get some images out of it.

### Logo-egui
Simple GUI for Logo environment, primarily used for testing & debugging purposes.

### Logo-web
Web-based IDE for Logo development, this one is shown in the demo.
