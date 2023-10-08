use dev_martin::CheckedMultiplication;
use crate::card::Card;

#[derive(Debug)]
pub(super) struct GameBoard {
    value_count: u8, /// Number of unique values on the board (like pictures).
    copies_per_value: u8,
    player_count: u8,
    pub cards: Vec<Card>,
    pub selected_card_indices : Box<[u8]>, /// Stores the currently selected cards during a players turn.
    player_score: Box<[u8]>,
    active_player: usize,
    turn_count: usize
}

impl GameBoard {
    pub(super) fn new_game(values: u8, copies: u8, players: u8) -> Result<Self, &'static str>{
        let card_count = values.checked_multiply(copies)?;

        let mut cards : Vec<Card> = Vec::new();
        for i in 0..card_count {
           cards.push(Card::new(i,  i / copies)) // integer division
        }

        let game_board = GameBoard {
            value_count: values,
            copies_per_value: copies,
            player_count: players,
            cards,
            selected_card_indices: Box::from(vec![0; usize::from(players)]),
            player_score: Box::from(vec![0; usize::from(players)]),
            active_player: 0,
            turn_count: 0,
        };

        return Ok(game_board)
    }

    pub(super) fn flip_card(&mut self, index: u8) -> Result<(), &'static str> {
        if let Some(card) = self.cards.get_mut(usize::from(index)) {
            card.reveal();
            self.selected_card_indices[self.turn_count] = index;
            self.turn_count += 1;
        }
        else {
            return Err("Could not retrieve a card for given index");
        }

        // game logic
        if self.turn_count == self.copies_per_value as usize {
            if self.check_flipped_cards() {
                self.increase_player_score();
                self.remove_flipped_cards();
            }
            else {
                self.next_player();
                self.hide_flipped_cards();
            }
            self.turn_count = 0
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
        if self.active_player == self.player_count as usize {
            self.active_player = 0
        }
        else {
            self.active_player += 1
        }
    }
}