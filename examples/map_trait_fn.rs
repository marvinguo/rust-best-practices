use std::result::Result;

type MyResult = Result<(), ()>;

trait MapToResult {
    fn test(self) -> i32;
}

impl MapToResult for () {
    fn test(self) -> i32 {
        123
    }
}

fn main() {
    let r = MyResult::Ok(());
    let m = r.map(MapToResult::test);
    println!("{:?}", m.unwrap());
}