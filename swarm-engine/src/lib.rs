use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
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
#[derive()]
pub struct Swarm {
    rovers: Vec<Rover>,
    width: f64,
    height: f64,
    grid: Vec<u8>,
    cols: usize,
}

#[wasm_bindgen]
impl Swarm {
    pub fn new(count: usize, width: f64, height: f64) -> Swarm {
        console_error_panic_hook::set_once();
        let cols = (width / 10.0) as usize;
        let rows = (height / 10.0) as usize;
        let grid = vec![0; cols * rows];

        let mut rovers = Vec::with_capacity(count);
        for _ in 0..count {
            rovers.push(Rover {
                x: width / 2.0,
                y: height / 2.0,
                vx: (js_sys::Math::random() - 0.5) * 4.0,
                vy: (js_sys::Math::random() - 0.5) * 4.0,
            });
        }
        Swarm {
            rovers,
            width,
            height,
            grid,
            cols,
        }
    }
    pub fn tick(&mut self) {
        let cols = self.cols;
        for rover in self.rovers.iter_mut() {
            rover.x += rover.vx;
            rover.y += rover.vy;

            // So that rover won't pass certain boundary
            if rover.x <= 0.0 {
                rover.x = 0.0;
                rover.vx *= -1.0;
            } else if rover.x >= self.width {
                rover.x = self.width;
                rover.vx *= -1.0;
            }

            if rover.y <= 0.0 {
                rover.y = 0.0;
                rover.vy *= -1.0;
            } else if rover.y >= self.height {
                rover.y = self.height;
                rover.vy *= -1.0;
            }

            let gx = (rover.x / 10.0) as usize;
            let gy = (rover.y / 10.0) as usize;
            let idx = gy * cols + gx;

            if idx < self.grid.len() {
                self.grid[idx] = 1;
            }
        }
    }

    pub fn get_rover_x(&self, index: usize) -> f64 {
        self.rovers[index].x
    }
    pub fn get_rover_y(&self, index: usize) -> f64 {
        self.rovers[index].y
    }
    pub fn count(&self) -> usize {
        self.rovers.len()
    }
    // fn mark_mapped(&mut self, x: f64, y: f64) {
    //     let gx = (x / 10.0) as usize;
    //     let gy = (y / 10.0) as usize;
    //     let idx = gy * self.cols + gx;
    //     if idx < self.grid.len() {
    //         self.grid[idx] = 1;
    //     }
    // MOVED TO INSIDE TICK FUNCTION WILL DEAL WITH RUST LIFETIMES AND BORROW CHECKER
    // }
    pub fn is_cell_mapped(&self, x: usize, y: usize) -> bool {
        self.grid[y * self.cols + x] == 1
    }
    pub fn get_mapped_percentage(&self) -> f64 {
        let mapped_count = self.grid.iter().filter(|&&cell| cell == 1).count();
        (mapped_count as f64 / self.grid.len() as f64) * 100.0
    }
}
