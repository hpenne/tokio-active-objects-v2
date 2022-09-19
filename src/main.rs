mod comp_a;
mod interfaces;

use crate::comp_a::CompA;
use crate::interfaces::CommandsA;
use std::sync::Arc;
use std::time;

#[tokio::main]
async fn main() {
    println!("Creating A");
    let a = Arc::new(CompA::new());
    a.say_hello().await;

    tokio::time::sleep(time::Duration::from_millis(500)).await;
}
