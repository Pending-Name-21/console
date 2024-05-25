use nannou::prelude::*;

const GRID_SIZE: usize = 3;
const CELL_SIZE: f32 = 100.0;

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub player: Option<Player>,
}

pub struct Model {
    pub grid: [[Cell; GRID_SIZE]; GRID_SIZE],
    pub current_player: Player,
    pub winner: Option<Player>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            grid: [[Cell { player: None }; GRID_SIZE]; GRID_SIZE],
            current_player: Player::X,
            winner: None,
        }
    }

    pub fn reset(&mut self) {
        self.grid = [[Cell { player: None }; GRID_SIZE]; GRID_SIZE];
        self.current_player = Player::X;
        self.winner = None;
    }

    pub fn check_winner(&mut self) {
        let lines = [
            // Rows
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            // Columns
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            // Diagonals
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        for line in &lines {
            let [a, b, c] = *line;
            if self.grid[a.0][a.1].player.is_some()
                && self.grid[a.0][a.1].player == self.grid[b.0][b.1].player
                && self.grid[b.0][b.1].player == self.grid[c.0][c.1].player
            {
                self.winner = self.grid[a.0][a.1].player;
                return;
            }
        }
    }

    pub fn play(&mut self, row: usize, col: usize) {
        if self.grid[row][col].player.is_none() && self.winner.is_none() {
            self.grid[row][col].player = Some(self.current_player);
            self.check_winner();
            self.current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }
    }
}

pub fn model(app: &App) -> Model {
    app.new_window()
        .size(
            (CELL_SIZE * GRID_SIZE as f32) as u32,
            (CELL_SIZE * GRID_SIZE as f32) as u32,
        )
        .view(view)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();
    Model::new()
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    // Draw grid
    for i in 0..=GRID_SIZE {
        let offset = i as f32 * CELL_SIZE - (CELL_SIZE * GRID_SIZE as f32) / 2.0;
        draw.line()
            .start(pt2(offset, -CELL_SIZE * GRID_SIZE as f32 / 2.0))
            .end(pt2(offset, CELL_SIZE * GRID_SIZE as f32 / 2.0))
            .color(BLACK);
        draw.line()
            .start(pt2(-CELL_SIZE * GRID_SIZE as f32 / 2.0, offset))
            .end(pt2(CELL_SIZE * GRID_SIZE as f32 / 2.0, offset))
            .color(BLACK);
    }

    // Draw X and O
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            let cell = model.grid[row][col];
            let x = col as f32 * CELL_SIZE - (CELL_SIZE * GRID_SIZE as f32) / 2.0 + CELL_SIZE / 2.0;
            let y = row as f32 * CELL_SIZE - (CELL_SIZE * GRID_SIZE as f32) / 2.0 + CELL_SIZE / 2.0;

            match cell.player {
                Some(Player::X) => {
                    draw.line()
                        .start(pt2(x - 20.0, y - 20.0))
                        .end(pt2(x + 20.0, y + 20.0))
                        .color(BLACK);
                    draw.line()
                        .start(pt2(x + 20.0, y - 20.0))
                        .end(pt2(x - 20.0, y + 20.0))
                        .color(BLACK);
                }
                Some(Player::O) => {
                    draw.ellipse()
                        .x_y(x, y)
                        .radius(20.0)
                        .stroke(BLACK)
                        .no_fill();
                }
                None => {}
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

pub fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    let win = app.window_rect();
    if let Some(pos) = app.mouse.position().into() {
        if win.contains(pos) {
            let col = ((pos.x + CELL_SIZE * GRID_SIZE as f32 / 2.0) / CELL_SIZE).floor() as usize;
            let row = ((pos.y + CELL_SIZE * GRID_SIZE as f32 / 2.0) / CELL_SIZE).floor() as usize;
            if row < GRID_SIZE && col < GRID_SIZE {
                model.play(row, col);
            }
        }
    }
}
