pub fn run(){
  // primitive array
  let arr1 = [1,2,3];
  let arr2 =arr1;

  // vector
  let vector1 = vec![1,2,3];
  let vector2 = &vector1;

  println!("Values: {:?}", (arr1, arr2));
  println!("Vectors: {:?}", (&vector1, vector2));
}