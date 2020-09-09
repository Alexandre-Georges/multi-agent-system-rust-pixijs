use wasm_bindgen::prelude::*;
// use web_sys::console;
use random::Source;

const CELL_SIZE: f64 = 10.0;

#[wasm_bindgen(module = "pixi.js")]
extern {
  pub type Container;

  #[wasm_bindgen(method, getter)]
  fn renderer(this: &Graphics) -> Renderer;

  #[wasm_bindgen(method)]
  fn height(this: &Container) -> i32;

  #[wasm_bindgen(method)]
  fn width(this: &Container) -> i32;

  #[wasm_bindgen(method)]
  fn addChild(this: &Container, child: &Graphics);
}

#[wasm_bindgen(module = "pixi.js")]
extern {
  pub type Graphics;

  #[wasm_bindgen(constructor)]
  fn new() -> Graphics;

  #[wasm_bindgen(method)]
  fn clear(this: &Graphics);

  #[wasm_bindgen(method)]
  fn beginFill(this: &Graphics, color: i32);

  #[wasm_bindgen(method)]
  fn drawRect(this: &Graphics, x: i32, y: i32, width: i32, height: i32);

  #[wasm_bindgen(method)]
  fn endFill(this: &Graphics);
}

#[wasm_bindgen(module = "pixi.js")]
extern {
  pub type Renderer;

  #[wasm_bindgen(method)]
  fn resize(this: &Renderer, width: i32, height: i32);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  Dead = 0,
  Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
  pub width: u32,
  height: u32,
  cells: Vec<Cell>,
  graphics: Graphics,
}

#[wasm_bindgen]
impl Universe {
  pub fn new(width: u32, height: u32, alive_odds: f64, container: Container, renderer: Renderer) -> Universe {
    let d = js_sys::Date::now();
    let mut rng = random::default().seed([d as u64, 0]);

    // console::log_1(&JsValue::from_f64(self.height as f64 * CELL_SIZE));

    let cells = (0..width * height)
      .map(|_| {
        let random: f64 = rng.read::<f64>();
        if random < alive_odds {
          Cell::Alive
        } else {
          Cell::Dead
        }
      })
      .collect();

    let graphics = Graphics::new();
    renderer.resize(width as i32 * CELL_SIZE as i32, height as i32 * CELL_SIZE as i32);
    container.addChild(&graphics);

    Universe {
      width,
      height,
      cells,
      graphics,
    }
  }

  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;
    for delta_row in [self.height - 1, 0, 1].iter().cloned() {
      for delta_col in [self.width - 1, 0, 1].iter().cloned() {
        if delta_row == 0 && delta_col == 0 {
          continue;
        }

        let neighbor_row = (row + delta_row) % self.height;
        let neighbor_col = (column + delta_col) % self.width;
        let idx = self.get_index(neighbor_row, neighbor_col);
        count += self.cells[idx] as u8;
      }
    }
    count
  }

  pub fn tick(&mut self) {
    let mut next = self.cells.clone();

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);
        let cell = self.cells[idx];
        let live_neighbors = self.live_neighbor_count(row, col);

        let next_cell = match (cell, live_neighbors) {
          (Cell::Alive, x) if x < 2 => Cell::Dead,
          (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
          (Cell::Alive, x) if x > 3 => Cell::Dead,
          (Cell::Dead, 3) => Cell::Alive,
          (otherwise, _) => otherwise,
        };

        next[idx] = next_cell;
      }
    }

    self.cells = next;
  }

  pub fn render(&mut self) {
    self.graphics.clear();
    self.graphics.beginFill(0x000000);

    for (index, cell) in self.cells.iter().enumerate() {
      if *cell == Cell::Alive {
        let x = index as u32 % self.width;
        let y = index as u32 / self.width;

        self.graphics.drawRect(
          x as i32 * CELL_SIZE as i32,
          y as i32 * CELL_SIZE as i32,
          CELL_SIZE as i32,
          CELL_SIZE as i32,
        );

      }
    }
    self.graphics.endFill();
  }

  pub fn click(&mut self, x: u32, y: u32) {
    let index = (x + y * self.width) as usize;
    let cell = self.cells[index];
    let mut next_cell = Cell::Alive;
    if cell == Cell::Alive {
      next_cell = Cell::Dead;
    }
    self.cells[index] = next_cell;
  }

  pub fn get_cell_size(&self) -> f64 {
    CELL_SIZE
  }
}
