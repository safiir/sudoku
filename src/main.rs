extern crate colored;
extern crate num;
extern crate radix;

pub mod util;

use colored::{Color, Colorize};
use num::integer::Roots;

#[allow(unused_imports)]
use std::{
  io::{stdout, Write},
  thread,
  time::{Duration, Instant},
  vec,
};
use util::char_to_radix;

impl Sudoku {
  pub fn solve(mat: Vec<Vec<char>>) -> Option<Vec<Vec<Vec<char>>>> {
    let mut board = mat.clone();
    let (mut row_dim, mut col_dim, mut box_dim) = Self::masks(&board);
    let res = Self::helper(&mut board, &mut row_dim, &mut col_dim, &mut box_dim);
    match res.len() {
      0 => None,
      _ => Some(res),
    }
  }

  fn helper(
    mat: &mut Vec<Vec<char>>,
    row_dim: &mut Vec<u16>,
    col_dim: &mut Vec<u16>,
    box_dim: &mut Vec<u16>,
  ) -> Vec<Vec<Vec<char>>> {
    if Self::reasoning(mat, row_dim, col_dim, box_dim).is_none() {
      return vec![];
    }

    if Self::completed(mat) {
      inspect(&mat, true);
      return vec![mat.to_vec()];
    }

    return Self::backtrack(mat, row_dim, col_dim, box_dim);
  }

  fn reasoning(
    mat: &mut Vec<Vec<char>>,
    row_dim: &mut Vec<u16>,
    col_dim: &mut Vec<u16>,
    box_dim: &mut Vec<u16>,
  ) -> Option<()> {
    let len = mat.len();
    let n = len.sqrt();
    let full = ((1u32 << len) - 1) as u16;

    loop {
      let mut count = 0;
      for i in 0..len {
        for j in 0..len {
          if mat[i][j] == '.' {
            let k = i / n * n + j / n;
            let avail = full & !(row_dim[i] | col_dim[j] | box_dim[k]);

            match avail.count_ones() {
              1 => {
                count += 1;

                let ans = avail.trailing_zeros() as u8 + 1;
                mat[i][j] = char_to_radix(ans, len + 1);

                let flag = 1u16 << (ans - 1);
                row_dim[i] |= flag;
                col_dim[j] |= flag;
                box_dim[k] |= flag;
              }
              0 => {
                return None;
              }
              _ => {}
            }
          }
        }
      }
      if count == 0 {
        break;
      }
    }
    return Some(());
  }

  fn backtrack(
    mat: &mut Vec<Vec<char>>,
    row_dim: &mut Vec<u16>,
    col_dim: &mut Vec<u16>,
    box_dim: &mut Vec<u16>,
  ) -> Vec<Vec<Vec<char>>> {
    let len = mat.len();
    let n = len.sqrt();
    let full = ((1u32 << len) - 1) as u16;

    // MRV: pick the empty cell with the fewest candidates.
    let mut best: Option<(usize, usize, u16)> = None;
    let mut best_count = u32::MAX;
    for i in 0..len {
      for j in 0..len {
        if mat[i][j] == '.' {
          let k = i / n * n + j / n;
          let avail = full & !(row_dim[i] | col_dim[j] | box_dim[k]);
          let count = avail.count_ones();
          if count < best_count {
            best_count = count;
            best = Some((i, j, avail));
          }
        }
      }
    }

    let (i, j, avail) = best.unwrap();
    let k = i / n * n + j / n;

    let mut results = vec![];
    let mut bits = avail;
    while bits != 0 {
      let flag = bits & bits.wrapping_neg(); // lowest set bit
      bits &= bits - 1;
      let ans = flag.trailing_zeros() as u8 + 1;

      let mut dup = mat.clone();
      let mut dup_row = row_dim.clone();
      let mut dup_col = col_dim.clone();
      let mut dup_box = box_dim.clone();

      dup[i][j] = char_to_radix(ans, len + 1);
      dup_row[i] |= flag;
      dup_col[j] |= flag;
      dup_box[k] |= flag;

      results.extend(Self::helper(
        &mut dup,
        &mut dup_row,
        &mut dup_col,
        &mut dup_box,
      ));
    }
    results
  }

  fn completed(mat: &Vec<Vec<char>>) -> bool {
    mat.into_iter().flatten().all(|&c| c != '.')
  }

  // Build row / column / box bitmasks in a single pass over the board.
  fn masks(mat: &Vec<Vec<char>>) -> (Vec<u16>, Vec<u16>, Vec<u16>) {
    let len = mat.len();
    let n = len.sqrt();
    let radix = len as u32 + 1;

    let mut row = vec![0u16; len];
    let mut col = vec![0u16; len];
    let mut bx = vec![0u16; len];

    for i in 0..len {
      for j in 0..len {
        if let Some(d) = mat[i][j].to_digit(radix) {
          let flag = 1u16 << (d - 1);
          let k = i / n * n + j / n;
          row[i] |= flag;
          col[j] |= flag;
          bx[k] |= flag;
        }
      }
    }
    (row, col, bx)
  }
}

const BORDER_COLOR: Color = Color::TrueColor {
  r: 100,
  g: 100,
  b: 160,
};

fn inspect(mat: &Vec<Vec<char>>, colorize: bool) {
  let n = mat.len().sqrt();
  let colors = vec![
    Color::BrightRed,
    Color::BrightGreen,
    Color::BrightYellow,
    Color::BrightMagenta,
    Color::BrightCyan,
  ];

  let block = "↔ ".repeat(n).to_string();
  let delimiter = format!("\n{}\n", vec![block; n].to_vec().join("+ "))
    .color(BORDER_COLOR)
    .to_string();

  let output = mat
    .iter()
    .enumerate()
    .map(|(i, xs)| {
      xs.iter()
        .enumerate()
        .map(|(j, &c)| {
          let k = i / n * n + j / n;

          match colorize {
            true => {
              let color = if c == '.' {
                Color::Black
              } else {
                colors[k % colors.len()]
              };
              c.to_string()
                .to_uppercase()
                .color(color)
                .bold()
                .italic()
                .to_string()
            }
            false => c.to_string().to_uppercase(),
          }
        })
        .collect::<Vec<String>>()
        .chunks(n)
        .map(|chunk| chunk.join(" "))
        .collect::<Vec<_>>()
        .join(&format!(" ↕ ").color(BORDER_COLOR).to_string())
    })
    .collect::<Vec<_>>()
    .chunks(n)
    .map(|chunk| chunk.join("\n"))
    .collect::<Vec<_>>()
    .join(&delimiter);
  println!("\x1B[2J\x1B[1;1H{}\n", output);
  stdout().flush().unwrap();
  // thread::sleep(Duration::from_millis(10))
}

struct Sudoku;

fn main() {
  let mats = [
    "8........",
    "..36.....",
    ".7..9.2..",
    ".5...7...",
    "....457..",
    "...1...3.",
    "..1....68",
    "..85...1.",
    ".9....4..",
  ]
  .iter()
  .map(|xs| xs.chars().collect::<Vec<_>>())
  .collect::<Vec<_>>();

  for mat in [mats] {
    let start = Instant::now();
    match Sudoku::solve(mat) {
      Some(answers) => {
        let duration = start.elapsed();
        println!(
          "{}",
          format!("{} answer found, spend {:?}\n", answers.len(), duration)
            .color(Color::BrightGreen)
            .bold()
            .italic()
        );
      }
      None => {
        let duration = start.elapsed();
        println!(
          "{}",
          format!("Oops! No answer, spend {:?}\n", duration)
            .color(Color::BrightRed)
            .bold()
            .italic()
        );
      }
    }
  }
}
