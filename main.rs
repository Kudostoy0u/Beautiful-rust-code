use std::fs;
fn parse(toparse:&str) -> i64 { toparse.parse::<i64>().expect("Failed to parse number") }
fn part1(data: &Vec<(i64,i64)>, taken: &mut Vec<i64>,index: i64, acc: i64,part2:bool) -> i64 { 
  if taken.contains(&index) {
    if !part2 {acc} else {0}
  } else {
    if data.get(index as usize) == None { return acc };
    let cidx = data[index as usize];
    taken.push(index);
    match cidx.0 {
    2 => part1(&data,taken,index+1,acc+cidx.1,part2),
    0 => part1(&data,taken,index+cidx.1,acc,part2),
    _ => part1(&data,taken,index+1,acc,part2)
    }}}
fn part2(mut data: Vec<(i64,i64)>) -> i64 {
  let datalen: usize = data.len();
  for i in 0..datalen {
    let currentelem = data[i];
    let switch: i64 = match currentelem.0 {
      0 => 1,
      1 => 0,
      _ => continue
    };
    data[i] = (switch,currentelem.1);
    let val = part1(&data, &mut Vec::new(), 0i64, 0i64, true);
    data[i] = (currentelem.0,currentelem.1);
    if val != 0 { return val }
  }return 0;}
fn main() {
  let data = fs::read_to_string("./day8.txt").unwrap().split("\n").collect::<Vec<_>>().iter().map(|x| {
    let twoparts = x.split(" ").collect::<Vec<_>>();
    let part1 = match twoparts[0] {
      "jmp" => 0,
      "nop" => 1,
      "acc" => 2,
      _ => 666
    };
    (part1,parse(twoparts[1]))
  }).collect::<Vec<_>>();
  let part1answer = part1(&data,&mut Vec::new(),0i64,0i64,false);
  let part2answer = part2(data);
  println!("Answer for Part 1: {}\nAnswer for Part 2: {}",part1answer,part2answer);
}