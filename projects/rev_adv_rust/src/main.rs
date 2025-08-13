trait Pilot {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

trait Wizard {
    fn fly(&self) {
        println!("Up!");
    }
}

struct Human;

panic!()
impl Pilot for Human {}

impl Wizard for Human {}

impl Human {}

fn main() {
    let h = Human {};

    <Human as Wizard>::fly(&h);
    Pilot::fly(&h);
}
