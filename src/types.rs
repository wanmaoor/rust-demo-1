// 值类型
// 整数, 浮点数, 布尔值, 字符(char), 元组, 数组

// 
pub fn run(){
  // find max  size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // bool
  let is_actived = true;
  let is_greater = 10 > 5;
  // char
  let face = '\u{1F600}';

  println!("{:?}",(is_actived, is_greater, face));
}