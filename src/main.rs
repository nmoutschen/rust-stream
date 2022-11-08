mod adder;

mod counter;
use std::time::Duration;

use adder::Adder;
use counter::Counter;
use futures::StreamExt;

#[tokio::main]
async fn main() {
    let counter = Counter::new(10);

    let mut adder: Adder<_, usize> = Adder::new(counter);

    while let Some(number) = adder.next().await {
        println!("{}", number);
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
