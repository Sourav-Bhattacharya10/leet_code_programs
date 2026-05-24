struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }

    fn append(&mut self, value: i32) {
        let last_node = self.get_last_node();
        last_node.next = Some(Box::new(Node::new(value)));
    }

    fn get_last_node(&mut self) -> &mut Self {
        if let Some(ref mut next_node) = self.next {
            next_node.get_last_node()
        } else {
            self
        }
    }

    fn print(&self) {
        print!("{} -> ", self.value);
        if let Some(ref next) = self.next {
            next.print();
        }
    }
}

fn main() {
    println!("Linked List:");
    let mut linked_list = Node::new(1);
    linked_list.append(2);
    linked_list.append(3);
    linked_list.print();
}
