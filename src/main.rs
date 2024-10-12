mod engine;
mod closures;
use engine::roman_numbers::roman_numbers;
use engine::cons_list::create_list;
use engine::my_box::run_my_box;
use closures::closures::{Products,ShirtColor};
pub fn main() {
    // run_roman();
    // create_list();
    // run_my_box();
    give_out_shirt();
    
}

fn run_roman() {
    let roman_number = "XV";
    let result = roman_numbers(roman_number);
    println!("The roman number {} is equal to {}", roman_number, result);
    create_list();
}

fn give_out_shirt(){
    let shirts=Products{
        shirts: vec![ShirtColor::Red,ShirtColor::Blue,ShirtColor::Red,ShirtColor::Blue,ShirtColor::Blue,ShirtColor::Red,ShirtColor::Red,ShirtColor::Red]
    };
    let giveawayshirt=shirts.give_away(None);

    println!("The shirt is {:#?}",{giveawayshirt})

}
    