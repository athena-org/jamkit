# Jamkit

A small game development library.

```Rust
extern crate jamkit;

fn main() {
    let mut display = jamkit::Graphics::init();
    let test_texture = jamkit::Texture::load(&display, "examples/test.png");

    'main: loop {
        for event in display.poll_events() {
            match event {
                jamkit::Event::Closed => break 'main,
                _ => ()
            }
        }

        let mut frame = jamkit::Frame::start(&display);
        frame.draw(&test_texture, None, [0, 0, 200, 200]);
        frame.draw(&test_texture, Some([50, 50, 150, 150]), [50, 250, 150, 350]);
        frame.finish();
    }
}
```
