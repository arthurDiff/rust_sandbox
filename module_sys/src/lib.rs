pub use crate::back_of_house::Breakfast as Lunch;
use crate::front_of_house::{
    hosting::{self, restroom},
    serving,
};

#[allow(dead_code)]
mod front_of_house {
    pub use crate::back_of_house::Breakfast as Lunch;
    fn door_person() {}
    pub mod hosting {
        pub mod restroom {}
        pub fn add_to_waitlist() {
            super::door_person();
            println!("Added something to waitlist");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Took Some Order");
        }
    }
}

mod back_of_house;

#[allow(dead_code)]
fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();
    let lunch2 = crate::front_of_house::Lunch::summer("toast");
    let breakfast = back_of_house::Breakfast::summer("Wheat");
    println!("{}", breakfast.toast);
    let lunch = Lunch::summer("toast");
}
