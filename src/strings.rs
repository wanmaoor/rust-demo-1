// 有两种string

// 第一种string: 不可修改, 长度固定
// 第二种: 可增加, 存放在堆里
pub fn run(){
  let hello = "Hello1"; //属于第一种
  let mut hello2 = String::from("Hello2"); //第二种

  println!("{} {}", hello, hello2);
  println!("Length: {}", hello.len());

  // push char
  hello2.push('W');
  println!("{}", hello2);

  // push string
  hello2.push_str("string: &str");
  println!("{}", hello2);

  // capacity in bytes
  println!("Capacity: {}", hello2.capacity());
  // check empty
  println!("Is empty: {}", hello2.is_empty());
  // contains
  println!("Contains 'hello': {}", hello2.contains("hello"));
  // replace
  println!("Replace: {}", hello2.replace("Hello", "There"));
  // loop through string by whitespace
  for word in hello2.split_whitespace() {
    println!("{}", word);
  }

  //create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  println!("s: {}", s);

  // assertion testing
  assert_eq!(2, s.len()); // show nothing if returns true
  assert_eq!(10, s.capacity());
}