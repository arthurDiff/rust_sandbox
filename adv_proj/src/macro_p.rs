use proc_macro;

#[macro_export]
macro_rules! vec {
    // "," = comma seperated, * = zero or more
    ($($x: expr), *) =>{
        {
            let mut tmp_vec = Vec::new();
            $(
                tmp_vec.push($x);
            )*
            tmp_vec
        }
    }
}

pub trait HelloMacro {
    fn hello_macro();
}
