use std::io;

fn main() {
  let mut i = String::new();
  io::stdin().read_line(&mut i).unwrap();

  // 음수에서 패닉 발생할 수 있기 때문에 i32로 변환
  let nums: Vec<i32> = i
    .trim()
    .split_whitespace()
    .map(|x| x.parse().expect("parse err"))
    .collect();

  print!("{}", nums[0] - nums[1]);
}