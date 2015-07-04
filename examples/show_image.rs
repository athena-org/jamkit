// Copyright 2015 The Athena Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
