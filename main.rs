pub mod helper;

fn main() {
    let full_name = helper::name_helpers::get_full_name("Suriya", "Deveraj");
    let age_plus_5 = helper::age_helpers::get_age_plus_5(26);
    let sample = helper::sample_fn();

    println!("Full Name: {0}", full_name);
    println!("Age: {age_plus_5}");
}

// Outer Attribute - Will tell the rustc to ignore the unused functions or variables
#[allow(dead_code)]
fn test() {
    // ******* FUNCTIONS AND STAEMENTS ********
    // The Code return within the () should be the expression.
    // let y = (let a = 1); // Throws error as the code within () is not the expression, its the statement

    // The Code withing the {} is the combination of statement and expression
    let y = {
        let x = 10; // (1)
        x + 1 // (2)
    };

    // (1) => Implies the statement as the variable as just assigned
    // (2) => `x+1` is the expression resolves to value `11`. Also we can see there is no semicolon
    //          that implies that block ends with expression and that value was returned.
    //          no need of return statement

    println!("Resultant value: {y}"); // result = 11
}
