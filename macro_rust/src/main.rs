macro_rules! hello {
    () => {
        println!("Hello Usman");
    };
}

macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

macro_rules! greet {
    () => {
        println!("Hello, World");
    };
    ($name:expr) => {
        println!("Hello, {}", $name);
    };
}

macro_rules! triple {
    ($x:expr) => {
        $x * 3
    };
}

macro_rules! make_printer {
    ($name:ident, $val:expr) => {
        fn $name() {
            println!("{}", $val)
        }
    };
}

fn prin(a: i32) {
    println!("{a}")
}

macro_rules! make_struct {
    ($name:ident, $t:ty) => {
        struct Type {
            $name: $t,
        }
    };
}

macro_rules! area {
    (square, $side:expr) => {
        $side * $side
    };

    (rectangle, $w:expr, $h:expr) => {
        $w * $h
    };
}

macro_rules! sum {
    ($($x:expr),*) => {{
       let mut value = 0;
       $(
           value += $x;
       )*
        value
    }};
}

macro_rules! print_all {
    ($($x:expr), +) => {{
        $(
            println!("{}", $x);
        )*
    }};
}

macro_rules! min {
    ($x:expr) => {
       $x
    };
    ($x:expr, $($rest:expr), +) => {
        std::cmp::min($x, min!($($rest),+))
    }
}

macro_rules! concat_string {
    ($x:expr) => {
       format!("{}",$x)
    };
    ($x:expr, $($rest:expr), +) => {
        format!("{} {}", $x, concat_string!($($rest),+))
    }
}

macro_rules! call_fn {
    ($path:ident, $arg:expr) => {
        $path($arg)
    };
}

macro_rules! run_block {
    ($b:block) => {
        $b
    };
}

macro_rules! show_toke {
    ($t:tt) => {
        println!("Token Token - {}", format!("{}", $t))
    };
}

macro_rules! make_getters {
    ($($ele:ident:$t:ty),*) => {
        $(
            fn $ele (&self) -> &$t {
                &self.$ele
            }
        )*
    };
}

macro_rules! make_struct_with_default {
    ($name:ident, $host:ident: $t:ty = $to:expr, $d:ident: $x:ty = $y:expr, $d1:ident: $x2:ty = $y3:expr ) => {
        struct $name {
            $host: $t,
            $d: $x,
            $d1: $x2,
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    $host: $to,
                    $d: $y,
                    $d1: $y3,
                }
            }
        }
    };
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    make_getters!(name: String, age: u32);
}

make_struct_with_default!(
    Config,
    host: String = "localhost".to_string(),
    port: u16 = 8080,
    debug: bool = true
);

macro_rules! dispatch {
    (
        $cmd:expr,
        $( ($con:pat => $b:block) ),+ $(,)?
    ) => {
        match $cmd {
            $(
                $con => $b,
            )+
            _ => { println!("Unknown command"); }
        }
    };
}

fn main() {
    println!("The square of x is {}", square!(4));
    greet!();
    greet!("Usman");
    hello!();

    let a = triple!(4);
    make_printer!(print, 3);
    println!("{a}");
    print();

    println!("area of square with side 3 is {}", area!(square, 3));
    println!(
        "area of rectangle with side w of 3 and height of 4 is {}",
        area!(rectangle, 3, 4)
    );

    println!("The some of this value is {}", sum!(3, 4, 5, 5));
    print_all!(3, 4, 5, 5);
    println!("The min of this value is {}", min!(3, 4, 5, 5));
    println!("{}", concat_string!("Hello", ",", "World"));
    call_fn!(prin, 6);

    run_block!({
        println!("hey block - {}", 4 + 6);
    });

    show_toke!({ 4 + 6 });

    let command: &str = "start";

    dispatch!(
        command,
        ("start" => { println!("Starting service..."); }),
        ("stop" => { println!("Stopping service..."); }),
        ("restart" => { println!("Restarting service..."); })
    );
}
