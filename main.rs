// Defination of struct
#[derive(Debug)] // This flag will help us that we can debug this struct
struct User {
    name: String,
    age: u16,
    employed: bool,
}

// Tuple structs
#[derive(Debug)]
struct Color(i8, i8, i8);

#[derive(Debug)]
struct Reactangle {
    width: i32,
    height: i32,
}

// Implementing the methods within the Structs
impl Reactangle {
    // &self shorthand property like (self: &Self)
    fn area(&self) -> i32 {
        self.height * self.width
    }

    fn compare_dimension(self: &Self, other: &Reactangle) -> bool {
        if self.width == other.width && self.height == other.height {
            return true;
        }
        false
    }

    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Initaiting the Struct
    let user_1 = User {
        name: String::from("Suriya"),
        age: 26,
        employed: true,
    };

    println!("{:?}", user_1); // this console works only if we set this flag #[derive(Debug)]
    println!("Age: {}", user_1.age);
    println!("Name: {}", user_1.name);
    println!("employed: {}", user_1.employed);

    // Using one value of struct into another
    // `..` similar to sperad operator in JS
    // One difference is the below code prints User { name: "Dev", age: 26, employed: true }
    //      not { name: "Suriya", age: 26, employed: true }, this is not working like JS
    let user_2 = User {
        name: String::from("Dev"),
        ..user_1
    };

    println!("{:?}", user_2); // results = User { name: "Dev", age: 26, employed: true }
    println!("{:?}", user_1); // results = User { name: "Suriya", age: 26, employed: true }

    // ********* TUPLE STRUCT *************
    println!("********* TUPLE STRUCT *************");

    let black = Color(0, 0, 0);
    println!("{:?}", black); // We can access the indiviadual value as black.0

    // ************ Area Implemenation ***************
    let width = 60;
    let height = 40;

    let area = reactangle_area(width, height);

    println!("Area of the reactangle: {area}");

    // ************ Using the tuple ***************
    let rect_dimension = (60, 40);

    let area = tuple_rect_area(rect_dimension);
    println!("Area of the reactangle: {area}");

    // ************ Using the Struct ***************
    let rect_dimension_struct_value = Reactangle {
        width: 60,
        height: 40,
    };

    let area_value = struct_rect_area(rect_dimension_struct_value);
    println!("Area of the reactangle: {area_value}");

    // Below throws the error (borrow of moved value: `rect_dimension_struct`)
    //      as the value was passed into `struct_rect_area`, because of this we should use pointer
    // println!("{:?}", rect_dimension_struct)

    let react_dimension_struct = Reactangle {
        width: 60,
        height: 40,
    };

    let area_v = struct_rect_area_pointer(&react_dimension_struct);
    println!("Area of the reactangle: {area_v}");
    println!(
        "react_dimension_struct pointer eg: {:#?}",
        react_dimension_struct
    );
    println!("Pointer: {:?}", &react_dimension_struct);

    // Calling the methods in rect
    let rect_struct = Reactangle {
        width: 50,
        height: 70,
    };

    println!("Area: {}", rect_struct.area());

    let rect_struct_1 = Reactangle {
        width: 50,
        height: 70,
    };

    println!("is same, {}", rect_struct_1.compare_dimension(&rect_struct));

    // Self method
    let val_2 = Reactangle::square(4);

    println!("Area of Square, {}", val_2.area());
}

fn reactangle_area(w: i32, h: i32) -> i32 {
    w * h
}

fn tuple_rect_area(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}

fn struct_rect_area(dimension: Reactangle) -> i32 {
    dimension.height * dimension.width
}

fn struct_rect_area_pointer(dimension: &Reactangle) -> i32 {
    dimension.height * dimension.width
}
