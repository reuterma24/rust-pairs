mod card;
mod game_board;
mod util;

use wasm_bindgen::prelude::*;
use crate::game_board::GameBoard;

#[wasm_bindgen]
pub fn greet() -> String {
	if let Ok(board) = simple_manual_test() {
		return format!("Hello, {:?}", board)
	}

	format!("pech gehabt")
}


/// https://rustwasm.github.io/wasm-bindgen/reference/types/result.html
/// How to handle the return type

#[wasm_bindgen]
pub fn start(values: u8, copies: u8, players: u8) -> Result<GameBoard, String> {
	return GameBoard::new_game(values, copies, players).map_err(|err| String::from(err))
}

#[wasm_bindgen]
pub fn select_card(game_board: &mut GameBoard, index: u8) -> Result<GameBoard, String>{
	return game_board.flip_card(index).map_err(|err| String::from(err));
}


fn simple_manual_test() -> Result<GameBoard, &'static str> {
	let values = 3;
	let copies = 2;

	let res = GameBoard::new_game(values, copies, 2);

	if let Ok(mut board) = res {
		for n in 0..copies * values {
			board.flip_card(n)?;
		}

		Ok(board)
	}
	else {
		res
	}
}