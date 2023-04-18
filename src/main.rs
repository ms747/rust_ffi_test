mod consts;
mod utility;

use std::ffi::c_char;

use consts::*;
use tictactoe::{Color, KeyboardKey, MouseKey, Raylib, Rectangle, Vector2, TraceLogLevel};
use utility::*;

fn main() {
    // Game variables
    let mut board = [[BlockState::Empty; 3]; 3];
    let mut game_state = GameState::Playing;
    let o_rectangle = Rectangle::new(0.0, 0.0, 300.0, 271.0);
    let x_rectangle = Rectangle::new(300.0, 0.0, 300.0, 271.0);
    let mut x_chance = true;

    Raylib::set_trace_log(TraceLogLevel::None);
    let thread = Raylib::init(WIDTH, HEIGHT, b"tictactoe\0".as_ptr() as *const c_char);
    thread.set_target_fps(60);

    let asset = thread.load_texture("asset.png");
    // asset.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_BILINEAR); // Enable Antialiasing

    // Enable 4x Multisampling
    // unsafe { SetConfigFlags(ConfigFlags::FLAG_MSAA_4X_HINT as u32) };

    while !thread.should_window_close() && !thread.is_key_released(KeyboardKey::Q) {
        // Compute
        let brush = thread.begin_drawing();

        // Restart
        if thread.is_key_released(KeyboardKey::R) && game_state != GameState::Playing {
            reset(&mut game_state, &mut board);
        }

        // On click threadr
        if thread.is_mouse_button_released(MouseKey::Left) && game_state == GameState::Playing {
            let (col, row) = map_mouse_to_board(thread.get_mouse_position());
            if board[row][col] == BlockState::Empty {
                board[row][col] = if x_chance {
                    BlockState::X
                } else {
                    BlockState::O
                };
                x_chance = !x_chance;
            }

            if check_draw(&board) {
                game_state = GameState::Draw;
            }

            match check_winner(&board) {
                Some(BlockState::X) => game_state = GameState::XWon,
                Some(BlockState::O) => game_state = GameState::OWon,
                _ => {}
            };
        }

        // Draw
        if game_state != GameState::Playing {
            brush.clear_background(Color::RAYWHITE);

            let winner_message = match game_state {
                GameState::OWon => WIN_O,
                GameState::XWon => WIN_X,
                GameState::Draw => DRAW,
                _ => unreachable!(),
            };

            let winner_font_size =
                thread.measure_text_ex(thread.get_font_default(), WIN_O, 48.0, 1.0);
            let restart_font_size =
                thread.measure_text_ex(thread.get_font_default(), RESTART, 24.0, 1.0);

            brush.draw_text_ex(
                thread.get_font_default(),
                winner_message,
                Vector2::new(
                    (WIDTH as f32 - winner_font_size.x) / 2.0,
                    (HEIGHT as f32 - winner_font_size.y) / 2.0,
                ),
                48.0,
                1.0,
                Color::BLACK,
            );

            brush.draw_text_ex(
                thread.get_font_default(),
                "( Press R to restart )",
                Vector2::new(
                    (WIDTH as f32 - restart_font_size.x) / 2.0,
                    (HEIGHT as f32 - restart_font_size.y + 84.0) / 2.0,
                ),
                24.0,
                1.0,
                Color::BLACK,
            );
        } else {
            for r in 0..3usize {
                for c in 0..3usize {
                    let x = (c * WIDTH as usize) as f32 / 3.0;
                    let y = (r * HEIGHT as usize) as f32 / 3.0;
                    let width = WIDTH as f32 / 3.0;
                    let height = HEIGHT as f32 / 3.0;

                    if board[r][c] == BlockState::Empty {
                        brush.draw_rectangle(
                            x as i32,
                            y as i32,
                            width as i32,
                            height as i32,
                            Color::RAYWHITE,
                        );
                    } else {
                        let x_or_o = if board[r][c] == BlockState::X {
                            x_rectangle
                        } else {
                            o_rectangle
                        };

                        brush.draw_texture_pro(
                            asset,
                            x_or_o,
                            Rectangle::new(x + 25.0, y + 25.0, width - 50.0, height - 50.0),
                            Vector2::new(0.0, 0.0),
                            0.0,
                            Color::WHITE,
                        );
                    }
                }
            }

            draw_grid(&brush);
        }
    }
}
