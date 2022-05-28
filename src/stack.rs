/*
Ref : https://applied-math-coding.medium.com/implementing-a-stack-in-rust-e5cfd3bb3ab9
*/
struct Stack<T> {
    top: Option<Node<T>>,
}
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    fn new(top: Node<T>) -> Stack<T> {
        Stack { top: Some(top) }
    }
    fn is_empty(&self) -> bool {
        match self.top {
            Some(_) => false,
            None => true,
        }
    }
    fn push(&mut self, data: T) {
        /*
        push creates a new Node using the given data and lets Stack.top own the new Node.
        Moreover, this node’s next shall become the new owner of the former top value
        */
        let mut node = Node::new(data);
        if let Some(top) = std::mem::replace(&mut self.top, None) {
            node.next = Some(Box::new(top));
        }
        self.top = Some(node);
    }

    fn peek(&self) -> Option<&T> {
        match &self.top {
            Some(top) => Some(&top.value),
            None => None,
        }
    }
    /*
    fn pop(&self) -> Option<&T> {
        match &self.top {}
    }
    */
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value: value,
            next: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = Node::new(1);
        let stack = Stack::new(node);
        print!("{:?}", stack.is_empty());
    }
    #[test]
    fn test_push() {
        let mut stack = Stack::new(Node::new(1));
        stack.push(2);
        stack.push(3);
        print!("{:?}", stack.is_empty());
    }
}
