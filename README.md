# Structs

Structs are similar to tuple, but strcuts will provide more meaningfull way to store data in `key:value` pair (ojects in JS). <br>

- To Spread the struct data in another, we use `..` similar to `spread` ops in JS

## We can debug the struct with following steps: <br>

1. Using the `:?` delimeter provides the output like below

```
println!("Struct: {:?}", struct)
// {  width: 60, height: 40, }
```

2. Using the `:#?` delimeter provides the output like below

```
println!("Struct: {:#?}", struct)
Reactangle {
    width: 60,
    height: 40
}
```

## Implementation of methods within struct

1. Implementing the method in the struct
   1. By default the method within the struct accepts the `self as the first parameter` which is something very similar to what we do for function implementation like `rectangle: &Reactangle`
   2. to pass the second param and consume the second arg

```
struct Reactangle {
    width: i32,
    height: i32,
}

impl Reactangle {
    <!-- &self shorthand property like (self: &Self) -->
    fn area(&self){
        self.width * self.height
    }

    <!--  -->
    fn compare_dimension(self: &Self, other: &Reactangle) -> bool {
        if self.width == other.width && self.height == other.height {
            return true;
        }
        false
    }
}

fn main(){
    <!-- block -->
    let rect_struct = Reactangle {
        width: 50,
        height: 70,
    };
    let rect_struct_1 = Reactangle {
        width: 50,
        height: 70,
    };

    println!("Area: {}", rect_struct.area())
    println!("is same, {}", rect_struct_1.compare_dimension(&rect_struct)) // true
}
```

2. All the methods within the struct are the `Associated function`, but there are some differences like below
   1. method `with &self` like above are the methods which we can call as method
   2. method `without &self` are like similar to constructor that initiate the params passed. (below eg)
      1. We can access all remaining methods withing the impl.

```
impl Rectangle{
    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main(){
    let square = Reactangle::square(5);

    println!("Area of Square, {}", val_2.area()); // 25
}
```

3. `&` represents that we are borrowing the value from the original heap
