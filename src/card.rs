use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, PartialEq, Copy, Clone)]
#[wasm_bindgen]
pub enum CardState { FaceDown, FaceUp, Removed }

#[derive(Debug, Copy, Clone)]
#[wasm_bindgen]
pub struct Card {
    pub id: u8,
    pub value: u8,
    pub state: CardState
}

#[wasm_bindgen]
impl Card {
    pub(super) fn new(id: u8, value: u8) -> Card{
        return Card {
            id,
            value,
            state: CardState::FaceDown,
        };
    }

    pub(super) fn reveal(&mut self) -> Result<(), &'static str> {
        if self.state != CardState::FaceDown {
            return Err("Card can not be flipped!")
        }

        self.state = CardState::FaceUp;
        Ok(())
    }

    pub(super) fn hide(&mut self) {
        self.state = CardState::FaceDown;
    }

    pub(super) fn remove(&mut self) {
        self.state = CardState::Removed;
    }
}
