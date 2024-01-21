// "mod" keyword tells that to consider the code as the modle to rustc
// Moreover its the submodule/child module for the helpers like helpers -> name_helpers
// We need to add "pub" keyword before the mod so that we can able to access this child mod.
pub mod name_helpers {
    // Assigning the "pub" keyword as the rustc will export all the function as private, since we do need to get access we are changing taht to pub
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name = format!("{0} {1}", first, last);
        return full_name;
    }
}

pub mod age_helpers {
    // Type of return statement was noted as "->"
    pub fn get_age_plus_5(age: u8) -> u8 {
        return age + 5;
    }
}

pub fn sample_fn() {
    println!("Some static fun")
}
