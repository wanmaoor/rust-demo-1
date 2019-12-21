// rust的变量分为值类型和引用类型
// 变量默认不可修改
// rust 有块级作用域
pub fn run() {
  let name = "wanmao";
  let mut age = 22;
  println!("{}, {}", name, age);
  age = 23;
  println!("{}, {}", name, age);

  // define constant
  const ID: i32 = 007;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("wanamo", 22);
  println!("My name is {}, age is {}", my_name, my_age);
}
