    use graphics::*;
    use opengl_graphics::{GlGraphics, GlyphCache};
    use piston::input::*;
    use crate::screens::{Screen, ScreenState};
    use super::popup::Popup;
    use rand::Rng;

    const GRID_MIN: i32 = -20;
    const GRID_MAX: i32 = 20;
    const POINT_SIZE: f64 = 5.0;
    const GRID_LINE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];    // White
    const PLAYER_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];       // Red
    const OBSTACLE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];     // White for obstacles
    const INTERACTABLE_COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0]; // Yellow for interactables
    const TRIANGLE_SIZE: f64 = POINT_SIZE * 1.8;
    const TRIANGLE_INSET: f64 = POINT_SIZE * 0.2;
    const TEXT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const TEXT_POS_X: f64 = 20.0;
    const TEXT_POS_Y: f64 = 30.0;
    const MAP_WIDTH: usize = (GRID_MAX - GRID_MIN + 1) as usize;
    const MAP_HEIGHT: usize = (GRID_MAX - GRID_MIN + 1) as usize;

    #[derive(Clone, Copy, PartialEq)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        fn to_string(&self) -> &'static str {
            match self {
                Direction::Up => "FACING: UP",
                Direction::Down => "FACING: DOWN",
                Direction::Left => "FACING: LEFT",
                Direction::Right => "FACING: RIGHT",
            }
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
        movable: bool,
        facing: Option<Direction>,
    }

    #[derive(Clone)]
    struct InteractablePoint {
        x: i32,
        y: i32,
        destination_map: Option<usize>,
        destination_position: Option<(i32, i32)>,
    }

    pub struct GameScreen {
        player: Point,
        map: Vec<Vec<u8>>,
        grid_scale: f64,
        popups: Vec<Popup>,
        interactables: Vec<InteractablePoint>,
        camera_position: (f64, f64),
    }

    impl GameScreen {
        pub fn new() -> Self {
            let player = Point {
                x: 0,
                y: 0,
                movable: true,
                facing: Some(Direction::Right),
            };

            let map = generate_map();
            let interactables = generate_interactables(&map);

            GameScreen {
                player,
                map,
                grid_scale: 30.0,
                popups: Vec::new(),
                interactables,
                camera_position: (0.0, 0.0),
            }
        }

        fn is_within_bounds(&self, x: i32, y: i32) -> bool {
            x >= GRID_MIN && x <= GRID_MAX && y >= GRID_MIN && y <= GRID_MAX
        }

        fn is_obstacle(&self, x: i32, y: i32) -> bool {
            let map_x = (x - GRID_MIN) as usize;
            let map_y = (GRID_MAX - y) as usize;

            // Check if indices are within bounds
            if map_x >= MAP_WIDTH || map_y >= MAP_HEIGHT {
                return true; // Treat out-of-bounds as obstacle
            }

            self.map[map_y][map_x] == 1 // Return true if the cell is an obstacle
        }

        fn try_move_player(&mut self, dx: i32, dy: i32) {
            // Update facing direction based on movement attempt
            self.player.facing = Some(match (dx, dy) {
                (0, 1) => Direction::Up,
                (0, -1) => Direction::Down,
                (-1, 0) => Direction::Left,
                (1, 0) => Direction::Right,
                _ => self.player.facing.unwrap_or(Direction::Right),
            });

            let new_x = self.player.x + dx;
            let new_y = self.player.y + dy;

            // Check if new position is within map bounds
            if !self.is_within_bounds(new_x, new_y) {
                self.show_boundary_message();
                return;
            }

            // Check if the new position is occupied by an obstacle
            if self.is_obstacle(new_x, new_y) {
                self.show_boundary_message();
                return;
            }

            // Move player to new position
            self.player.x = new_x;
            self.player.y = new_y;
        }

        fn try_interact(&mut self) {
            // Calculate the point in front of the player based on facing direction
            let (dx, dy) = match self.player.facing.unwrap_or(Direction::Right) {
                Direction::Up => (0, 1),
                Direction::Down => (0, -1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };

            let target_x = self.player.x + dx;
            let target_y = self.player.y + dy;

            // Check if there's an interactable point at the target position
            if let Some(index) = self
                .interactables
                .iter()
                .position(|point| point.x == target_x && point.y == target_y)
            {
                // Get the interactable point
                let interactable = self.interactables[index].clone();

                // Transport the player to the new map
                self.generate_new_map(interactable.destination_position);

                // Optionally remove the interactable point from the current map
                // self.interactables.remove(index);

                // Display a popup indicating the map has changed
                self.popups.push(Popup::new_text_box(
                    "You have entered a new area.".to_string(),
                    2.0,
                ));
            } else {
                // Show a message if there's nothing to interact with
                self.popups.push(Popup::new_text_box(
                    "Nothing to interact with".to_string(),
                    2.0,
                ));
            }
        }

        fn update_popups(&mut self) {
            self.popups.retain_mut(|popup| {
                popup.update();
                popup.active
            });
        }

        fn show_boundary_message(&mut self) {
            self.popups.push(Popup::new_text_box(
                "Boundary in the way".to_string(),
                2.0, // Display for 2 seconds
            ));
        }

        fn update_camera_position(&mut self, window_size: [f64; 2]) {
            let num_tiles_x = (GRID_MAX - GRID_MIN + 1) as f64;
            let num_tiles_y = (GRID_MAX - GRID_MIN + 1) as f64;

            let world_width = num_tiles_x * self.grid_scale;
            let world_height = num_tiles_y * self.grid_scale;

            // Player position in world coordinates
            let player_world_x = (self.player.x - GRID_MIN) as f64 * self.grid_scale;
            let player_world_y = (GRID_MAX - self.player.y) as f64 * self.grid_scale; // Adjusted for Y inversion

            // Desired camera position to center the player
            let desired_camera_x = player_world_x - window_size[0] / 2.0;
            let desired_camera_y = player_world_y - window_size[1] / 2.0;

            // Calculate camera bounds
            let min_camera_x = 0.0;
            let max_camera_x = world_width - window_size[0];
            let min_camera_y = 0.0;
            let max_camera_y = world_height - window_size[1];

            // Clamp camera position
            self.camera_position.0 = desired_camera_x.clamp(min_camera_x, max_camera_x);
            self.camera_position.1 = desired_camera_y.clamp(min_camera_y, max_camera_y);
        }

        fn grid_to_screen(&self, x: i32, y: i32) -> [f64; 2] {
            // Convert grid coordinates to world coordinates
            let world_x = (x - GRID_MIN) as f64 * self.grid_scale;
            let world_y = (GRID_MAX - y) as f64 * self.grid_scale; // Y-axis inversion

            // Convert world coordinates to screen coordinates
            let screen_x = world_x - self.camera_position.0;
            let screen_y = world_y - self.camera_position.1;

            [screen_x, screen_y]
        }

        fn draw_grid(&self, c: &Context, g: &mut GlGraphics) {
            for x in GRID_MIN..=GRID_MAX {
                let start = self.grid_to_screen(x, GRID_MIN);
                let end = self.grid_to_screen(x, GRID_MAX);
                line(
                    GRID_LINE_COLOR,
                    0.5,
                    [start[0], start[1], end[0], end[1]],
                    c.transform,
                    g,
                );
            }

            for y in GRID_MIN..=GRID_MAX {
                let start = self.grid_to_screen(GRID_MIN, y);
                let end = self.grid_to_screen(GRID_MAX, y);
                line(
                    GRID_LINE_COLOR,
                    0.5,
                    [start[0], start[1], end[0], end[1]],
                    c.transform,
                    g,
                );
            }
        }

        fn draw_point(&self, point: &Point, color: [f32; 4], c: &Context, g: &mut GlGraphics) {
            let pos = self.grid_to_screen(point.x, point.y);

            if point.movable && point.facing.is_some() {
                // For the player, draw a directional triangle
                let (sin, cos) = match point.facing.unwrap() {
                    Direction::Right => (0.0, 1.0),   // Point right
                    Direction::Up => (-1.0, 0.0),     // Point up
                    Direction::Left => (0.0, -1.0),   // Point left
                    Direction::Down => (1.0, 0.0),    // Point down
                };

                let tip_x = pos[0] + cos * TRIANGLE_SIZE;
                let tip_y = pos[1] + sin * TRIANGLE_SIZE;

                let base_x = pos[0] - cos * TRIANGLE_INSET;
                let base_y = pos[1] - sin * TRIANGLE_INSET;

                let half_base = TRIANGLE_SIZE * 0.5;
                let base1_x = base_x - sin * half_base;
                let base1_y = base_y + cos * half_base;
                let base2_x = base_x + sin * half_base;
                let base2_y = base_y - cos * half_base;

                let triangle = [[tip_x, tip_y], [base1_x, base1_y], [base2_x, base2_y]];

                polygon(color, &triangle, c.transform, g);
            } else {
                ellipse(
                    color,
                    [
                        pos[0] - POINT_SIZE,
                        pos[1] - POINT_SIZE,
                        POINT_SIZE * 2.0,
                        POINT_SIZE * 2.0,
                    ],
                    c.transform,
                    g,
                );
            }
        }

        fn draw_direction_text(&self, c: &Context, g: &mut GlGraphics, glyphs: &mut GlyphCache) {
            if let Some(direction) = self.player.facing {
                text::Text::new_color(TEXT_COLOR, 16)
                    .draw(
                        direction.to_string(),
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(TEXT_POS_X, TEXT_POS_Y),
                        g,
                    )
                    .unwrap_or_else(|e| eprintln!("Error drawing text: {}", e));
            }
        }

        fn draw_obstacles(&self, c: &Context, g: &mut GlGraphics) {
            for (map_y, row) in self.map.iter().enumerate() {
                for (map_x, &cell) in row.iter().enumerate() {
                    if cell == 1 {
                        // Convert map indices back to grid coordinates
                        let x = map_x as i32 + GRID_MIN;
                        let y = GRID_MAX as i32 - map_y as i32;

                        let obstacle = Point {
                            x,
                            y,
                            movable: false,
                            facing: None,
                        };
                        self.draw_point(&obstacle, OBSTACLE_COLOR, c, g);
                    }
                }
            }
        }

        fn generate_new_map(&mut self, destination_position: Option<(i32, i32)>) {
            // Generate a new map
            self.map = generate_map();

            // Reinitialize interactable points
            self.interactables = generate_interactables(&self.map);

            // Set player position based on the destination coordinates
            if let Some((x, y)) = destination_position {
                self.player.x = x;
                self.player.y = y;
            } else {
                // Default position if no destination is specified
                self.player.x = 0;
                self.player.y = 0;
            }

            // Reset the player's facing direction
            self.player.facing = Some(Direction::Right);

            // Clear existing popups
            self.popups.clear();
        }
    }

    impl Screen for GameScreen {
        fn draw(
            &mut self,
            c: &Context,
            g: &mut GlGraphics,
            glyphs: &mut GlyphCache,
            window_size: [f64; 2],
        ) {
            self.update_camera_position(window_size);

            clear([0.0, 0.0, 0.0, 1.0], g);
            self.draw_grid(c, g);

            // Draw obstacles
            self.draw_obstacles(c, g);

            // Draw interactable points
            for interactable in &self.interactables {
                let point = Point {
                    x: interactable.x,
                    y: interactable.y,
                    movable: false,
                    facing: None,
                };
                self.draw_point(&point, INTERACTABLE_COLOR, c, g);
            }

            // Draw player
            self.draw_point(&self.player, PLAYER_COLOR, c, g);

            // Draw direction text
            self.draw_direction_text(c, g, glyphs);

            // Draw popups
            for popup in &self.popups {
                popup.draw(c, g, glyphs, window_size);
            }
        }

        fn update(&mut self) -> Option<ScreenState> {
            self.update_popups();
            None
        }

        fn handle_input(&mut self, input: &Input) -> Option<ScreenState> {
            match input {
                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Keyboard(key),
                    ..
                }) => match key {
                    Key::W => self.try_move_player(0, 1),
                    Key::S => self.try_move_player(0, -1),
                    Key::A => self.try_move_player(-1, 0),
                    Key::D => self.try_move_player(1, 0),
                    Key::E => self.try_interact(),
                    Key::Escape => return Some(ScreenState::Pause),
                    _ => {}
                },
                _ => {}
            }
            None
        }
    }

    fn add_house(
        map: &mut Vec<Vec<u8>>,
        top_left: (i32, i32),
        bottom_right: (i32, i32),
        entrance_position: (i32, i32),
    ) {
        let (x1, y1) = top_left;
        let (x2, y2) = bottom_right;

        // Determine the start and end coordinates for the house
        let x_start = x1.min(x2);
        let x_end = x1.max(x2);
        let y_start = y1.min(y2);
        let y_end = y1.max(y2);

        // Iterate over the specified area to place walls
        for y in y_start..=y_end {
            for x in x_start..=x_end {
                // Skip the entrance position
                if (x, y) == entrance_position {
                    continue;
                }

                // Only place walls on the edges to create the house outline
                if y == y_start || y == y_end || x == x_start || x == x_end {
                    // Convert grid coordinates to map indices
                    let map_x = (x - GRID_MIN) as usize;
                    let map_y = (GRID_MAX - y) as usize;

                    // Check if indices are within bounds
                    if map_x < MAP_WIDTH && map_y < MAP_HEIGHT {
                        map[map_y][map_x] = 1; // Place an obstacle
                    }
                }
            }
        }
    }

    fn generate_map() -> Vec<Vec<u8>> {
        let mut map = vec![vec![0u8; MAP_WIDTH]; MAP_HEIGHT];

        // Set boundaries (1 on the edges)
        for x in 0..MAP_WIDTH {
            map[0][x] = 1;                  // Top boundary
            map[MAP_HEIGHT - 1][x] = 1;     // Bottom boundary
        }
        for y in 0..MAP_HEIGHT {
            map[y][0] = 1;                  // Left boundary
            map[y][MAP_WIDTH - 1] = 1;      // Right boundary
        }

        // Add houses with corrected tuples
        add_house(
            &mut map,
            (-10, 10),    // top_left corner as a tuple
            (-5, 5),      // bottom_right corner as a tuple
            (-7, 5),      // entrance position
        );

        add_house(
            &mut map,
            (5, -5),
            (10, -10),
            (7, -10),
        );

        // Randomly add obstacles
        let mut rng = rand::thread_rng();
        let obstacle_count = rng.gen_range(50..150); // Adjust as desired

        for _ in 0..obstacle_count {
            let x = rng.gen_range(GRID_MIN + 1..GRID_MAX);
            let y = rng.gen_range(GRID_MIN + 1..GRID_MAX);

            // Convert grid coordinates to map indices
            let map_x = (x - GRID_MIN) as usize;
            let map_y = (GRID_MAX - y) as usize;

            // Ensure we don't overwrite boundaries
            if map_x < MAP_WIDTH && map_y < MAP_HEIGHT && map[map_y][map_x] == 0 {
                map[map_y][map_x] = 1; // Place an obstacle
            }
        }

        map
    }

    fn generate_interactables(map: &Vec<Vec<u8>>) -> Vec<InteractablePoint> {
        let mut interactables: Vec<InteractablePoint> = Vec::new();
        let mut rng = rand::thread_rng();
        let mut attempts = 0;

        while interactables.len() < 5 && attempts < 1000 {
            let x = rng.gen_range(GRID_MIN + 1..GRID_MAX);
            let y = rng.gen_range(GRID_MIN + 1..GRID_MAX);

            // Convert grid coordinates to map indices
            let map_x = (x - GRID_MIN) as usize;
            let map_y = (GRID_MAX - y) as usize;

            // Check if the tile is empty
            if map_x < MAP_WIDTH && map_y < MAP_HEIGHT && map[map_y][map_x] == 0 {
                // Ensure no duplicate interactables
                if !interactables.iter().any(|p| p.x == x && p.y == y) {
                    // For demonstration, we'll set the destination position to a random point
                    let dest_x = rng.gen_range(GRID_MIN + 1..GRID_MAX);
                    let dest_y = rng.gen_range(GRID_MIN + 1..GRID_MAX);

                    let interactable = InteractablePoint {
                        x,
                        y,
                        destination_map: None, // For now, we'll leave it as None
                        destination_position: Some((dest_x, dest_y)),
                    };

                    interactables.push(interactable);
                }
            }

            attempts += 1;
        }

        interactables
    }
