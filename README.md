# Refernces and Borrowing

[Click here](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) for more details. <br>

- String literals(`str`) are used to stored in stack so we can easily replace any values to that location - but we can't able to edit the existing value
- `String` stores in Heap - where the memory is allocated.
- Using `clone()` function in the `String` we can make the value stored to be mutated.
- Keeping the ownership on the mind
  - As Each value will have only one owner
  - If owner goes out of scope it is popped out (underthehood it will `drop` the values)
  - Only one owner at the time
- We can't able to pass the reference from one function to another
- Whenever the function was called, all its `args` and `local variables` will be dropped once the exceution got completed.
- `&` represents the pointer of the Heap
- `&mut` represents the mutable reference

We can't make 2 mutable reference at same time

```
<!-- Valid Code -->
let mut a = String::from("");
let b = &mut a;
let c = &mut a;
```

We can make multiple immuatble references

```
<!-- Valid Code -->
let mut a = String::from("");
let b = & a;
let c = & a;
```
