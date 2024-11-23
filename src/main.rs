extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
use std::env;
use std::path::PathBuf;

mod screens;
use screens::{ScreenManager, ScreenState};
use screens::game::GameScreen;

fn main() {
    // Initialize OpenGL
    let opengl = OpenGL::V3_2;
    let window_size = [800, 600];

    // Create a Glutin window
    let mut window: GlutinWindow = WindowSettings::new("INTERIM", window_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Failed to build GlutinWindow.");

    let mut gl = GlGraphics::new(opengl);

    // Determine the executable's directory
    let exe_path = env::current_exe().expect("Failed to get executable path.");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory.");

    // Construct the path to the font file relative to the executable's directory
    let font_path = exe_dir.join("assets").join("dogicapixel.ttf");

    // Verify that the font file exists
    if !font_path.exists() {
        eprintln!("Font file not found at {:?}", font_path);
        std::process::exit(1);
    }

    // Load the font
    let mut glyphs = GlyphCache::new(
        font_path,
        (),
        TextureSettings::new(),
    ).expect("Could not load font.");

    // Initialize the screen manager and add the game screen
    let mut screen_manager = ScreenManager::new();
    screen_manager.add_screen(ScreenState::Game, Box::new(GameScreen::new()));

    // Create an event loop
    let mut events = Events::new(EventSettings::new());

    // Start the main event loop
    while let Some(e) = events.next(&mut window) {
        // Handle rendering
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                screen_manager.draw(&c, g, &mut glyphs, [args.window_size[0] as f64, args.window_size[1] as f64]);
            });
        }

        // Update game state
        screen_manager.update();

        // Handle button press inputs
        if let Some(input) = e.press_args() {
            screen_manager.handle_input(&Input::Button(ButtonArgs {
                state: ButtonState::Press,
                button: input,
                scancode: None,
            }));
        }

        // Handle mouse movement inputs
        if let Some(pos) = e.mouse_cursor_args() {
            screen_manager.handle_input(&Input::Move(Motion::MouseCursor(pos)));
        }
    }
}