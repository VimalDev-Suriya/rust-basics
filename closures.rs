pub fn closures_fn() {
    // || place where we can consume the args
    // Can be multiline statemnets and expression
    // SImilar to arrow functions
    let add = |val_1: i8, val_2: i8| {
        println!("x: {}, y: {}", val_1, val_2);
        val_1 + val_2
    };

    let result = add(4, 5);

    let print_value = || println!("The result: {result}");

    print_value();
}
