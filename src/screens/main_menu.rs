use graphics::*;
use graphics::types::Color;
use piston::input::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use crate::screens::{Screen, ScreenState};

const BUTTON_COLOR: Color = [0.2, 0.2, 0.2, 1.0];
const HOVER_COLOR: Color = [0.3, 0.3, 0.3, 1.0];
const TEXT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
const TRIANGLE_COLOR: Color = [0.8, 0.2, 0.2, 1.0];

pub struct MainMenu {
    mouse_pos: [f64; 2],
    play_hover: bool,
    quit_hover: bool,
}

impl MainMenu {
    pub fn new() -> Self {
        MainMenu {
            mouse_pos: [0.0, 0.0],
            play_hover: false,
            quit_hover: false,
        }
    }

    fn update_hover_states(&mut self, pos: [f64; 2], window_size: [f64; 2]) {
        let button_width = 200.0;
        let button_height = 50.0;
        let center_x = window_size[0] / 2.0 - button_width / 2.0;
        
        let play_y = window_size[1] / 2.0;
        self.play_hover = pos[0] >= center_x 
            && pos[0] <= center_x + button_width
            && pos[1] >= play_y 
            && pos[1] <= play_y + button_height;

        let quit_y = play_y + button_height + 20.0;
        self.quit_hover = pos[0] >= center_x 
            && pos[0] <= center_x + button_width
            && pos[1] >= quit_y 
            && pos[1] <= quit_y + button_height;
    }
}

impl Screen for MainMenu {
    fn draw(&mut self, c: &Context, g: &mut GlGraphics, glyphs: &mut GlyphCache, window_size: [f64; 2]) {
        // Clear screen
        clear([0.1, 0.1, 0.1, 1.0], g);

        let button_width = 200.0;
        let button_height = 50.0;
        let center_x = window_size[0] / 2.0 - button_width / 2.0;

        // Draw play button
        let play_y = window_size[1] / 2.0;
        rectangle(
            if self.play_hover { HOVER_COLOR } else { BUTTON_COLOR },
            [center_x, play_y, button_width, button_height],
            c.transform,
            g
        );

        // Draw "PLAY" text
        let play_text = "PLAY";
        let text_size = 16;
        
        // Calculate text width for centering
        let play_text_width = glyphs.width(text_size, play_text)
            .unwrap_or(70.0);  // Fallback width if calculation fails
        
        text::Text::new_color(TEXT_COLOR, text_size)
            .draw(
                play_text,
                glyphs,
                &c.draw_state,
                c.transform.trans(
                    center_x + (button_width - play_text_width) / 2.0,
                    play_y + button_height * 0.65  // Vertically center text
                ),
                g,
            )
            .unwrap();

        // Draw quit button
        let quit_y = play_y + button_height + 20.0;
        rectangle(
            if self.quit_hover { HOVER_COLOR } else { BUTTON_COLOR },
            [center_x, quit_y, button_width, button_height],
            c.transform,
            g
        );

        // Draw "QUIT" text
        let quit_text = "QUIT";
        
        // Calculate text width for centering
        let quit_text_width = glyphs.width(text_size, quit_text)
            .unwrap_or(70.0);  // Fallback width if calculation fails
        
        text::Text::new_color(TEXT_COLOR, text_size)
            .draw(
                quit_text,
                glyphs,
                &c.draw_state,
                c.transform.trans(
                    center_x + (button_width - quit_text_width) / 2.0,
                    quit_y + button_height * 0.65  // Vertically center text
                ),
                g,
            )
            .unwrap();

        // Draw triangle
        let triangle_size = 50.0;
        let triangle_x = window_size[0] - triangle_size - 20.0;
        let triangle_y = window_size[1] / 2.0;

        let triangle = [
            [triangle_x, triangle_y - triangle_size],
            [triangle_x - triangle_size, triangle_y + triangle_size],
            [triangle_x + triangle_size, triangle_y + triangle_size],
        ];

        polygon(
            TRIANGLE_COLOR,
            &triangle,
            c.transform,
            g
        );
    }

    fn handle_input(&mut self, input: &Input) -> Option<ScreenState> {
        match input {
            Input::Move(Motion::MouseCursor(pos)) => {
                self.update_hover_states(*pos, [800.0, 600.0]);
                None
            }
            Input::Button(ButtonArgs {state: ButtonState::Press, button: Button::Mouse(MouseButton::Left), ..}) => {
                if self.play_hover {
                    Some(ScreenState::Game)
                } else if self.quit_hover {
                    std::process::exit(0);
                } else {
                    None
                }
            }
            _ => None
        }
    }
} 