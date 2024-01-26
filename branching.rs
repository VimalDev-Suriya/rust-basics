pub mod if_block_mod {
    pub fn get_driver_licence_status() {
        const AGE: u8 = 18;
        let mut input_age = String::new();

        println!("Please enter your input");

        std::io::stdin()
            .read_line(&mut input_age)
            .expect("Please enter valid age");

        // We need to assign the value with data type or else it will throw error
        // as the parse needs to understand to which the input data needs to be parsed
        let value: u8 = input_age
            .trim()
            .parse()
            .expect("Issue occured while parsing");

        // If block always accept the Boolean condition - not like JS
        if value >= AGE {
            println!("You are eligible to get licence")
        }
    }
}

pub mod loop_mod {
    pub fn loop_fn() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 6 {
                break counter;
            };
        };

        println!("Loop statement: {result}")
    }

    pub fn for_in_loop(arr: [i32; 4]) {
        println!("{:?}", arr);

        for ele in arr {
            println!("the elemantes in the arr: {ele}")
        }
    }
}
