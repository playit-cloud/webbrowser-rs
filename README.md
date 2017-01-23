# webbrowser

Rust library to open URLs in the web browsers available on a platform

Inspired by the [webbrowser](https://docs.python.org/2/library/webbrowser.html) python library

[Detailed documentation](http://code.rootnet.in/webbrowser-rs/webbrowser/)

[Crate location](https://crates.io/crates/webbrowser)

##Examples

```rust
use webbrowser;

if webbrowser::open("http://github.com").is_ok() {
    // ...
}
```

Currently state of platform support is:

* macos => default, as well as browsers listed under [Browser](enum.Browser.html)
* windows => default browser only
* linux => default browser only (uses $BROWSER env var, failing back to xdg-open, gvfs-open, gnome-open, whichever works first)
* android => not supported right now
* ios => not supported right now

Important note:

* This library requires availability of browsers and a graphical environment during runtime
* `cargo test` will actually open the browser locally

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
