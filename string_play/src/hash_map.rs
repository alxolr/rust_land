use std::collections::HashMap;

pub fn _show() {
  let teams = vec![String::from("Blue"), String::from("Yellow")];
  let initial_scores = vec![10, 50];

  let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

  for (key, value) in &scores {
    println!("{}: {}", key, value);
  }

  let mut scores = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);
  scores.entry(String::from("Green")).or_insert(35);

  println!("{:?}", scores);
}

pub fn _update_value() {
  let text = "hello world wonderful wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", map);
}

pub fn integer_play() {
  let mut numbers = vec![1, 3, 5, 2, 9, 1, 8, 3, 9, 9, 13];
  let mut map = HashMap::new();
  numbers.sort();

  for item in &numbers {
    let count = map.entry(item).or_insert(0);
    *count += 1;
  }

  println!(
    "avg = {:.3}, median = {}, freq = {}",
    average(&numbers),
    median(&numbers),
    most_frequent_number(&numbers)
  )
}

fn most_frequent_number(values: &[i32]) -> i32 {
  let mut map = HashMap::new();

  for item in values {
    let count = map.entry(item).or_insert(0);
    *count += 1
  }

  let mut max_cnt = 0;
  let mut max_val = 0;

  for (key, value) in map {
    if value > max_cnt {
      max_cnt = value;
      max_val = *key;
    }
  }

  max_val
}

fn median(values: &[i32]) -> i32 {
  values[values.len() / 2]
}

fn average(values: &[i32]) -> f64 {
  let sum: f64 = sum(values);
  let size: f64 = len(values);

  sum / size
}

fn sum(values: &[i32]) -> f64 {
  let mut sum: f64 = 0.0;
  for value in values {
    sum += f64::from(*value);
  }

  sum
}

fn len(values: &[i32]) -> f64 {
  let len = values.len();

  f64::from(len as i32)
}
