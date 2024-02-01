use separate_modules::coder;
use separate_modules::front_of_house;
fn main() {
    front_of_house::hosting::add_to_waitlist();
    coder::backend::write_rust();
    coder::frontend::write_react();
}
