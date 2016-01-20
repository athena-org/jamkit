extern crate jamkit;

use jamkit::{Graphics, Texture, Frame, Key};
use jamkit::utils::{TickTimer, InputState};

fn main() {
    // Initialize jamkit
    let mut display = Graphics::init("test", 640, 480);

    // Load in a texture to display
    let test_texture = Texture::load(&display, "examples/test.png");

    // Some helpers and game state
    let mut input = InputState::new();
    let mut timer = TickTimer::at_interval(10);
    let mut x = 0;

    // Run the game loop
    'main: loop {
        // Handle all the events
        for event in display.poll_events() {
            match event {
                jamkit::Event::Closed => break 'main,
                jamkit::Event::KeyboardInput(state, key) => input.process_keyboard(state, key),
                _ => {}
            }
        }

        // Update our game state
        timer.update(|_| {
            let a = input[Key::A];
            let d = input[Key::D];
            if a.is_pressed() && d.is_released() { x -= 1; }
            if d.is_pressed() && a.is_released() { x += 1; }
        });

        // Draw our game
        let mut frame = Frame::start(&display);
        frame.draw(&test_texture, None, [0, 0, 200, 200]);
        frame.draw(&test_texture, Some([50, 50, 150, 150]), [x, 250, x + 100, 350]);
        frame.finish();
    }
}
