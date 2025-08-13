use std::{pin::Pin, process::Output};

use trpl;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx2 = tx.clone();

        let a = async move {
            let v = vec![
                String::from("Hello"),
                String::from("This"),
                String::from("is"),
                String::from("Usman"),
            ];

            for val in v {
                tx.send(val).unwrap();
            }
        };

        let b = async move {
            let v = vec![
                String::from("Hello"),
                String::from("This"),
                String::from("is"),
                String::from("Usman"),
            ];

            for val in v {
                tx2.send(val).unwrap();
            }
        };

        let c = async {
            while let Some(r) = rx.recv().await {
                println!("rx {r}");
            }
        };

        // trpl::join3(a, b, c).await;

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(a), Box::pin(b), Box::pin(c)];
        trpl::join_all(futures).await;
    })
}
