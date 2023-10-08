use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	fn getRandomNumber() -> f64;
}

#[wasm_bindgen]
pub struct Game {
	elements: Vec<u32>,
}

#[wasm_bindgen]
impl Game {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Game {
		let mut elements = Vec::new();

		for i in 1..17 {
			elements.push(i);
			elements.push(i);
		}
		shuffle(&mut elements);

		Game{ elements }
	}

	#[wasm_bindgen]
	pub fn pick_pair(&mut self, index1: usize, index2: usize) -> i32 {
		// catch invalid inputs
		if index1 > 31 || index2 > 31 || index1 == index2 || self.elements[index1] == 0 || self.elements[index2] == 0 {
			return -1;
		}

		if self.elements[index1] == self.elements[index2] {
			self.elements[index1] = 0;
			self.elements[index2] = 0;
			return 1;
		}
		0
	}
	#[wasm_bindgen]
	pub fn pick_one(&mut self, index: usize) -> u32 {
		self.elements[index]
	}
}


fn shuffle<T>(array: &mut [T]) {
	let mut i = array.len();
	while i > 1 {
		i -= 1;
		let j = (getRandomNumber() * (i as f64 + 1.0)) as usize;
		array.swap(i, j);
	}
}