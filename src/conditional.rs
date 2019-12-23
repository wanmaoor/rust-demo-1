pub fn run() {
  let age = 23;
  let check_id = false;
  // if else
  if age > 18 && check_id {
    println!("I'm a adult");
  } else if age < 18 && check_id {
    println!("Little kids");
  } else {
    println!("balala");
  }

  // short if
  let is_of_age = if 10 > 8 { true } else { false };
  println!("Is of Age {}", is_of_age);
}
