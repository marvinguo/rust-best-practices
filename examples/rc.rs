use std::rc::Rc;

struct Test(i32);

trait Hello {
    fn hello(&self) {
        println!("hello");
    }
}

impl Hello for Test {}

fn main() {
    let test1 = Rc::new(Test(1));
    /* test1 memory layout

    stack test1(usize)
  -------------------------------
    heap  Test(1)
   */

    let test2: Rc<dyn Hello> = Rc::new(Test(2));

    /* test2 memory layout, it's a fat pointer

        stack  test2(usize,usize)
      -------------------------------
        heap   Test(2)
    */
}