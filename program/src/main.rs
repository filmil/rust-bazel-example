/// This one lame rust program
use bumpalo::Bump;
use std::u64;

mod some;

#[allow(dead_code)]
struct Doggo {
    cuteness: u64,
    age: u8,
    scritches_required: bool,
}

fn main() {
    println!("hello world!");

    // Here is how to call a function from a mod.
    some::some();

    // The code below has been taken from the bumpalo examples, just to show
    // how one includes a dependency into a rust binary built with bazel.

    // Create a new arena to bump allocate into.
    let bump = Bump::new();

    // Allocate values into the arena.
    let scooter = bump.alloc(Doggo {
        cuteness: u64::max_value(),
        age: 8,
        scritches_required: true,
    });

    // Exclusive, mutable references to the just-allocated value are returned.
    assert!(scooter.scritches_required);
    scooter.age += 1;

    println!("bump: {:?}", &bump);
}
