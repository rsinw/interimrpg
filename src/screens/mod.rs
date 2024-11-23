use std::collections::HashMap;
use piston::input::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use graphics::Context;

pub mod main_menu;
pub mod game;
pub mod popup;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum ScreenState {
    MainMenu,
    Game,
    Settings,
    Pause,
    // Add more screens as needed
}

pub trait Screen {
    fn draw(&mut self, c: &Context, g: &mut GlGraphics, glyphs: &mut GlyphCache, window_size: [f64; 2]);
    fn handle_input(&mut self, _input: &Input) -> Option<ScreenState> { None }
    fn update(&mut self) -> Option<ScreenState> { None }
}

pub struct ScreenManager {
    screens: HashMap<ScreenState, Box<dyn Screen>>,
    current_screen: ScreenState,
    previous_screen: Option<ScreenState>,
}

impl ScreenManager {
    pub fn new() -> Self {
        let mut manager = ScreenManager {
            screens: HashMap::new(),
            current_screen: ScreenState::MainMenu,
            previous_screen: None,
        };

        // Add initial screens
        manager.add_screen(ScreenState::MainMenu, Box::new(main_menu::MainMenu::new()));
        manager
    }

    pub fn add_screen(&mut self, state: ScreenState, screen: Box<dyn Screen>) {
        self.screens.insert(state, screen);
    }

    pub fn change_screen(&mut self, new_state: ScreenState) {
        if self.screens.contains_key(&new_state) {
            self.previous_screen = Some(self.current_screen);
            self.current_screen = new_state;
        }
    }

    pub fn return_to_previous(&mut self) {
        if let Some(previous) = self.previous_screen {
            self.current_screen = previous;
            self.previous_screen = None;
        }
    }

    pub fn draw(&mut self, c: &Context, g: &mut GlGraphics, glyphs: &mut GlyphCache, window_size: [f64; 2]) {
        if let Some(screen) = self.screens.get_mut(&self.current_screen) {
            screen.draw(c, g, glyphs, window_size);
        }
    }

    pub fn update(&mut self) {
        if let Some(screen) = self.screens.get_mut(&self.current_screen) {
            if let Some(new_state) = screen.update() {
                self.change_screen(new_state);
            }
        }
    }

    pub fn handle_input(&mut self, input: &Input) {
        if let Some(screen) = self.screens.get_mut(&self.current_screen) {
            if let Some(new_state) = screen.handle_input(input) {
                self.change_screen(new_state);
            }
        }
    }
}
