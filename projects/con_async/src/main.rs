use std::pin::pin;
use std::{pin::Pin, time::Duration};
// use trpl::{Either, Html};

fn main() {
    // trpl::run(async {
    //     let handle = trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });
    //
    //     for i in 1..5 {
    //         println!("hi number {i} from the second task!");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }
    //
    //     handle.await.unwrap();
    // });

    // trpl::run(async {
    //     let fut1 = async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };
    //
    //     fut1.await;
    //
    //     let fut2 = async {
    //         for i in 1..5 {
    //             println!("hi number {i} from the second task");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };
    //
    //     fut2.await
    //
    //     // trpl::join(fut1, fut2).await;
    // })

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        // let val = String::from("hi");
        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("Got: {value}");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        // trpl::join3(tx_fut, rx_fut, tx1_fut).await;
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];
        trpl::join_all(futures).await;

        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
    })
}
