
// the Bytes is a very useful crate.

// it can deal with static and heap allocated memory

use bytes::Bytes;

fn main() {
    let mut mem = Bytes::from("Hello world");
    let a = mem.slice(0..5);
    println!("{?}");
    let b = mem.split_to(6);

    assert_eq!(mem, "world");
    assert_eq!(b, "Hello ");
}