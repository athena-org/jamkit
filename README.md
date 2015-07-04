# Jamkit

A small game development library.

```Rust
extern crate jamkit;

use jamkit::{JamkitGraphics, JamkitFrame, JamkitTexture};

fn main() {
    let mut display = JamkitGraphics::init();
    let test_texture = JamkitTexture::load(&display, "examples/test.png");

    while !display.is_closed() {
        display.poll_events();

        let mut frame = JamkitFrame::start(&display);
        frame.draw(&test_texture, None, [0, 0, 200, 200]);
        frame.draw(&test_texture, Some([50, 50, 150, 150]), [50, 250, 150, 350]);
        frame.finish();
    }
}
```
