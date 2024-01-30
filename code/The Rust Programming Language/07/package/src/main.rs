use garden::vegetables::Asparagus;
use package::eat_at_restaurant;

pub mod garden;
fn main() {
    let plant: Asparagus = Asparagus {};
    println!("I'm growing {:?} !", plant);
    eat_at_restaurant();
}
