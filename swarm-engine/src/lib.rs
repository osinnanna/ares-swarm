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
pub struct Swarm {
    rovers: Vec<Rover>,
    width: f64,
    height: f64,
}

#[wasm_bindgen]
impl Swarm {
    pub fn new(count: usize, width: f64, height: f64) -> Swarm {
        console_error_panic_hook::set_once();

        let mut rovers = Vec::with_capacity(count);
        for _ in 0..count {
            rovers.push(Rover {
                x: width / 2.0,
                y: height / 2.0,
                vx: (js_sys::Math::random() - 0.5) * 4.0,
                vy: (js_sys::Math::random() - 0.5) * 4.0,
            });
        }
        Swarm { rovers, width, height }
    }
    pub fn tick(&mut self) {
        for rover in self.rovers.iter_mut() {
            rover.x += rover.vx;
            rover.y += rover.vy;

            // So that rover won't pass certain boundary
            if rover.x <= 0.0 { rover.x = 0.0; rover.vx *= -1.0; }
            else if rover.x >= self.width { rover.x = 800.0; rover.vx *= -1.0; }

            if rover.y <= 0.0 { rover.y = 0.0; rover.vy *= -1.0; }
            else if rover.y >= self.height { rover.y = 800.0; rover.vy *= -1.0; }
        }
    }

    pub fn get_rover_x(&self, index: usize) -> f64 { self.rovers[index].x }
    pub fn get_rover_y(&self, index: usize) -> f64 { self.rovers[index].y }
    pub fn count(&self) -> usize { self.rovers.len() }
}