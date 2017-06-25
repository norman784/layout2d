#[macro_use]
extern crate glium;
extern crate rctree;
extern crate simd;

pub mod ui_screen;
pub mod input;
pub mod node_data;
pub mod renderer;
mod debug;

use rctree::NodeRef;
use node_data::NodeData;
use debug::DebugColor;
use renderer::Renderer;

const INITIAL_HEIGHT: u32 = 600;
const INITIAL_WIDTH: u32 = 800;

fn main() {

    // Initialize keyboard & mouse
    let mut keyboard_state = input::KeyboardState::new();
    let mut mouse_state = input::MouseState::new(10.0, 10.0);
    let mut window_state = input::WindowState::new(INITIAL_WIDTH, INITIAL_HEIGHT);
    let renderer = Renderer::new(INITIAL_WIDTH, INITIAL_HEIGHT);

    // Construct the explorer UI
    let mut ui_screen = ui_screen::UiScreen::new();
        // Top bar, 100 - 200 pixels tall, stretches full window
        ui_screen.root.append(NodeRef::new(NodeData::new(None, Some(100), None, Some(200), DebugColor::green())));
        // Side bar, max 400 px wide 
        ui_screen.root.append(NodeRef::new(NodeData::new(None, None, Some(400), None, DebugColor::red())));
        // Main explorer view, stretches all sides
        ui_screen.root.append(NodeRef::new(NodeData::new(None, None, None, None, DebugColor::blue() )));

        // renderer.display.get_window().unwrap().set_cursor(glium::glutin::MouseCursor::Wait);

        /// when adding animations, change this to poll events
        for event in renderer.display.wait_events() {
            input::handle_event(&event, &mut window_state, &mut keyboard_state, &mut mouse_state, &mut ui_screen);
            renderer.render(&window_state, &ui_screen);
        }
}
