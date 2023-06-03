use std::collections::HashSet;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

type Cell = (i32, i32);
type Cells = HashSet<Cell>;

fn get_neighbours(&(x, y): &Cell) -> Vec<Cell> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn next(cells: &Cells) -> Cells {
    let mut neighbour_counts = Cells::new();

    for cell in cells.iter().flat_map(get_neighbours) {
        if !neighbour_counts.contains(&cell) {
            neighbour_counts.insert(cell);
        }
    }

    neighbour_counts
        .into_iter()
        .filter(|cell| {
            let count = get_neighbours(cell)
                .into_iter()
                .filter(|neighbour| cells.contains(neighbour))
                .count();

            count == 3 || (count == 2 && cells.contains(cell))
        })
        .collect()
}

fn main() {
    let mut cells = [
        (0, 0),
        (1, 0),
        (2, 0),
        (0, 1),
        (1, 2),
    ]
    .iter()
    .cloned()
    .collect::<Cells>();

    loop {
        let _ = io::stdout().write(b"\x1b[2J\x1b[H");
        let min_x = cells.iter().map(|&(x, _)| x).min().unwrap();
        let min_y = cells.iter().map(|&(_, y)| y).min().unwrap();
        let max_x = cells.iter().map(|&(x, _)| x).max().unwrap();
        let max_y = cells.iter().map(|&(_, y)| y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!("{}", if cells.contains(&(x, y)) { 'O' } else { ' ' });
            }

            println!();
        }

        cells = next(&cells);
        sleep(Duration::from_millis(100));
    }
}
