extern crate num_cpus;

fn main() {
    // let s: String = num_cpus::get_physical().to_string();
    // println!("{}", s);

    let logical_cpus = num_cpus::get();
    let physical_cpus = num_cpus::get_physical();
    if logical_cpus > physical_cpus {
        println!(
            "We have simultaneous multithreading with about {:.2} \
             logical cores to 1 physical core.",
            (logical_cpus as f64) / (physical_cpus as f64)
        );
    } else if logical_cpus == physical_cpus {
        println!(
            "Either we don't have simultaneous multithreading, or our \
             system doesn't support getting the number of physical CPUs."
        );
    } else {
        println!(
            "We have less logical CPUs than physical CPUs, maybe we only have access to \
             some of the CPUs on our system."
        );
    }

    println!("{}", logical_cpus.to_string());
    println!("{}", physical_cpus.to_string());
}
