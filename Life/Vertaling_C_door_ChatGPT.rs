use std::io::{stdin, stdout, Write};
use std::convert::TryInto;

struct GameOfLife {
    field: Vec<Vec<bool>>,
    maxX: usize,
    maxY: usize,
}

impl GameOfLife {
    fn new(maxX: usize, maxY: usize) -> Self {
        let field = vec![vec![false; maxY+2]; maxX+2];
        GameOfLife { field, maxX, maxY }
    }

    fn set_cell(&mut self, x: usize, y: usize, alive: bool) {
        if x > self.maxX || y > self.maxY { return; }
        self.field[x][y] = alive;
    }

    fn draw_field(&self) {
        for x in 0..=self.maxX {
            for y in 0..=self.maxY {
                print!("{}", if self.field[x][y] { '*' } else { ' ' });
            }
            println!();
        }
    }

    fn calculate_new_generation(&mut self) {
        let mut new_field = self.field.clone();
        for x in 0..=self.maxX {
            for y in 0..=self.maxY {
                let mut neighbours = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        let nx = wrap_around(x as i32 + dx, self.maxX as i32);
                        let ny = wrap_around(y as i32 + dy, self.maxY as i32);
                        if self.field[nx][ny] { neighbours += 1; }
                    }
                }
                new_field[x][y] = match (self.field[x][y], neighbours) {
                    (true, 2) => true,
                    (_, 3) => true,
                    _ => false,
                };
            }
        }
        self.field = new_field;
    }
}

fn wrap_around(val: i32, max: i32) -> usize {
    if val < 0 {
        max as usize
    } else if val > max {
        0
    } else {
        val as usize
    }
}

fn main() {
    let mut game = GameOfLife::new(80, 24);  // replace with your desired dimensions
    game.draw_field();
    loop {
        game.calculate_new_generation();
        game.draw_field();
        stdout().flush().unwrap();
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
    }
}