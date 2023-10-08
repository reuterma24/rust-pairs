#[derive(Debug, Copy, Clone)]
enum CardState {FaceDown, FaceUp, Removed}

#[derive(Debug, Copy, Clone)]
pub(super) struct Card {
    id: u8,
    pub(super) value: u8,
    state: CardState
}

impl Card {
    pub(super) fn new(id: u8, value: u8) -> Card{
        return Card {
            id,
            value,
            state: CardState::FaceDown,
        };
    }

    pub(super) fn reveal(&mut self) {
        //TODO: ensure card is not flipped already
        self.state = CardState::FaceUp;
    }

    pub(super) fn hide(&mut self) {
        self.state = CardState::FaceDown;
    }

    pub(super) fn remove(&mut self) {
        self.state = CardState::Removed;
    }
}
