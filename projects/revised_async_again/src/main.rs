use std::{
    pin::{self, pin},
    time::Duration,
};

use trpl::{self, ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        // let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        //
        // let mut stream = trpl::stream_from_iter(v).filter(|x| x % 3 == 0 || x % 5 == 0);

        let stream = get_messages().timeout(Duration::from_millis(200));

        let mut stream = pin!(stream
            .merge(
                get_intervals()
                    .map(|x| String::from(format!("value - {x}")))
                    .timeout(Duration::from_millis(10))
                    .throttle(Duration::from_millis(100))
            )
            .take(20));

        while let Some(val) = stream.next().await {
            // println!("val = {val}");
            trpl::sleep(Duration::from_millis(100)).await;

            match val {
                Ok(val) => println!("val = {val}"),
                Err(err) => println!("err = {err}"),
            }
        }
    })
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let v = vec![
            String::from("Adewole"),
            String::from("is"),
            String::from("a"),
            String::from("Boy"),
        ];

        for (i, val) in v.into_iter().enumerate() {
            if i % 2 == 0 {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            } else {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;

        loop {
            tx.send(count).unwrap();
            trpl::sleep(Duration::from_millis(100)).await;
            count += 1;
        }
    });

    ReceiverStream::new(rx)
}
