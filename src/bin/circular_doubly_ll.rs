use std::marker::PhantomData;
use std::ptr::{null_mut, NonNull};

struct Node<T> {
    element: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

pub struct CircularDoublyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> CircularDoublyLinkedList<T> {
    pub fn new() -> Self {
        CircularDoublyLinkedList {
            head: None,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, element: T) {
        unsafe {
            let new_node = Box::into_raw(Box::new(Node {
                element,
                next: null_mut(),
                prev: null_mut(),
            }));

            match self.head {
                Some(head) => {
                    let tail = (*head.as_ptr()).prev;

                    (*new_node).next = head.as_ptr();
                    (*new_node).prev = tail;

                    (*head.as_ptr()).prev = new_node;
                    if !tail.is_null() {
                        (*tail).next = new_node;
                    } else {
                        (*head.as_ptr()).next = new_node;
                    }
                }
                None => {
                    (*new_node).next = new_node;
                    (*new_node).prev = new_node;
                }
            }

            self.head = Some(NonNull::new_unchecked(new_node));
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, element: T) {
        self.push_front(element);
        unsafe {
            self.head = Some(NonNull::new_unchecked((*self.head.unwrap().as_ptr()).next));
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.head.map(|head| {
                let head_ptr = head.as_ptr();
                let tail = (*head_ptr).prev;

                if head_ptr == (*head_ptr).next {
                    self.head = None;
                } else {
                    let next = (*head_ptr).next;
                    (*next).prev = tail;
                    (*tail).next = next;
                    self.head = Some(NonNull::new_unchecked(next));
                }
                self.len -= 1;

                let boxed_node = Box::from_raw(head_ptr);
                boxed_node.element
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.head.map(|head| {
                let tail_ptr = (*head.as_ptr()).prev;
                if tail_ptr == head.as_ptr() {
                    self.head = None;
                } else {
                    let prev = (*tail_ptr).prev;
                    (*prev).next = head.as_ptr();
                    (*head.as_ptr()).prev = prev;
                }
                self.len -= 1;

                let boxed_node = Box::from_raw(tail_ptr);
                boxed_node.element
            })
        }
    }
}

impl<T> Drop for CircularDoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

fn main() {
    let mut list = CircularDoublyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    println!("List length: {}", list.len());

    while let Some(val) = list.pop_front() {
        println!("Popped: {}", val);
    }
}
