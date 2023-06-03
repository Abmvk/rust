extern crate ncurses;

use ncurses::*;
use std::convert::TryInto;

struct GameOfLife {
    field: Vec<Vec<(bool, bool)>>,
    maxX: i32,
    maxY: i32,
}

impl GameOfLife {
    fn new(maxX: i32, maxY: i32) -> Self {
        let field = vec![vec![(false, false); (maxY+2) as usize]; (maxX+2) as usize];
        GameOfLife { field, maxX, maxY }
    }

    fn set_cell(&mut self, x: i32, y: i32, alive: bool) {
        if x < 0 || x > self.maxX || y < 0 || y > self.maxY { return; }
        self.field[x as usize][y as usize] = (alive, false);
    }

    fn draw_field(&self) {
        for x in 0..=self.maxX {
            for y in 0..=self.maxY {
                let (cell, _) = self.field[x as usize][y as usize];
                mvaddch(x, y, if cell { '*' } else { ' ' });
            }
        }
        refresh();
    }

    fn calculate_new_generation(&mut self) {
        for x in 0..=self.maxX {
            for y in 0..=self.maxY {
                let mut neighbours = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        let nx = wrap_around(x + dx, self.maxX);
                        let ny = wrap_around(y + dy, self.maxY);
                        if self.field[nx as usize][ny as usize].0 { neighbours += 1; }
                    }
                }
                self.field[x as usize][y as usize].1 = match (self.field[x as usize][y as usize].0, neighbours) {
                    (true, 2) => true,
                    (_, 3) => true,
                    _ => false,
                };
            }
        }
        for x in 0..=self.maxX {
            for y in 0..=self.maxY {
                self.field[x as usize][y as usize].0 = self.field[x as usize][y as usize].1;
            }
        }
    }
}

fn wrap_around(val: i32, max: i32) -> i32 {
    if val < 0 {
        max
    } else if val > max {
        0
    } else {
        val
    }
}

fn main() {
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);
    let mut maxX = 0;
    let mut maxY = 0;
    getmaxyx(stdscr(), &mut maxY, &mut maxX);
    let mut game = GameOfLife::new((maxX-1).try_into().unwrap(), (maxY-1).try_into().unwrap());
    // setup and run the game
    endwin();
}