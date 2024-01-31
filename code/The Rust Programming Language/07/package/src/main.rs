use garden::vegetables::Asparagus;
use rand::Rng;

pub mod garden;
fn main() {
    // let plant: Asparagus = Asparagus {};
    // println!("I'm growing {:?} !", plant);
    // eat_at_restaurant();
    let secret_number = rand::thread_rng().gen_range(0..100);
    println!("{}", secret_number)
}
