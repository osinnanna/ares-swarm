use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C"{
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Rover {
    pub x: f64, 
    pub y: f64,
    vx: f64,
    vy: f64,
}

#[wasm_bindgen]
impl Rover {
    pub fn new(x: f64, y: f64) -> Rover {
        Rover { x, y, vx: 1.0, vy: 1.0 }
    }
    pub fn tick(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        // So that rover won't pass certain boundary
        if self.x > 800.0 { self.x = 0.0; }
        if self.y > 800.0 { self.y = 0.0}
    }
}
