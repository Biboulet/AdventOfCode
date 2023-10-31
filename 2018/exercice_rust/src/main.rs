
use std::env;
mod utils;
mod exercice1;
mod exercice2;
mod exercice3;
mod exercice4;
mod exercice5;

fn main() {
    let args: Vec<String> = env::args().collect();  
    let exercice_index: u32 = args[1].parse().unwrap();
    let input = utils::get_input();
    match exercice_index {
        1 => println!("{:?}", exercice1::compute_results(input)),
        2 => println!("{:?}", exercice2::compute_results(input)),
        3 => println!("{:?}", exercice3::compute_results(input)),
        4 => println!("{:?}", exercice4::compute_results(input)),
        5 => println!("{:?}", exercice5::compute_results(input)),
        _ => panic!("Pas d'exercice")
        
    }
}
