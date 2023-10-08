#[allow(dead_code)]
mod game_board;
#[allow(dead_code)]
mod card;

fn main() ->  Result<(), &'static str>  {
    let res = game_board::GameBoard::new_game(3, 2, 2);

    if let Ok(mut board) = res {
        // 1st match - player 0
        board.flip_card(0)?;
        board.flip_card(1)?;

        // fail - player 0
        board.flip_card(2)?;
        board.flip_card(4)?;

        // 1st match - player 1
        board.flip_card(2)?;
        board.flip_card(3)?;

        // 2nd match - player2
        board.flip_card(4)?;
        board.flip_card(5)?;

        println!("{:?}", board)
    }

    Ok(())
}

// TODO:
// - shuffle cards
// - do not allow flipping removed or already flipped cards
// - game finished logic
//
// - restart game ???