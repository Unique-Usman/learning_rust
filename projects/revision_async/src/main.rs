use std::{
    os::unix::thread,
    pin::{self, pin, Pin},
    process::Output,
    time::Duration,
};

use trpl::{self, spawn_task, Either};

fn main() {
    trpl::run(async {
        // let a = trpl::get("https://www.google.com");
        // let b = trpl::get("https://www.twitter.com");
        //
        // let value = match trpl::race(b, a).await {
        //     trpl::Either::Left(left) => left,
        //     trpl::Either::Right(right) => right,
        // };
        //
        // let title = trpl::Html::parse(&value.text().await)
        //     .select_first("title")
        //     .unwrap()
        //     .inner_html();
        // println!("title - {title}");

        let (tx, mut rx) = trpl::channel();

        let tx2 = tx.clone();
        let a = pin!(async move {
            let v = vec![
                String::from("Ade"),
                String::from("is"),
                String::from("a"),
                String::from("Legend"),
            ];

            for val in v {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let b = pin!(async move {
            let v = vec![
                String::from("Ade"),
                String::from("is"),
                String::from("a"),
                String::from("Legend"),
            ];

            for val in v {
                tx2.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let c = pin!(async {
            while let Some(val) = rx.recv().await {
                println!("value received -> {val} ");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let v: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![a, b, c];
        trpl::join_all(v).await;

        let slow = async {
            // println!("Slow boy");
            trpl::sleep(Duration::from_millis(100)).await;
            "Slow Boy"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(val) => println!("val = {val}"),
            Err(err) => println!("Error => {err:?}"),
        }
    })
}

async fn timeout<F: Future>(f: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(f, trpl::sleep(max_time)).await {
        Either::Left(left) => Ok(left),
        Either::Right(_) => Err(max_time),
    }
}
