<h1>
    Jamkit
        <span style="float:right">
        <a href="https://travis-ci.org/athena-org/jamkit">
            <img src="https://img.shields.io/travis/athena-org/jamkit/master.svg?style=flat-square" alt="Build Status">
        </a>
        <a href="https://crates.io/crates/jamkit">
            <img src="http://meritbadge.herokuapp.com/gfx?style=flat-square" alt="crates.io">
        </a>
    </span>
</h1>

A small game development library.

```Rust
let mut display = jamkit::Graphics::init("test", 640, 480);
let test_texture = jamkit::Texture::load(&display, "examples/test.png");

let mut x = 0;

'main: loop {
    for event in display.poll_events() {
        match event {
            jamkit::Event::Closed => break 'main,
            jamkit::Event::KeyboardInput(state, _) =>
                if state.is_pressed() { x += 5; },
            _ => ()
        }
    }

    let mut frame = jamkit::Frame::start(&display);
    frame.draw(&test_texture, None, [0, 0, 200, 200]);
    frame.draw(&test_texture, Some([50, 50, 150, 150]), [x, 250, x + 100, 350]);
    frame.finish();
}
```

## Contributing

When creating a pull request, make sure you merge into the develop branch. The
master branch mirrors the latest crates.io release.
