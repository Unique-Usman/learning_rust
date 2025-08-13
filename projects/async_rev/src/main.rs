use std::{
    pin::{self, pin, Pin},
    thread,
    time::Duration,
};

use trpl::{self, Either};

fn main() {
    trpl::run(async {
        // let t1 = trpl::get("https://www.google.com");
        //
        // let t2 = trpl::get("https://www.google.com");
        //
        // let result = match trpl::race(t1, t2).await {
        //     Either::Right(right) => right,
        //     Either::Left(left) => left,
        // };
        //
        // let result = result.text().await;
        //
        // let title = trpl::Html::parse(&result)
        //     .select_first("title")
        //     .unwrap()
        //     .inner_html();
        // println!("{title}");

        let (tx1, mut rx) = trpl::channel();

        let tx2 = tx1.clone();

        let t1 = pin!(async move {
            let v = vec![
                String::from("Hello World1"),
                String::from("This"),
                String::from("is"),
                String::from("Usman"),
            ];

            for val in v {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let t2 = pin!(async move {
            let v = vec![
                String::from("Hello World2"),
                String::from("This"),
                String::from("is"),
                String::from("Usman"),
            ];

            for val in v {
                tx2.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let r = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("value - {}", value);
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let vec: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![(t1), (t2), (r)];

        trpl::join_all(vec).await;

        let slow = async {
            println!("Hello from slow");
            trpl::sleep(Duration::from_millis(1000)).await;
            "Hello World"
        };

        match timeout(slow, Duration::from_millis(100)).await {
            Ok(message) => println!("MESSAGE {message}"),
            Err(err) => println!("err, {:?}", err),
        }
    });
}

async fn timeout<F: Future>(f: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(f, trpl::sleep(max_time)).await {
        Either::Left(left) => Ok(left),
        Either::Right(_) => Err(max_time),
    }
}
