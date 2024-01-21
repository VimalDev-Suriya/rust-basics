fn main() {
    println!("Rust: Variables and Data types");

    mutability_Shadowing_fn();
    unit_fn();
    integer_fn();
    casting_fn();
    string_fn();
    tuple_fn();
    array_fn();
}

fn mutability_Shadowing_fn() {
    // MUTABILITY
    // By Default all variable are IMMUTABLE.
    let var_1 = 10;
    // var_1 = var_1 + 15; // Throws the error as the var_1 is immutable
    println!("Immutable variable var_1{}", var_1);

    let mut var_2 = 16;
    println!("Mutable variable var_2(initial): {}", var_2);
    var_2 = 100;
    println!("Mutable variable var_2(changed): {}", var_2);

    // SHADOWING
    // Still the X is immutable
    let x = 10;
    // The let keyword reassign the value with same variable name.
    let x = x + 10;

    // Seperate block
    {
        let x = x + 25;
        println!("Shadowed value withinn the block: {}", x)
    }

    println!("Shadowed value: {}", x)
}

fn unit_fn() {
    // unit type - similar to void, null. (a tuple without a values)
    // '()' is the unit data type
    let x: ();

    // Assigning the unit value to unit data type variable
    let y: () = ();

    println!("Y: {:?}", y);
}

fn integer_fn() {
    // Integer has 2 main data types signed and unsigned
    // 1. Signed - i{8, 16, 32, 64, 128, arch}
    // As we know signed will accept the both +ve and -ve value
    let signed_integer: i8 = 5;
    let signed_neg_integer: i8 = -5;

    // Below will throw error as the 8bit signed datatype will accept only -128 to 127 value
    // let signed_integer: i8 = 255;

    println!("signed_integer: {}", signed_integer);
    println!("signed_neg_integer: {}", signed_neg_integer);

    // unsigned will throw error because unsigned will not accept the -ve
    // let unsigned_integer: u8 = -5;
    let unsigned_integer: u8 = 255;

    println!("unsigned_integer: {}", unsigned_integer);
}

fn casting_fn() {
    // By default compiler takes f64;
    let f64_value = 230.0;
    let float_val: f32 = 255.0;
    let x: u8 = 5;

    // Below operation won't work because both the variabls are different data type
    // let y = float_val - x;
    // "as" keyword helps o typecast the data type
    let y = float_val as u8 - x;
    println!("Subratcted value: {}", y)
}

fn string_fn() {
    let char_value = 'A';
    let string_value = "suriya";

    println!("{}", char_value);
    println!("{}", string_value);
}

fn tuple_fn() {
    // Similar to arrays - but has the ability to store different data types.

    let tuple_value = ('A', "franklin", 1000, 40 as u8);

    // {:?} is the delimeter that prevents the cause of issue while executing the code
    // it removes the default formatterunder the hood
    println!("{:?}", tuple_value);

    // To Get value from the tuple
    // Method 1 - Destructuring
    let (a, b, c, d) = tuple_value;
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);

    // Method 2 - Accessing using the period
    // tuplevalue.{position of element}
    let a_value = tuple_value.0;
    let b_value = tuple_value.1;
    let c_value = tuple_value.2;

    println!("{}", a_value);
    println!("{}", b_value);
    println!("{}", c_value);
}

fn array_fn() {
    // Array also has the collection of data but in same data types.
    // Best - when we know the number of elements - we also have vectors (as the size can vary on time)
    // if you see return type [dat_type; length_of_array]
    // Array postions will start from 0
    let array_value = [1, 2, 3, 4, 5];
    println!("Array elements: {:?}", array_value);

    // To Get the data from the array
    let first_array_el = array_value[0];
    println!("Array element in first postion: {}", first_array_el);

    // To Slice the elements in the array.
    // [1..3] => where the elemnts wll be sliced from position 1(inclusive) to position 3 (exclusive) => only pos 1, 2
    // [1..=3] => both 1st and 3rd position are inclusive
    let sliced_elements = &array_value[1..3];
    println!("SLiced elements: {:?}", sliced_elements);

    // To create the array elemnet with length
    let new_array = [3; 5];
    println!("Array elements: {:?}", new_array);
}
