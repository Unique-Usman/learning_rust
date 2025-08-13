use std::{
    pin::{pin, Pin},
    process::Output,
    time::{self, Duration},
};
use trpl::{self, Either, Html};

fn main() {
    trpl::run(async {
        let (tx, mut rec) = trpl::channel();

        let tx2 = tx.clone();
        let a = pin!(async move {
            let v = vec![String::from("Usman Akinyemi"), String::from("Adewole")];

            for var in v {
                tx.send(var).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let b = pin!(async move {
            let v = vec![String::from("Usman Akinyemi"), String::from("Adewole")];

            for var in v {
                tx2.send(var).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let c = pin!(async {
            while let Some(r) = rec.recv().await {
                println!("value - {r}");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![(a), (b), (c)];

        trpl::join_all(futures).await;

        let slow = async {
            trpl::sleep(Duration::from_millis(100)).await;
            "Finish"
        };

        match timeout(slow, Duration::from_millis(10)).await {
            Ok(m) => println!("{m}"),
            Err(t) => println!("{t:?}"),
        }
    });
}

async fn timeout<F: Future>(f: F, t: Duration) -> Result<F::Output, Duration> {
    match trpl::race(f, trpl::sleep(t)).await {
        Either::Left(m) => Ok(m),
        Either::Right(_) => Err(t),
    }
}
