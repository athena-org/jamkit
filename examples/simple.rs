extern crate jamkit;

use jamkit::{Key};
use jamkit::utils::{DeterminismTimer, InputState};

fn main() {
    let mut display = jamkit::Graphics::init("test", 640, 480);
    let test_texture = jamkit::Texture::load(&display, "examples/test.png");

    let mut input = InputState::new();
    let mut x = 0;

    let mut timer = DeterminismTimer::at_interval(10);
    'main: loop {
        for event in display.poll_events() {
            match event {
                jamkit::Event::Closed => break 'main,
                jamkit::Event::KeyboardInput(state, key) => input.process_keyboard(&state, &key),
                _ => {}
            }
        }

        timer.update(&mut |_| {
            let a = input.get(Key::A);
            let d = input.get(Key::D);
            if a.is_pressed() && d.is_released() { x -= 1; }
            if d.is_pressed() && a.is_released() { x += 1; }
        });

        let mut frame = jamkit::Frame::start(&display);
        frame.draw(&test_texture, None, [0, 0, 200, 200]);
        frame.draw(&test_texture, Some([50, 50, 150, 150]), [x, 250, x + 100, 350]);
        frame.finish();
    }
}
