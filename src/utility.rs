use crate::{HEIGHT, WIDTH};
use tictactoe::{Brush, Color, Vector2};

#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
pub enum BlockState {
    X,
    O,
    Empty,
}

#[repr(i32)]
#[derive(PartialEq, Copy, Clone)]
pub enum GameState {
    OWon = -1,
    Playing = 0,
    XWon = 1,
    Draw = 2,
}

pub fn map_mouse_to_board(mouse_position: Vector2) -> (usize, usize) {
    let x = (mouse_position.x / WIDTH as f32 * 3.0).floor() as usize;
    let y = (mouse_position.y / HEIGHT as f32 * 3.0).floor() as usize;
    (x, y)
}

pub fn check_winner(board: &[[BlockState; 3]; 3]) -> Option<BlockState> {
    for r in 0..3 {
        if board[r][0] != BlockState::Empty
            && board[r][0] == board[r][1]
            && board[r][0] == board[r][2]
        {
            return Some(board[r][0]);
        }
    }

    for c in 0..3 {
        if board[0][c] != BlockState::Empty
            && board[0][c] == board[1][c]
            && board[0][c] == board[2][c]
        {
            return Some(board[0][c]);
        }
    }

    if board[0][0] != BlockState::Empty && board[0][0] == board[1][1] && board[0][0] == board[2][2]
    {
        return Some(board[0][0]);
    }

    if board[0][2] != BlockState::Empty && board[0][2] == board[1][1] && board[0][2] == board[2][0]
    {
        return Some(board[0][2]);
    }

    None
}

pub fn draw_grid(brush: &Brush) {
    brush.draw_line_ex(
        Vector2::new(WIDTH as f32 / 3.0, 0.0),
        Vector2::new(WIDTH as f32 / 3.0, HEIGHT as f32),
        2.0,
        Color::BLACK,
    );
    brush.draw_line_ex(
        Vector2::new(WIDTH as f32 / 3. * 2.0, 0.0),
        Vector2::new(WIDTH as f32 / 3. * 2.0, HEIGHT as f32),
        2.0,
        Color::BLACK,
    );
    brush.draw_line_ex(
        Vector2::new(0., HEIGHT as f32 / 3.0),
        Vector2::new(WIDTH as f32, HEIGHT as f32 / 3.0),
        2.0,
        Color::BLACK,
    );
    brush.draw_line_ex(
        Vector2::new(0., HEIGHT as f32 / 3.0 * 2.0),
        Vector2::new(WIDTH as f32, HEIGHT as f32 / 3.0 * 2.0),
        2.0,
        Color::BLACK,
    );
}

pub fn check_draw(board: &[[BlockState; 3]; 3]) -> bool {
    let mut count = 0;

    for r in 0..3 {
        for c in 0..3 {
            if board[r][c] != BlockState::Empty {
                count += 1;
            }
        }
    }

    count == 9
}

pub fn reset(game_state: &mut GameState, board: &mut [[BlockState; 3]; 3]) {
    *game_state = GameState::Playing;

    for r in 0..3 {
        for c in 0..3 {
            board[r][c] = BlockState::Empty;
        }
    }
}
