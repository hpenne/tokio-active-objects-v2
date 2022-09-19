use async_trait::async_trait;

#[async_trait]
pub trait CommandsA {
    async fn say_hello<'a, 'r>(&self);
}

#[async_trait]
pub trait EventsA {
    fn hello_from_a(&self);
}

#[async_trait]
pub trait EventsB {
    fn hello_from_b(&self);
}
