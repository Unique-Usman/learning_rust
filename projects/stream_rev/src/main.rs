use std::{future::pending, pin::pin, time::Duration};

use trpl::{self, ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        // let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // let mut v = trpl::stream_from_iter(v).filter(|x| x % 3 == 0 || x % 5 == 0);

        let messages = get_message().timeout(Duration::from_millis(200));
        let intervals = get_interval()
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_millis(10));

        let mut messages = pin!(messages.merge(intervals).take(20));

        while let Some(val) = messages.next().await {
            match val {
                Ok(val) => println!("val = {val}"),
                Err(err) => eprintln!("{err:?}"),
            }
        }
    })
}

fn get_message() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for val in v {
            if val % 2 == 0 {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(300)).await;
            } else {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_interval() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}
