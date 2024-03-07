use std::{
    fmt::Debug,
    mem::swap,
    ops::{DerefMut, Index, IndexMut},
};

struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub const fn new(element: T) -> Self {
        Self {
            element,
            next: None,
        }
    }

    pub const fn element(&self) -> &T {
        &self.element
    }

    pub fn element_mut(&mut self) -> &mut T {
        &mut self.element
    }

    pub fn len(&self, len: usize) -> usize {
        if let Some(next) = self.next.as_deref() {
            next.len(len) + 1
        } else {
            len
        }
    }

    pub fn get(&self, index: usize) -> Result<&Node<T>, (&Node<T>, usize)> {
        if index == 0 {
            Ok(self)
        } else if let Some(next) = self.next.as_ref() {
            next.get(index - 1)
        } else {
            Err((self, index))
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Result<&mut Node<T>, (*mut Node<T>, usize)> {
        if index == 0 {
            Ok(self)
        } else {
            let t = self as *mut Node<T>;
            self.next
                .as_deref_mut()
                .map_or(Err((t, index)), |next| next.get_mut(index - 1))
        }
    }

    // replace next with None even when he is referencing himself.
    pub fn drop_next(&mut self) {
        let mut temp = None;
        swap(&mut self.next, &mut temp);
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("element", &self.element)
            .field("next", &self.next)
            .finish()
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
            next: self.next.clone(),
        }
    }
}

pub struct List<T> {
    first: Option<Box<Node<T>>>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self { first: None }
    }
}

impl<T> List<T> {
    pub const fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    pub fn len(&self) -> usize {
        if let Some(first) = self.first.as_deref() {
            first.len(0) + 1
        } else {
            0
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if let Some(first) = self.first.as_ref() {
            first.get(index).map(Node::element).ok()
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if let Some(first) = self.first.as_mut() {
            first.get_mut(index).map(Node::element_mut).ok()
        } else {
            None
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn insert(&mut self, index: usize, element: T) -> Option<()> {
        if index == 0 {
            let mut a = Box::new(Node::new(element));
            let b = &mut a.deref_mut().next;
            swap(&mut self.first, b);
            self.first = Some(a);
            Some(())
        } else if let Some(first) = self.first.as_mut() {
            match first.get_mut(index) {
                Ok(node) => {
                    let mut a = Box::new(Node::new(element));
                    let b = &mut a.deref_mut().next;
                    swap(&mut node.next, b);
                    node.next = Some(a);
                    Some(())
                }
                Err((node, 1)) => {
                    let node = unsafe { node.as_mut().expect("") };
                    let mut a = Box::new(Node::new(element));
                    let b = &mut a.deref_mut().next;
                    swap(&mut node.next, b);
                    node.next = Some(a);
                    Some(())
                }
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn push(&mut self, element: T) {
        let mut e = &mut self.first;
        while let Some(node) = e {
            e = &mut node.next;
        }
        *e = Some(Box::new(Node::new(element)));
    }

    pub fn replace(&mut self, index: usize, element: T) -> Option<()> {
        if let Some(first) = self.first.as_mut() {
            if let Ok(node) = first.get_mut(index) {
                node.element = element;
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn remove(&mut self, index: usize) -> Option<()> {
        let a = std::ptr::addr_of_mut!(self.first);
        if let Some(first) = self.first.as_deref_mut() {
            if index == 0 {
                swap(unsafe { a.as_mut().expect("") }, &mut first.next);
                first.drop_next();
                Some(())
            } else if let Ok(node) = first.get_mut(index - 1) {
                let a = std::ptr::addr_of_mut!(node.next);
                if let Some(node2) = node.next.as_deref_mut() {
                    swap(unsafe { a.as_mut().expect("") }, &mut node2.next);
                    node2.drop_next();
                    Some(())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = f.debug_struct("List");
        let mut i: usize = 0;
        let mut current = self.first.as_ref();
        while let Some(c) = current {
            r.field(i.to_string().as_str(), &c.element);
            i += 1;
            current = c.next.as_ref();
        }
        r.finish()
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        Self {
            first: self.first.clone(),
        }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = List::default();
        let mut current = &mut list.first;
        for element in iter {
            *current = Some(Box::new(Node::new(element)));
            current = &mut current.as_mut().expect("").next;
        }
        list
    }
}

impl<T: PartialEq<V>, V> PartialEq<List<V>> for List<T> {
    fn eq(&self, other: &List<V>) -> bool {
        let mut current1 = &self.first;
        let mut current2 = &other.first;
        loop {
            if current1.is_none() && current2.is_none() {
                return true;
            }
            if current1.is_none() || current2.is_none() {
                return false;
            }
            let c1 = current1.as_ref().expect("").as_ref();
            let c2 = current2.as_ref().expect("").as_ref();
            if c1.element != c2.element {
                return false;
            }
            current1 = &c1.next;
            current2 = &c2.next;
        }
    }
}

impl<T: Clone> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.first
            .as_ref()
            .expect("List Index")
            .get(index)
            .map(Node::element)
            .ok()
            .expect("List Index")
    }
}

impl<T: Clone> IndexMut<usize> for List<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.first
            .as_mut()
            .expect("List Index")
            .get_mut(index)
            .map(Node::element_mut)
            .expect("List Index")
    }
}
