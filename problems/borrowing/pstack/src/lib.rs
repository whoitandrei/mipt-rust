#![forbid(unsafe_code)]
use std::rc::Rc;

pub struct PRef<T> {
    inner: Rc<T>,
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

////////////////////////////////////////////////////////////////////////////////

struct Node<T> {
    value: Rc<T>,
    next: Option<Rc<Node<T>>>,
}

pub struct PStack<T> {
    head: Option<Rc<Node<T>>>,
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        PStack { head: None }
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        PStack {
            head: self.head.clone(),
        }
    }
}

struct PStackIter<T> {
    cur: Option<Rc<Node<T>>>,
}

impl<T> Iterator for PStackIter<T> {
    type Item = PRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.cur.take()?;
        self.cur = node.next.clone();
        Some(PRef {
            inner: node.value.clone(),
        })
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        PStack::default()
    }

    pub fn push(&self, value: T) -> Self {
        let new_node = Node {
            value: Rc::new(value),
            next: self.head.clone(),
        };
        PStack {
            head: Some(Rc::new(new_node)),
        }
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        match &self.head {
            None => None,
            Some(node) => {
                let val = PRef {
                    inner: node.value.clone(),
                };

                let new_stack = PStack { head: node.next.clone() };

                Some((val, new_stack))
            }
        }
    }

    pub fn len(&self) -> usize {
        let mut size: usize = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            size += 1;
            current = &node.next;
        }

        size
    }

    pub fn is_empty(&self) -> bool {
        match &self.head {
            None => true,
            _ => false,
        }
        // self.head().is_none()
    }

    pub fn iter(&self) -> impl Iterator<Item = PRef<T>> {
        PStackIter {
            cur: self.head.clone(),
        }
    }
}
