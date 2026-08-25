use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    head: Link<T>
}

struct Node<T> {
    data: T,
    next: Link<T>
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Self { data, next }
    }
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut curr = &self.head;
        let mut size = 0;
        while let Some(x) = curr {
            size += 1;
            curr = &x.next;
        }
        size
    }

    pub fn push(&mut self, element: T) {
        let node = Box::new(Node::new(element, self.head.take()));
        self.head = Some(node)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_some() {
            let head_node = self.head.take().unwrap();
            self.head = head_node.next;
            Some(head_node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut new_list = SimpleLinkedList::new();
        let mut cur = self.head;
        
        while let Some(node) = cur {
            new_list.push(node.data);
            cur = node.next;
        }
        new_list
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let linked_list = linked_list.rev();
        let mut list = Vec::new();
        let mut cur = linked_list.head;
        while let Some(node) = cur {
            list.push(node.data);
            cur = node.next;
        }

        list
    }
}
