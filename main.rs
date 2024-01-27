fn main() {
    // String literal
    let var_1 = "string literal";
    let var_ls = var_1; // Copying the value is fast and easy in stack

    println!("{var_ls} {var_1}");

    // I can't able to mutate the variable eventhough its the mutable, because its string literal and how the value stored in the memory(stack)
    // I can assign the new value but can't able to alter the existing value
    // var_1.push()

    // String - amount of memory allocated is unknown during compile time (because of Heap)
    // String -> stores value in Heap
    // I can alter the value in heap not in stack
    let mut var_2 = String::from("hello");

    var_2.push('a'); // to push char
    var_2.push_str("world"); //to push string

    println!("{var_2}"); // result= helloaworld

    // Passing the value and cloning
    let var_3 = String::from("test");
    let var_4 = var_3;

    // Here "var_3" value passed (Moved) its value to "var_4" and the rust removes the var_3
    // println!("Passed value: {var_4} {var_3}"); // Throws error
    println!("{var_4}");

    // To perform the deep copy
    let var_5 = String::from("test_2");
    let var_6 = var_5.clone();

    println!("Passed value: {var_5} {var_6}");

    // ********** POINTERS ***************
    println!("********** POINTERS ***************");
    pointer_fn();

    // ********** Mutable References ***************
    println!("********** Mutable References ***************");
    mutable_ref();
}

fn pointer_fn() {
    // Passing the value
    let str_1 = String::from("string_pointer");
    let (s, length) = calculate_length(str_1);

    println!("String pointer length: {:?}", length);

    // Passing the reference (pointer)
    let str_2 = String::from("string_value_pointer");
    let str_length = calculate_length_ptr(&str_2);

    println!("String pointer length pointer : {:?}", str_length);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    // (s, s.len()) // This won't work here as the tuple values
    // like s => was assigned directly to tuple's first value
    // s.len() after that s does not have any value since it was passed to tuple's first value
    (s, length) // I am passing the tuple here
}

fn calculate_length_ptr(s: &String) -> usize {
    s.len()
}

fn mutable_ref() {
    let mut str = String::from("hello");

    // Comment out either one of the two code
    // Commented out because we are passing the entire str to concat_str_mut
    // let concant_str = concat_str_mut(str);
    // println!("Mutable String : {concant_str}");

    // Mutable string reference: mutable ref can be declared as "&mut"
    let concant_str_ptr = concat_str_mut_ref(&mut str);

    println!("Mutable String ref: {concant_str_ptr}");
}

// fn concat_str_mut(mut s: String) -> String {
//     s.push_str("hello");
//     s
// }

fn concat_str_mut_ref(s: &mut String) -> String {
    s.push_str("added the string");

    s.to_string()
}
