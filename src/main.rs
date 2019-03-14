extern crate termion;

/// A Rust implementation of John Conway's Game of Life

use rand::Rng;
use std::thread;
use std::time::Duration;
use termion::{clear, cursor, color,style, raw::IntoRawMode, async_stdin};
use std::io::{Write, Read, stdout};


const ALIVE: char = '#';
const WIDTH: usize = 40;
const HEIGHT: usize = 20;

type World = [[u8; WIDTH]; HEIGHT];

/// Representation of current game state
struct State {
    generation: u32,
    living: u32,
    world: World,
}

/// Render the state to the terminal
fn render(state: &State) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    write!(stdout,"{}", cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();
    write!(stdout,"╔═══════════════════════════════════════╗\r\n").unwrap();


    for x in 0..HEIGHT - 1 {
        write!(stdout,"║").unwrap();
        for y in 0..WIDTH - 1 {
            let state = state.world[x][y];
            match state {
                1 => write!(stdout,"{}", ALIVE).unwrap(),
                _ => write!(stdout," ").unwrap(),
            }
        }
        write!(stdout,"║\r\n").unwrap();
    }
    write!(stdout,"╚═══════════════════════════════════════╝\r\n").unwrap();
    write!(stdout,"{}Generation:{} , Alive: {}{}\r\n", color::Fg(color::Blue),state.generation, state.living, style::Reset).unwrap();
    write!(stdout,"{}(q)uit (p)ause {}\r\n", color::Fg(color::Green), style::Reset).unwrap();
    write!(stdout,"{}An implementation of the Game of Life, by smeag0l{}\r\n", color::Fg(color::Cyan), style::Reset).unwrap();
    stdout.flush().unwrap();
}

/// Update the game state
fn update(state: &State) -> State {
    let mut new_state = State {
        generation: state.generation + 1,
        living: state.living,
        world: state.world,
    };
    for x in 0..HEIGHT - 1 {
        for y in 0..WIDTH - 1 {
            let alive = if state.world[x][y] == 1 { true } else { false };
            let alive_neighbours = alive_neighbours(x, y, &state.world);

            if alive && alive_neighbours < 2 {
                new_state.world[x][y] = 0;
                new_state.living -= 1;
            } else if alive && alive_neighbours > 3 {
                new_state.world[x][y] = 0;
                new_state.living -= 1;
            } else if !alive && alive_neighbours == 3 {
                new_state.world[x][y] = 1;
                new_state.living += 1;
            }
        }
    }
    new_state
}

/// Find the number of adjacent living cells to a given cell
fn alive_neighbours(row: usize, col: usize, world: &World) -> u8 {
    let mut alive_neighbours: u8 = 0;

    // top left

    if row > 0 && col > 0 {
        alive_neighbours += world[row - 1][col - 1];
    }

    // top

    if row > 0 {
        alive_neighbours += world[row - 1][col];
    }

    // top right

    if row > 0 && col < WIDTH - 1 {
        alive_neighbours += world[row - 1][col + 1];
    }

    // left

    if col > 0 {
        alive_neighbours += world[row][col - 1];
    }

    // right

    if col < HEIGHT - 1 {
        alive_neighbours += world[row][col + 1];
    }

    // bottom left
    if row < HEIGHT - 1 && col > 0 {
        alive_neighbours += world[row + 1][col - 1];
    }

    // bottom
    if row < HEIGHT - 1 {
        alive_neighbours += world[row + 1][col];
    }

    // bottom right
    if row < HEIGHT - 1 && col < WIDTH - 1 {
        alive_neighbours += world[row + 1][col + 1];
    }

    alive_neighbours
}

/// Initialise the game state
/// 
/// Creates a new initial game state,
/// randomising the initial living cells

fn init() -> State {
    let world: World = [[0; WIDTH]; HEIGHT];
    let generation = 0;
    let living = 0;
    let mut state = State {
        generation: generation,
        living: living,
        world: world,
    };

    for x in 0..HEIGHT - 1 {
        for y in 0..WIDTH - 1 {
            let r = rand::thread_rng().gen_range(0, 100);
            if r > 80 {
                state.world[x][y] = 1;
                state.living += 1;
            } else {
                state.world[x][y] = 0;
            }
        }
    }
    state
}


fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();
    let mut state = init();
    let mut paused:bool = false;
    write!(stdout,"{}{}{}",cursor::Hide, clear::All, cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();
    loop {

        if ! paused {
            render(&state);
            if state.living <= 0 || state.generation == std::u32::MAX {
                break;
            }
    
            state = update(&state);
        }

        // Poll for input
        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            write!(stdout,"{}", cursor::Show).unwrap();
            break;
        } else if let Some(Ok(b'p')) = b {
            paused = if paused {false} else {true};
        }
         
        
         thread::sleep(Duration::from_millis(300));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialise_test_state() -> State {
        let initial_state = State {
            generation: 0,
            living: 0,
            world: [[0; WIDTH]; HEIGHT],
        };
        initial_state
    }

    /// Test update with no neighbours
    ///
    /// Assert that a cell with no neighbours dies, as if by solitude
    ///
    /// e.g.:
    ///
    /// #
    ///
    #[test]
    fn test_update_no_neighbours() {
        let mut initial_state = initialise_test_state();
        initial_state.world[1][1] = 1;
        initial_state.living = 1;

        let new_state = update(&initial_state);

        assert_eq!(new_state.living, 0);
    }

    /// Test update with one neighbour
    ///
    /// Assert that a cell with one neighbour dies, as if by solitude
    ///
    /// e.g.:
    ///
    /// ##
    ///
    #[test]
    fn test_update_one_neighbour() {
        let mut initial_state = initialise_test_state();

        initial_state.world[1][1] = 1;
        initial_state.world[1][2] = 1;
        initial_state.living = 2;

        let new_state = update(&initial_state);

        assert_eq!(new_state.living, 0);
    }

    /// Test update with two neighbour
    ///
    /// Assert that cell with two neighbours survives
    /// Assert that cell with three neighbours becomes populated
    ///
    /// e.g.:
    /// 
    /// ##  -> ##
    /// #      ##

    #[test]
    fn test_update_two_neighbours() {
        let mut initial_state = initialise_test_state();

        initial_state.world[1][1] = 1;
        initial_state.world[1][2] = 1;
        initial_state.world[2][1] = 1;
        initial_state.living = 3;

        let new_state = update(&initial_state);

        assert_eq!(new_state.living, 4);
        assert_eq!(new_state.world[1][1], 1);
        assert_eq!(new_state.world[1][2], 1);
        assert_eq!(new_state.world[2][1], 1);
        assert_eq!(new_state.world[2][2], 1);
    }

    /// Test update with three neighbours
    ///
    /// Assert that cell with three neighbours survives
    ///
    ///
    /// e.g.:
    /// 
    /// ##  -> ##
    /// ##     ##

    #[test]
    fn test_update_three_neighbours() {
        let mut initial_state = initialise_test_state();

        initial_state.world[1][1] = 1;
        initial_state.world[1][2] = 1;
        initial_state.world[2][1] = 1;
        initial_state.world[2][2] = 1;
        initial_state.living = 4;

        let new_state = update(&initial_state);

        assert_eq!(new_state.living, 4);
        assert_eq!(new_state.world[1][1], 1);
        assert_eq!(new_state.world[1][2], 1);
        assert_eq!(new_state.world[2][1], 1);
        assert_eq!(new_state.world[2][2], 1);
    }

    /// Test update with four neighbours
    ///
    /// Assert that cell with four neighbours dies as if by overpopulation
    ///
    ///
    /// e.g.:
    /// 
    ///  #  ->  ###
    /// ###     # #
    ///  #      ###
    ///
    #[test]
    fn test_update_four_neighbours() {
        let mut initial_state = initialise_test_state();

        initial_state.world[1][2] = 1;
        initial_state.world[2][1] = 1;
        initial_state.world[2][2] = 1;
        initial_state.world[2][3] = 1;
        initial_state.world[3][2] = 1;
        initial_state.living = 5;

        let new_state = update(&initial_state);

        assert_eq!(new_state.living, 8);
        assert_eq!(new_state.world[1][1], 1);
        assert_eq!(new_state.world[1][2], 1);
        assert_eq!(new_state.world[1][3], 1);
        assert_eq!(new_state.world[2][1], 1);
        assert_eq!(new_state.world[2][3], 1);
        assert_eq!(new_state.world[3][1], 1);
        assert_eq!(new_state.world[3][2], 1);
        assert_eq!(new_state.world[3][3], 1);
    }
}
