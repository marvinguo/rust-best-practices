// async fn in trait is not stable
// try use async_trait crate instead to test
// check https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/

use tokio::runtime::Runtime;
use async_trait::async_trait;

#[async_trait]
trait Engine {
    type Output;
    async fn run(&self) -> Result<Self::Output, ()>;
}

struct Simple;

#[async_trait]
impl Engine for Simple {
    type Output = i32;

    async fn run(&self) -> Result<Self::Output, ()> {
        println!("run");
        Ok(123)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = Runtime::new()?;
    let simple = Simple;
    let future = simple.run();
    let result = runtime.block_on(future);
    println!("result:{:?}", result.unwrap());
    Ok(())
}