use std::io;

fn main() {
  let mut i = String::new();
  io::stdin().read_line(&mut i).unwrap();

  let nums: Vec<u32> = i
    .trim()
    .split_whitespace()
    .map(|x| x.parse().expect("parse err"))
    .collect();

  print!("{}", nums[0] + nums[1]);
}