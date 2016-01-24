# Jamkit [![Build Status](https://img.shields.io/travis/athena-org/jamkit/master.svg?style=flat-square)](https://travis-ci.org/athena-org/jamkit) [![crates.io](http://meritbadge.herokuapp.com/jamkit?style=flat-square)](https://crates.io/crates/jamkit)

A small game development library.

```Rust
let mut display = jamkit::Graphics::init("test", 640, 480);
let texture = jamkit::Texture::load(&display, "texture.png");

'main: loop {
    for event in display.poll_events() {
        match event {
            jamkit::Event::Closed => break 'main,
            _ => ()
        }
    }

    let mut frame = jamkit::Frame::start(&display);
    frame.draw(&texture, None, [0, 0, 200, 200]);
    frame.finish();
}
```

## License
Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
