use crate::util::CheckedMultiplication;
use crate::card::Card;
use rand::seq::SliceRandom;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug)]
#[wasm_bindgen]
pub struct GameBoard {
    value_count: u8, /// Number of unique values on the board (like pictures).
    copies_per_value: u8,
    player_count: u8,
    cards: Vec<Card>,
    selected_card_indices : Box<[u8]>, /// Stores the currently selected cards during a players turn.
    player_score: Box<[u8]>,
    pub active_player: usize,
    turn_count: usize,
    pub game_finished: bool
}

// custom getters for WASM
#[wasm_bindgen]
impl GameBoard {
    pub fn player_score(&self) -> js_sys::Uint8Array {
        return js_sys::Uint8Array::from(&self.player_score[..])
    }

    pub fn cards(&self) -> js_sys::Array {
        //clone und into_iter ist scheinbar nötig, weil JsValue::from nicht auf Referenzen (&) klappt
        self.cards.clone().into_iter().map(JsValue::from).collect()
    }
}


impl GameBoard {
    pub(super) fn new_game(values: u8, copies: u8, players: u8) -> Result<Self, &'static str>{
        let card_count = values.checked_multiply(copies)?;

        let mut cards : Vec<Card> = Vec::new();
        for i in 0..card_count {
           cards.push(Card::new(i,  i / copies)) // integer division
        }

        cards.shuffle(&mut rand::thread_rng());

        let game_board = GameBoard {
            value_count: values,
            copies_per_value: copies,
            player_count: players,
            cards,
            selected_card_indices: Box::from(vec![0; usize::from(copies)]),
            player_score: Box::from(vec![0; usize::from(players)]),
            active_player: 0,
            turn_count: 0,
            game_finished: false
        };

        return Ok(game_board)
    }

    pub(super) fn flip_card(&mut self, index: u8) -> Result<(), &'static str> {
        self.assure_game_active()?;

        // logic to flip and remember the selected card
        if let Some(card) = self.cards.get_mut(usize::from(index)){
            card.reveal()?;
            self.selected_card_indices[self.turn_count] = index;
            self.turn_count += 1;
        }
        else {
            return Err("Could not retrieve a card for given index!");
        }

        // logic to verify the current players selection of cards
        // and increase counters
        if self.turn_count == self.copies_per_value as usize {
            if self.check_flipped_cards() {
                self.increase_player_score();
                self.remove_flipped_cards();
            }
            else {
                self.next_player();
                self.hide_flipped_cards();
            }
            self.reset_turn_count();
        }

        // logic to check if all cards have been flipped
        if self.player_score.iter().sum::<u8>() == self.value_count {
            self.game_finished();
        }

        Ok(())
    }

    fn assure_game_active(&self) -> Result<(), &'static str>{
        if self.game_finished{
            return Err("game is already finished!");
        }

        Ok(())
    }

    fn check_flipped_cards(&self) -> bool{
        for index in 0..self.selected_card_indices.len() - 1  {
            if let Some(c1) = self.cards.get(self.selected_card_indices[index] as usize){
                if let Some(c2) =  self.cards.get(self.selected_card_indices[index + 1] as usize){
                    if c1.value != c2.value{
                        return false
                    }
                }
            }
        }

        return true
    }

    fn remove_flipped_cards(&mut self){
        for (_, val) in self.selected_card_indices.iter().enumerate() {
            if let Some(card) = self.cards.get_mut(*val as usize){
                card.remove()
            }
        }
    }

    fn hide_flipped_cards(&mut self){
        for (_, val) in self.selected_card_indices.iter().enumerate() {
            if let Some(card) = self.cards.get_mut(*val as usize){
                card.hide()
            }
        }
    }

    fn increase_player_score(&mut self){
        self.player_score[self.active_player] += 1;
    }

    fn next_player(&mut self){
        if self.active_player == (self.player_count - 1) as usize {
            self.active_player = 0
        }
        else {
            self.active_player += 1
        }
    }

    fn game_finished(&mut self){
        self.game_finished = true
    }

    fn reset_turn_count(&mut self) {
        self.turn_count = 0
    }
}