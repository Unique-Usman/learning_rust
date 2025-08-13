pub trait Draw {
    fn draw(&self);
}
// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }

// pub struct Screen<T>
// where
//     T: Draw,
// {
//     pub components: Vec<T>,
// }

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// impl<T: Draw> Screen<T> {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
