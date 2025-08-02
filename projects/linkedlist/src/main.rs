#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn printList(&self) {
        println!("{:?}", self);
    }

    fn printListValue(&self) {
        let mut current = self.head.as_ref();

        while let Some(value) = current {
            println!("{}", value.value);
            current = value.next.as_ref();
        }
    }

    fn push_front(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn push_back(&mut self, value: i32) {
        let new_node = Box::new(Node { value, next: None });

        // Get a mutable reference to the pointer-to-head
        let mut current = &mut self.head;

        // Traverse the list to find the tail (None)
        while let Some(node) = current {
            current = &mut node.next;
        }

        // Insert the new node at the end
        *current = Some(new_node);
    }
}

fn main() {
    let mut list = LinkedList {
        head: Some(Box::new(Node {
            value: 32,
            next: None,
        })),
    };
    list.push_front(42);
    list.push_back(45);
    println!("Hello, world! {:?}", list);
}
