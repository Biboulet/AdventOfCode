
use std::{env, time::Instant};
mod utils;
mod exercice1;
mod exercice2;
mod exercice3;
mod exercice4;
mod exercice5;
mod exercice6;
//mod exercice7;
mod exercice8;
mod exercice9;
mod exercice10;

fn main() {
    let args: Vec<String> = env::args().collect();  
    let exercice_index: u32 = args[1].parse().unwrap();
    let input = utils::get_input();
    let current_time = Instant::now(); 
    match exercice_index {
        1 => println!("{:?}", exercice1::compute_results(input)),
        2 => println!("{:?}", exercice2::compute_results(input)),
        3 => println!("{:?}", exercice3::compute_results(input)),
        4 => println!("{:?}", exercice4::compute_results(input)),
        5 => println!("{:?}", exercice5::compute_results(input)),
        6 => println!("{:?}", exercice6::compute_results(input)),
        7 => println!("{:?}", dbg!("Flemme de réparer ca")),
        8 => println!("{:?}", exercice8::compute_results(input)),
        9 => println!("{:?}", exercice9::compute_results()),
        10 => println!("{:?}", exercice10::compute_results(input)),
        _ => panic!("Pas d'exercice") 
    }
    println!("Computed in {} secs", current_time.elapsed().as_secs_f64());

}
