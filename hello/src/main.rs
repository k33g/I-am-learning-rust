
use my_tools;

fn main() {
    println!("👋 Hello, world! 🌍");
    my_tools::all_tools::big::hello();
    let r = my_tools::all_tools::small::add(1, 2);
    println!("{}", r)
}
