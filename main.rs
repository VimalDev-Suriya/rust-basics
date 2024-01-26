pub mod branching;

fn main() {
    branching::if_block_mod::get_driver_licence_status();

    let val_1 = 10;
    // We know that anything between "{}" is the expression and that returns the value
    let val_2 = if val_1 == 10 { 5 } else { 10 };

    println!("{val_2}");

    branching::loop_mod::loop_fn();

    let a = [1, 2, 3, 4];
    branching::loop_mod::for_in_loop(a)
}
