pub struct SimpleLinkedList<T> {
    list: Vec<T>,
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn push(&mut self, element: T) {
        self.list.push(element)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.last()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut new_list = self.list;
        new_list.reverse();

        let mut link = SimpleLinkedList::new();
        new_list.iter().for_each(|item: &T| link.push(item.clone()));
        link
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut link = Self::new();
        iter.into_iter().for_each(|item| {
            link.push(item);
        });
        link
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Vec<T> {
        linked_list.list
    }
}
