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
use util::{char_to_radix, Space};

impl Sudoku {
  pub fn solve(mat: Vec<Vec<char>>) -> Option<Vec<Vec<Vec<char>>>> {
    let res = Self::helper(&mut mat.clone());
    match res.len() {
      0 => None,
      _ => Some(res),
    }
  }

  fn helper(mat: &mut Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let res = Self::reasoning(mat);
    if res.is_none() {
      return vec![];
    }

    if Self::completed(mat) {
      inspect(&mat, true);
      return vec![mat.to_vec()];
    }

    return Self::backtrack(mat);
  }

  fn reasoning(mat: &mut Vec<Vec<char>>) -> Option<Vec<Vec<char>>> {
    let len = mat.len();
    let n = len.sqrt();

    loop {
      let mut row_dim = Self::row_matrix(mat);
      let mut col_dim = Self::col_matrix(mat);
      let mut box_dim = Self::box_matrix(mat);

      let mut count = 0;
      for (i, row) in mat.into_iter().enumerate() {
        for (j, cel) in row.into_iter().enumerate() {
          if *cel == '.' {
            let k = i / n * n + j / n;
            let l = j % n * n + i % n; // box matrix offset

            let opts: Vec<_> = (1..=len)
              .into_iter()
              .map(|n| n as u8)
              .filter(|n| {
                return !row_dim[i].contains(n)
                  && !col_dim[j].contains(n)
                  && !box_dim[k].contains(n);
              })
              .collect();

            match opts.len() {
              1 => {
                count += 1;

                let ans = opts[0];

                *cel = char_to_radix(ans, len + 1);

                row_dim[i][j] = ans;
                col_dim[j][i] = ans;
                box_dim[k][l] = ans;
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
    return Some(mat.to_vec());
  }

  fn backtrack(mat: &mut Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let n = mat.len().sqrt();

    let row_dim = Self::row_matrix(mat);
    let col_dim = Self::col_matrix(mat);
    let box_dim = Self::box_matrix(mat);

    let mut spaces: Vec<Space> = vec![];

    for (i, row) in mat.iter().enumerate() {
      for (j, cel) in row.iter().enumerate() {
        if *cel == '.' {
          let k = i / n * n + j / n;
          let opts: Vec<_> = (1..=mat.len())
            .into_iter()
            .map(|n| n as u8)
            .filter(|n| {
              return !row_dim[i].contains(n) && !col_dim[j].contains(n) && !box_dim[k].contains(n);
            })
            .collect();
          spaces.push(Space {
            pos: (i, j),
            opts: opts,
          });
        }
      }
    }
    let sample = spaces.iter().min_by_key(|s| s.opts.len()).unwrap();
    let (i, j) = sample.pos;

    sample
      .opts
      .iter()
      .flat_map(|opt| {
        let mut dup = mat.clone();
        dup[i][j] = char_to_radix(*opt, mat.len() + 1);
        Self::helper(&mut dup)
      })
      .collect()

    // for &opt in &sample.opts {
    //   let mut dup = board.clone();
    //   dup[i][j] = char_to_radix(opt, board.len() + 1);
    //   let res = Self::helper(&mut dup);
    //   if !res.is_empty() {
    //     return vec![res[0].clone()];
    //   }
    // }
    // return vec![];
  }

  fn completed(mat: &Vec<Vec<char>>) -> bool {
    mat.into_iter().flatten().all(|&c| c != '.')
  }

  fn transpose(mat: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut t = vec![Vec::with_capacity(mat.len()); mat[0].len()];
    for r in mat {
      for i in 0..r.len() {
        t[i].push(r[i]);
      }
    }
    t
  }

  fn row_matrix(mat: &Vec<Vec<char>>) -> Vec<Vec<u8>> {
    mat
      .iter()
      .map(|xs| {
        xs.iter()
          .map(|&x| {
            if x == '.' {
              0
            } else {
              x.to_digit(mat.len() as u32 + 1).unwrap() as u8
            }
          })
          .collect()
      })
      .collect()
  }

  fn col_matrix(mat: &Vec<Vec<char>>) -> Vec<Vec<u8>> {
    Self::transpose(mat)
      .iter()
      .map(|xs| {
        xs.iter()
          .map(|&x| {
            if x == '.' {
              0
            } else {
              x.to_digit(mat.len() as u32 + 1).unwrap() as u8
            }
          })
          .collect()
      })
      .collect()
  }

  fn box_matrix(mat: &Vec<Vec<char>>) -> Vec<Vec<u8>> {
    let n = mat.len().sqrt();
    mat
      .chunks(n)
      .flat_map(|slice| {
        Self::transpose(&slice.to_owned())
          .chunks(n)
          .map(|chunk| chunk.to_owned())
          .collect::<Vec<_>>()
      })
      .map(|xs| {
        xs.iter()
          .flatten()
          .map(|&x| {
            if x == '.' {
              0
            } else {
              x.to_digit(mat.len() as u32 + 1).unwrap() as u8
            }
          })
          .collect()
      })
      .collect()
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
