// Node struct
struct Node{
    data: i32,
    next: Option<Box<Node>>,
}

// Linked List struct
struct LinkedList{
    head: Option<Box<Node>>,
}

impl LinkedList{
    // Create a new empty linked list
    fn new() -> LinkedList{
        LinkedList{head: None}
    }

    // Add node at the end
    fn append(&mut self, data: i32){                            // &mut self -> This method takes a mutable reference to the current instance
        let new_node = Box::new(Node { data, next: None });     // new_node -> Box<Node>
        match self.head.as_mut(){                               // self.head.as_mut() -> Option<&mut Box<Node>>
            None => {
                self.head = Some(new_node);
            }
            Some(mut current) => {                              // current -> &mut Box<Node>
                while let Some(ref mut next) = current.next{    // current.next -> Option<Box<Node>>
                    current = next;
                }
                current.next = Some(new_node);
            }
        }
    }

    // Add node at the beginning
    fn prepend(&mut self, data: i32){
        let new_node = Box::new(Node { data,
                                       next: self.head.take()});
        self.head = Some(new_node);
    }

    // Delete a node
    fn delete_node(&mut self, key: i32) {
        // handle situation of deleting self.head node
        if let Some(ref node) = self.head {                      // node -> &Box<Node>
            if node.data == key {                                // self.head node to be deleted
                let old_head = self.head.take();
                if let Some(n) = old_head {
                    self.head = n.next;                          // delete old_head node
                    return;
                }
            }
        }
        let mut current = &mut self.head;                        // current -> &mut Option<Box<Node>>
        while let Some(node) = current {                         // node -> &mut Box<Node>
            if let Some(ref next) = node.next {
                if next.data == key {                            // node.next node to be deleted
                    let del_node = node.next.take();
                    if let Some(n) = del_node {
                        node.next = n.next;
                        return;
                    }
                }
            }
            current = &mut node.next;                            // node.next -> Option<Box<Node>>
        }
    }

    // Print the linked list
    fn print_list(&self){
        let mut current = self.head.as_ref();
        while let Some(node) = current{
            print!("{} -> ", node.data);
            current = node.next.as_ref();
        }
        println!("None");
    }
}

fn main() {
    let mut linked_list = LinkedList::new();
    linked_list.print_list();

    linked_list.append(10);
    linked_list.append(20);
    linked_list.append(30);
    linked_list.print_list();

    linked_list.prepend(5);
    linked_list.print_list();

    linked_list.delete_node(20);
    linked_list.print_list();
}
