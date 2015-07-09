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

## Contributing

When creating a pull request, make sure you merge into the develop branch. The
master branch mirrors the latest crates.io release.
