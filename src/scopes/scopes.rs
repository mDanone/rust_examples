mod front_of_house {
    pub mod hosting {
        pub fn add_to_whitelist(){}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant(){
        super::hosting::add_to_whitelist();
    }
}