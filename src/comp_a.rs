use crate::interfaces::CommandsA;
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct CompA {
    tx: UnboundedSender<Messages>,
}

impl CompA {
    pub fn new() -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel::<Messages>();
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                let mut i = CompAImpl::new();
                use Messages::*;
                match message {
                    AsyncOp { operation } => {
                        operation(&mut i);
                    }
                    SyncOp { operation } => {
                        operation(&mut i);
                    }
                }
            }
        });
        Self { tx }
    }
}

#[async_trait]
impl CommandsA for CompA {
    async fn say_hello<'a, 'r>(&self) {
        let _ = self.tx.send(Messages::AsyncOp {
            operation: Box::new(async move |i: &mut CompAImpl| {
                i.say_hello().await;
            }),
        });
    }
}

type AsyncOp = Box<dyn FnOnce(&mut CompAImpl) + Send + 'static>;

enum Messages {
    AsyncOp { operation: AsyncOp },
    SyncOp { operation: AsyncOp },
}

struct CompAImpl {}

impl CompAImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CommandsA for CompAImpl {
    async fn say_hello<'a, 'r>(&self) {
        println!("Hello world!");
    }
}
