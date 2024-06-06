pub mod backroom;
#[allow(dead_code)]
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: toast.to_string(),
            seasonal_fruit: String::from("mandarin"),
        }
    }
}

#[allow(dead_code)]
pub enum Appetizer {
    Soup,
    Candy,
}
