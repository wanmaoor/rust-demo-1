use std::mem;
pub fn run() {
  let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
  println!("{:?}", numbers);

  // Get single val
  println!("Single value 1:{}", numbers[0]);

  // Reassign val
  numbers[0] = 11;
  println!("Single value 1:{}", numbers[0]);

  // Get array length
  println!("Array length: {}", numbers.len());

  // Arrays are stack allocated
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[0..3];
  println!("Slice: {:?}", slice);
}
