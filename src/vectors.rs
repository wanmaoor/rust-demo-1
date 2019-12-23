// vectors are resizable arrays
use std::mem;
pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
  println!("{:?}", numbers);

  // Get single val
  println!("Single value 1:{}", numbers[0]);

  // Reassign val
  numbers[0] = 11;
  println!("Single value 1:{}", numbers[0]);

  // Get vectors length
  println!("vectors length: {}", numbers.len());

  // vectors are stack allocated
  println!("vectors occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[0..3];
  println!("Slice: {:?}", slice);

  // add on to vector
  numbers.push(5);
  numbers.push(6);
  numbers.pop();
  println!("Push & pop{:?}", numbers);

  // loop through vector
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // loop & mutate each values
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("numbers Vectors{:?}", numbers);
}
