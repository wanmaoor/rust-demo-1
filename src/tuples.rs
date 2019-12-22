// tuples group together values of different types
// max 12 elements
pub fn run(){
  let person: (&str, &str, i8) = ("wanmao", "loves", 12);
  println!("{} {} basketball {}", person.0, person.1, person.2);
}