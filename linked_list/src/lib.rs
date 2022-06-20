
use std::borrow::{Borrow, BorrowMut};


#[allow(dead_code)]
struct LinkedNode<T> {
    value: T,
    next: Option<Box<LinkedNode<T>>>,
}

#[allow(dead_code)]
impl<T> LinkedNode<T> {
    fn new(value:T) -> Self {
        Self { value, next: None}
    }

    fn has_next(&self) -> bool {
        self.next.is_some()
    }

    fn add_next(&mut self, value: T) {
        self.next = Some(Box::new(Self::new(value)));
    }
}
#[allow(dead_code)]
struct LinkedList<T> {
    head: Option<Box<LinkedNode<T>>>,
}

#[allow(dead_code)]
impl<T> LinkedList<T> {
    fn new(values: Option<Vec<T>>) -> Self {
        match values {
            None => Self { head: None },
            Some(v) => {
                let mut cursor: Option<Box<LinkedNode<T>>> = None;
                for value in v {
                    cursor = match cursor.borrow() {
                        Some(_)=> Some(Box::new(LinkedNode {value, next: cursor})),
                        None => Some(Box::new(LinkedNode::new(value))),
                    }
                }

                Self { head: cursor }
            }
        }
    }

    fn count(&self) -> usize {
        let mut counter: usize = 0;
        let mut cursor: &Option<Box<LinkedNode<T>>>= &self.head;

        loop {
            if cursor.is_none() {
                break;
            } else {
                counter+=1;
                cursor = &cursor.as_ref().unwrap().next;
            }
        }

        counter
    }
}

impl<T> LinkedList<T> where T: Clone {
    fn as_vec(&self) -> Vec<T> {
        let mut output : Vec<T> = Vec::new();

        let mut cursor: &Option<Box<LinkedNode<T>>>= &self.head;

        loop {
            match cursor.borrow_mut() {
                None => break,
                Some(c) => {
                    output.push(c.value.clone());
                    cursor = &c.next
                },
            }
        }

        output.reverse();
        output
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_count() {
        let value = vec![3,4,5];
        let new_list = LinkedList::new(Some(value));
        assert_eq!(new_list.count(), 3);
    }

    #[test]
    fn test_linked_list_as_vec() {
        let value = vec!["test".to_string(),"23".to_string(),"39".to_string()];
        let new_list = LinkedList::new(Some(value.clone()));
        assert_eq!(new_list.as_vec(), value);
    }

}
