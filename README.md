# Jamkit

A small game development library.

```Rust
let mut display = jamkit::Graphics::init();
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
