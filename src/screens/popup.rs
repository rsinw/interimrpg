use graphics::*;
use opengl_graphics::{GlGraphics, GlyphCache};
use std::time::{Instant, Duration};

pub enum PopupType {
    TextBox {
        text: String,
        duration: Duration,
        created_at: Instant,
    }
}

pub struct Popup {
    pub active: bool,
    pub popup_type: PopupType,
}

impl Popup {
    pub fn new_text_box(text: String, duration_secs: f64) -> Self {
        Popup {
            active: true,
            popup_type: PopupType::TextBox {
                text,
                duration: Duration::from_secs_f64(duration_secs),
                created_at: Instant::now(),
            },
        }
    }

    pub fn draw(&self, c: &Context, g: &mut GlGraphics, glyphs: &mut GlyphCache, window_size: [f64; 2]) {
        if !self.active {
            return;
        }

        match &self.popup_type {
            PopupType::TextBox { text, .. } => {
                let box_height = 60.0;
                let margin = 20.0;
                let y_position = window_size[1] - box_height - margin;
                let box_width = window_size[0] - (margin * 2.0);

                // Draw text box background
                rectangle(
                    [0.0, 0.0, 0.0, 0.8],  // Semi-transparent black
                    [margin, y_position, box_width, box_height],
                    c.transform,
                    g
                );

                // Draw white border (four lines)
                let border_width = 2.0;
                let border_color = [1.0, 1.0, 1.0, 1.0];  // White

                // Top border
                rectangle(
                    border_color,
                    [margin, y_position, box_width, border_width],
                    c.transform,
                    g
                );

                // Bottom border
                rectangle(
                    border_color,
                    [margin, y_position + box_height - border_width, box_width, border_width],
                    c.transform,
                    g
                );

                // Left border
                rectangle(
                    border_color,
                    [margin, y_position, border_width, box_height],
                    c.transform,
                    g
                );

                // Right border
                rectangle(
                    border_color,
                    [margin + box_width - border_width, y_position, border_width, box_height],
                    c.transform,
                    g
                );

                // Draw text
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 16)
                    .draw(
                        text,
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(margin + 10.0, y_position + 35.0),
                        g,
                    )
                    .unwrap_or_else(|e| eprintln!("Error drawing text: {}", e));
            }
        }
    }

    pub fn update(&mut self) {
        match &self.popup_type {
            PopupType::TextBox { created_at, duration, .. } => {
                if created_at.elapsed() >= *duration {
                    self.active = false;
                }
            }
        }
    }
} 