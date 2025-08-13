use std::time::Duration;
use trpl::{self, race, Either};

fn main() {
    trpl::run(async {
        let f = trpl::spawn_task(async {
            trpl::sleep(Duration::from_millis(2000)).await;
            println!("Hello world");
        });

        let result = trpl::get("https://www.linkedin.com");
        println!("Hello Wolrd4");

        // let title1 = trpl::Html::parse(&result)
        //     .select_first("title")
        //     .map(|ele| ele.inner_html())
        //     .unwrap();

        let result1 = trpl::get("https://www.facebook.com");

        let answer = trpl::race(result, result1);

        let string = match answer.await {
            Either::Right(right) => right,
            Either::Left(left) => left,
        };

        let result = string.text().await;

        let title1 = trpl::Html::parse(&result)
            .select_first("title")
            .map(|ele| ele.inner_html())
            .unwrap();

        println!("First come - > {}", title1);

        f.await.unwrap();
    })
}
