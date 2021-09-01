pub struct Board {
  data: [[Tile; 9]; 9],
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BoardError {
  msg: String,
}

impl fmt::Display for BoardError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl Error for BoardError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

pub type Tile = Option<u32>;
pub type TilePointer = (usize, usize);
pub type Row = [Tile; Board::SIZE];
impl Board {
  pub const SIZE: usize = 9;

  pub fn new(input_data: Vec<Vec<Tile>>) -> Result<Board, BoardError> {
    if input_data.len() != Self::SIZE {
      return Err(BoardError {
        msg: "Invalid row count".to_string(),
      });
    }

    let mut data: [Row; Self::SIZE] = [[None; Self::SIZE]; Self::SIZE];

    for (i, row) in input_data.iter().enumerate() {
      if row.len() != Self::SIZE {
        return Err(BoardError {
          msg: "Invalid columns count".to_string(),
        });
      }

      for (j, tile) in row.iter().enumerate() {
        data[i][j] = tile.to_owned();
      }
    }

    Ok(Board { data })
  }

  pub fn get_tile(&self, ptr: TilePointer) -> Tile {
    let (x, y) = ptr;
    self.data[y][x]
  }

  pub fn set_tile(&mut self, ptr: TilePointer, value: Tile) {
    let (x, y) = ptr;
    self.data[y][x] = value;
  }

  pub fn clone(&self) -> Self {
    Board { data: self.data }
  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string: String = self
      .data
      .iter()
      .map(|row| {
        row
          .iter()
          .map(|field| match field {
            Some(num) => num.to_string(),
            None => "-".to_string(),
          })
          .collect::<String>()
          + "\n"
      })
      .collect::<String>();
    write!(f, "{}", string)
  }
}
