use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
    Green,
}
#[derive(PartialEq)]
pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    pub fn give_away(&self, user_pref: Option<ShirtColor>) -> ShirtColor {
        user_pref.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        if self.shirts.is_empty() {
            panic!("NO ITEMS IN STOCK");
        }
        let mut stock_map: HashMap<ShirtColor, i32> = HashMap::new();
        let mut stocked: ShirtColor = self.shirts[0];
        for shirt in self.shirts.iter() {
            let value = *stock_map.entry(*shirt).and_modify(|v| *v += 1).or_insert(1);
            let prev_value = stock_map.get(&stocked).unwrap_or(&0);
            if prev_value < &value {
                stocked = *shirt
            }
        }
        stocked
    }
}
