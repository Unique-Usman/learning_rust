use std::{pin::pin, time::Duration};

use trpl::{self, ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let stream = trpl::stream_from_iter(v).map(|n| n * 2);

        let mut filtered = stream.filter(|n| n % 3 == 0 || n % 5 == 0);

        while let Some(val) = filtered.next().await {
            println!("{val}");
        }

        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(message) = messages.next().await {
            match message {
                Ok(result) => println!("{result}"),
                Err(reason) => eprintln!("Problem: {reason}"),
            }
        }
    })
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });
    ReceiverStream::new(rx)
}
