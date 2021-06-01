// get argv
use std::env;
// files
use std::fs::File;
use std::io::prelude::*;

use std::time::Duration;

mod board;

use board::Board;
type Error = Box<dyn std::error::Error>;
fn main() {
  println!("Solving!\n");

  match &env::args().collect::<Vec<String>>()[..] {
    [_, input] => match run(input, false) {
      Ok(_) => println!("Done!"),
      Err(msg) => panic!("{}", msg),
    },
    [_, input, _] => match run(input, true) {
      Ok(_) => println!("Done!"),
      Err(msg) => panic!("{}", msg),
    },
    _ => println!("Invalid arguments"),
  }
}

fn run(path_to_input: &str, verbose: bool) -> Result<(), Error> {
  let input_string = load_input(&path_to_input)?;
  let board = parse_board(&input_string)?;

  println!("{}", board);

  let start = std::time::Instant::now();
  let solved = solve(&board, verbose)?;

  if !verbose {
    println!("{}", solved);
    println!("Time taken: {} μs", start.elapsed().as_micros());
  };

  Ok(())
}

fn load_input(path: &str) -> Result<String, Error> {
  let mut file = File::open(path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  Ok(contents)
}

fn parse_board(input_string: &str) -> Result<Board, Error> {
  let rows = input_string
    .split('\n')
    .map(|row| row.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

  let board = board::Board::new(
    rows
      .iter()
      .map(|row| row.iter().map(|ch| ch.to_digit(10)).collect())
      .collect(),
  )?;

  Ok(board)
}

fn solve(board: &Board, verbose: bool) -> Result<Board, Error> {
  let mut board = board.clone();

  let mut empty_fields: Vec<board::TilePointer> = vec![];

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

  if solve_loop(&mut board, &empty_fields, 0, &mut 0, verbose) {
    if verbose {
      reprint(&board);
    }
    Ok(board)
  } else {
    Err("Unsolvable".into())
  }
}

fn reprint(board: &Board) {
  // for _ in 0..Board::SIZE {
  //   println!("\x1b[1K\r \x1b[A")
  // }
  print!("{esc}c", esc = 27 as char);
  println!("{}", board);
}

fn solve_loop(
  board: &mut Board,
  empty_fields: &[board::TilePointer],
  current_index: usize,
  iteration: &mut u32,
  verbose: bool,
) -> bool {
  // println!("\n{}\n{}", current_index, board);
  if verbose {
    *iteration += 1;
    if *iteration % 10 == 0 {
      std::thread::sleep(Duration::from_millis(50));
      reprint(&board);
    }
  }
  if current_index == empty_fields.len() {
    return true;
  }

  let current = empty_fields[current_index];

  let possible_values: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
  for value in possible_values.iter() {
    board.set_tile(current, Some(value.to_owned()));
    // println!("{:?} {:?} {}", current, value, is_valid(&board, &current));
    if is_valid(&board, &current) {
      if solve_loop(board, empty_fields, current_index + 1, iteration, verbose) {
        return true;
      } else {
        board.set_tile(current, None);
      }
    }
  }
  board.set_tile(current, None);

  false
}

fn is_valid(board: &Board, last_play: &board::TilePointer) -> bool {
  let mut seen: bool;
  let (x, y) = last_play;
  let last_play_value = board.get_tile(*last_play);
  let mut current: board::TilePointer;

  // row
  current = (0, *y);
  seen = false;
  for _ in 0..Board::SIZE {
    // println!("{:?}", current);
    // println!("{:?} {:?}", board.get_tile(current), last_play_value);
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
    // println!("{:?}", current);
    // println!("{:?} {:?}", board.get_tile(current), last_play_value);
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
      // println!("{:?}", current);
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
