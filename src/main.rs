// import module
mod print;
fn main() {
    print::run(); // invoke print's run
    println!("Number: {} {}",  1, 2); // { } is a placeholder

    // positional arguments
    println!("{0} is handsome and {0} likes {1}", "Joy", "RAP");

    // named arguments
    println!("{name} likes to {activity}", name ="Alice", activity="singing");

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug
    println!("{:?}", (12, true, "hello"));
}
