extern crate num_cpus;

fn main() {
    let s: String = num_cpus::get_physical().to_string();
    println!("{}", s);

}