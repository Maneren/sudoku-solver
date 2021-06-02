// get argv
use std::env;
// files
use std::fs::File;
use std::io::prelude::*;

mod board;
use board::{Board, TilePointer};

type Error = Box<dyn std::error::Error>;
fn main() {
  println!("Solving!\n");

  match &env::args().collect::<Vec<String>>()[..] {
    [_, input] => match run(input) {
      Ok(_) => println!("Done!"),
      Err(msg) => println!("Error: {}", msg),
    },
    _ => println!("Invalid arguments"),
  }
}

fn run(path_to_input: &str) -> Result<(), Error> {
  let input_string = load_input(&path_to_input)?;
  let board = parse_board(&input_string)?;

  println!("{}", board);

  let start = std::time::Instant::now();

  let solved = solve(&board)?;

  let run_time = start.elapsed().as_micros();

  println!("{}", solved);
  if run_time < 20000 {
    println!("Time taken: {} μs", run_time);
  } else {
    println!("Time taken: {} ms", run_time / 1000);
  }

  Ok(())
}

fn load_input(path: &str) -> Result<String, Error> {
  let mut file = File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  Ok(contents)
}

fn parse_board(input_string: &str) -> Result<Board, Error> {
  // split string into Vec<Vec<chars>>
  let rows = input_string
    .split('\n')
    .map(|row| row.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

  // parse Vec<Vec<char>> into Vec<Vec<Tile>>
  let parsed_data = rows
    .iter()
    .map(|row| row.iter().map(|ch| ch.to_digit(10)).collect())
    .collect();

  let board = Board::new(parsed_data)?;

  Ok(board)
}

fn solve(board: &Board) -> Result<Board, Error> {
  let mut board = board.clone();

  let mut empty_fields: Vec<TilePointer> = vec![];

  for y in 0..Board::SIZE {
    for x in 0..Board::SIZE {
      let tile = board.get_tile((x, y));
      if tile == None {
        empty_fields.push((x, y));
      }
    }
  }

  if empty_fields.is_empty() {
    return Ok(board);
  }

  if solve_loop(&mut board, &empty_fields, 0) {
    Ok(board)
  } else {
    Err("Unsolvable".into())
  }
}

fn solve_loop(board: &mut Board, empty_fields: &[TilePointer], current_index: usize) -> bool {
  if current_index == empty_fields.len() {
    // we correctly filled all tiles
    return true;
  }

  let current = empty_fields[current_index];

  let possible_values: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  for value in possible_values.iter() {
    board.set_tile(current, Some(value.to_owned()));
    if is_valid(&board, &current) {
      // true if boards leads to solution, false if dead end
      if solve_loop(board, empty_fields, current_index + 1) {
        return true;
      } else {
        board.set_tile(current, None);
      }
    }
  }
  // backtrack
  board.set_tile(current, None);
  false
}

fn is_valid(board: &Board, last_play: &TilePointer) -> bool {
  let mut seen: bool;
  let (x, y) = last_play;
  let last_play_value = board.get_tile(*last_play);
  let mut current: TilePointer;

  // row
  current = (0, *y);
  seen = false;
  for _ in 0..Board::SIZE {
    if board.get_tile(current) == last_play_value {
      if seen {
        return false;
      } else {
        seen = true;
      }
    }
    current.0 += 1;
  }

  // collumn
  current = (*x, 0);
  seen = false;
  for _ in 0..Board::SIZE {
    if board.get_tile(current) == last_play_value {
      if seen {
        return false;
      } else {
        seen = true;
      }
    }
    current.1 += 1;
  }

  // box
  let box_x = x - x % 3;
  let box_y = y - y % 3;
  current = (box_x, box_y);
  seen = false;

  for _ in 0..3 {
    for _ in 0..3 {
      if board.get_tile(current) == last_play_value {
        if seen {
          return false;
        } else {
          seen = true;
        }
      }
      current.0 += 1;
    }
    current.0 = box_x;
    current.1 += 1;
  }

  true
}
