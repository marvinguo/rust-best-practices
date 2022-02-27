struct Test(i32);

trait Hello {
    fn hello(&self) {
        println!("hello");
    }
}

impl Hello for Test {}

fn main() {
    let test1 = Box::new(Test(1));
    /* test1 memory layout

    stack test1(usize)
  -------------------------------
    heap  Test(1)
   */

    // use Box::leak get the raw ref and it is static lifetime
    let static_ref: &mut Test = Box::leak(test1);

    let test2: Box<dyn Hello> = Box::new(Test(2));

    /* test2 memory layout, it's a fat pointer

        stack  test2(usize,usize)
      -------------------------------
        heap   Test(2)
    */
}