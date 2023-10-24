extern crate ncurses;

use ncurses::*;
use std::ptr;
use std::ffi::CString;
use std::thread::sleep;
use std::time::Duration;

const CELL_VELD: usize = 0;
const COPIE_VELD: usize = 1;

struct GameOfLife {
    veld: Vec<Vec<Vec<bool>>,
    maxX: i32,
    maxY: i32,
}

impl GameOfLife {
    fn new(maxX: i32, maxY: i32) -> GameOfLife {
        GameOfLife {
            veld: vec![vec![vec![false; 2]; (maxY + 2) as usize]; (maxX + 2) as usize],
            maxX,
            maxY,
        }
    }
}

fn is_getal(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

fn cell_set(game: &GameOfLife, x: i32, y: i32, aan: bool) {
    if x >= 0 && x <= game.maxX && y >= 0 && y <= game.maxY {
        if aan {
            mvaddch(x, y, ' ' as chtype | A_REVERSE());
        } else {
            mvaddch(x, y, ' ' as chtype);
        }
    }
}

fn teken_veld(game: &GameOfLife) {
    for x in 0..=game.maxX {
        for y in 0..=game.maxY {
            let cel = game.veld[x as usize][y as usize][CELL_VELD];
            cell_set(game, x, y, cel);
        }
    }
    refresh();
}

fn maak_veld(game: &mut GameOfLife) {
    let mut x = game.maxX / 2;
    let mut y = game.maxY / 2;
    let mut ch;
    let mut stoppen = false;
    let mut teken = 0;

    while !stoppen {
        teken_veld(game);
        move(x, y);
        if game.veld[x as usize][y as usize][CELL_VELD] {
            printw("#");
        } else {
            printw("_");
        }
        move(x, y);
        ch = getch();

        match ch {
            't' => teken = 1,
            'g' => teken = -1,
            'b' => teken = 0,
            'q' if x > 0 => x -= 1,
            'a' if x < game.maxX => x += 1,
            'o' if y > 0 => y -= 1,
            'p' if y < game.maxY => y += 1,
            ' ' => game.veld[x as usize][y as usize][CELL_VELD] = !game.veld[x as usize][y as usize][CELL_VELD],
            '\n' => stoppen = true,
            _ => {}
        }
        if teken == 1 {
            game.veld[x as usize][y as usize][CELL_VELD] = true;
        }
        if teken == -1 {
            game.veld[x as usize][y as usize][CELL_VELD] = false;
        }
    }
}

fn bereken_nieuwe_generatie(game: &mut GameOfLife) {
    for x in 0..=game.maxX {
        for y in 0..=game.maxY {
            let mut buren = 0;
            for i in x - 1..=x + 1 {
                for j in y - 1..=y + 1 {
                    if i == x && j == y {
                        continue;
                    }
                    let xb = if i < 0 {
                        game.maxX
                    } else if i > game.maxX {
                        0
                    } else {
                        i
                    };
                    let yb = if j < 0 {
                        game.maxY
                    } else if j > game.maxY {
                        0
                    } else {
                        j
                    };
                    if game.veld[xb as usize][yb as usize][CELL_VELD] {
                        buren += 1;
                    }
                }
            }
            if game.veld[x as usize][y as usize][CELL_VELD] && buren == 2 {
                game.veld[x as usize][y as usize][COPIE_VELD] = true;
            }
            if buren > 3 || buren < 2 {
                game.veld[x as usize][y as usize][COPIE_VELD] = false;
            }
            if buren == 3 {
                game.veld[x as usize][y as usize][COPIE_VELD] = true;
            }
        }
    }
    for x in 0..=game.maxX {
        for y in 0..=game.maxY {
            game.veld[x as usize][y as usize][CELL_VELD] = game.veld[x as usize][y as usize][COPIE_VELD];
        }
    }
}

fn main() {
    initscr();
    cbreak();
    noecho();
    keypad(stdscr(), true);
    start_color();
    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    attron(COLOR_PAIR(1));
    clear();
    refresh();

    let args: Vec<String> = std::env::args().collect();
    let mut aantal = 1000;

    if args.len() > 1 && is_getal(&args[1]) {
        aantal = args[1].parse().unwrap();
    }

    let (maxY, maxX) = {
        let mut x = 0;
        let mut y = 0;
        getmaxyx(stdscr(), &mut y, &mut x);
        (y - 1, x - 1)
    };

    let mut game = GameOfLife::new(maxX, maxY);

    printw(
        "Welkom bij Conway's Game of Life.\n\n\
        Het 'spel' speelt zich af op een oneindig veld van cellen die leven of niet leven. \
        Oneindigheid wordt benaderd doordat het hele scherm (of window)\nvan de terminal wordt gebruikt, \
        en doordat het overloopt. \
        Dat betekent dat de cel helemaal links grenst aan de cel helemaal rechts, en net zo \nde cellen helemaal boven en onderaan het scherm.\n\n\
        In eerste instantie kan je cellen aan en uit zetten. De cursor kan je daartoe verplaatsen met de q, a, o en p-toetsen. \
        Met de spatiebalk zet je de \ncel van de cursor aan of uit. Je kan ook met t aangeven dat je altijd wil tekenen, met g dat je altijd wil gummen, \
        en met b dat je weer een \nblanco-functie wil. Als je klaar bent druk je op enter.\n\n\
        Daarna zal het programma zelf steeds nieuwe generaties berekenen en afbeelden. Als je achter life een aantal hebt opgegeven \
        dan wordt dat \naantal generaties uitgevoerd. Als je niets hebt opgegeven worden 1.000 generaties uitgevoerd. \
        Als je geen toets indrukt, pauzeert het \nprogramma tussen twee generaties. Druk je dan op f, dan wordt niet meer gepauzeerd.\
        \n\n\nEr worden {} generaties berekend.    \
        \n\n\nDruk op een toets als je dit hebt begrepen.\n\n\n",
        aantal
    );

    getch();
    clear();

    maak_veld(&mut game);

    for _ in 0..aantal {
        bereken_nieuwe_generatie(&mut game);
        teken_veld(&game);
        mv(0, 0);
        printw!("{}", aantal);
        refresh();
        sleep(Duration::from_millis(100));
    }

    mv(0, 0);
    printw("Dat waren alle generaties");
    getch();
    clear();
    refresh();
    endwin();
}
