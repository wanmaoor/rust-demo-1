use std::env;
pub fn run(){
  let args: Vec<String> = env::args().collect();
  let command = args[1].clone();
  let name = "aW";
  let status = "100%";
  // println!("command: {}", command);
  if command == "hello" {
    println!("Hi {}, How are you", name);
  } else if command == "status" {
    println!("Status is {}", status);
  } else {
    println!("That is not a valid command");
  }
}