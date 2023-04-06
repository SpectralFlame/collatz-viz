use std::collections::{HashMap, VecDeque};
use std::ptr::NonNull;
use std::marker::PhantomData;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CollatzKind {
    Full,
    Short,
    Odd,
    Compact,
}

type Link = Option<NonNull<CollatzNode>>;

#[derive(Debug)]
struct CollatzNode {
    value: u64,
    down: Link,
    up1: Link,
    up2: Link,
}

impl CollatzNode {
    pub fn new(value: u64) -> Self {
        CollatzNode {
            value,
            down: None,
            up1: None,
            up2: None,
        }
    }
}

impl std::fmt::Display for CollatzNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct Collatz {
    kind: CollatzKind,
    tree: NonNull<CollatzNode>,
    nodes: HashMap<u64, NonNull<CollatzNode>>,
    _boo: PhantomData<CollatzNode>,
}

impl Default for Collatz {
    fn default() -> Self {
        let head = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(1)))) };
        let nodes = HashMap::from([(1, head)]);
        Self {
            kind: CollatzKind::Full,
            tree: head,
            nodes,
            _boo: PhantomData,
        }
    }
}

impl Collatz {
    pub fn new(kind: CollatzKind) -> Self {
        let head = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(1)))) };
        let nodes = HashMap::from([(1, head)]);
        Self {
            kind,
            tree: head,
            nodes,
            _boo: PhantomData,
        }
    }

    pub fn contains(&self, n: &u64) -> bool {
        self.nodes.contains_key(n)
    }

    fn get_node(&self, n: u64) -> Link {
        self.nodes.get(&n).copied()
    }

    pub fn down(&self, n: u64) -> u64 {
        match &self.kind {
            CollatzKind::Full => {
                if n % 2 == 0 {
                    return n / 2;
                } else {
                    return n * 3 + 1;
                }
            },
            CollatzKind::Short => {
                if n % 2 == 0 {
                    return n / 2;
                } else {
                    return (n * 3 + 1) / 2;
                }
            },
            CollatzKind::Odd => todo!(),
            CollatzKind::Compact => todo!(),
        }
    }

    pub fn up(&self, n: u64) -> (u64, Option<u64>) {
        match &self.kind {
            CollatzKind::Full => {
                if n % 3 == 1 && n > 8 {
                    // `n / 3` is equivalent to `(n - 1) / 3` due to integer arithmetic
                    return (n * 2, Some(n / 3)); 
                } else {
                    return (n * 2, None);
                }
            },
            CollatzKind::Short => {
                if n % 3 == 2 {
                    return (n * 2, Some(2 * n / 3));
                } else {
                    return (n * 2, None);
                }
            },
            CollatzKind::Odd => todo!(),
            CollatzKind::Compact => todo!(),
        }
    }

    pub fn generate_down(&mut self, mut n: u64) {
        if self.contains(&n) {
            // Tree already contains `n`.
            return;
        }
        let mut prev_node: Option<NonNull<CollatzNode>> = None;
        while !self.contains(&n) {
            unsafe {
                // Create a new node
                let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(n))));

                // Doubly link this and the previous node, if Some(_)
                if let Some(prev_node) = prev_node {
                    (*prev_node.as_ptr()).down = Some(new_node);
                    (*new_node.as_ptr()).up1 = Some(prev_node);
                }

                self.nodes.insert(n, new_node);

                n = self.down(n);

                prev_node = Some(new_node);
            }
        }

        // Merge created nodes to the found root node
        if let Some(root_node) = self.get_node(n) {
            unsafe {
                if let Some(prev_node) = prev_node {
                    (*prev_node.as_ptr()).down = Some(root_node);
                    if (*root_node.as_ptr()).up1.is_none() {
                        (*root_node.as_ptr()).up1 = Some(prev_node);
                    } else {
                        (*root_node.as_ptr()).up2 = Some(prev_node);
                    }
                }
            }
        } else {
            unreachable!();
        }
    }

    pub fn generate_up(&mut self, max: u64) {
        let mut node_stack = vec![self.tree];
        let mut count = 0;
        while !node_stack.is_empty() && count < 10 {
            count += 1;
            unsafe {
                let node = node_stack.pop().unwrap();

                if (*node.as_ptr()).up1.is_none() {
                    let (up1, up2) = self.up((*node.as_ptr()).value);
                    if up1 <= max {
                        let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(up1))));
                        
                        (*new_node.as_ptr()).down = Some(node);
                        (*node.as_ptr()).up1 = Some(new_node);

                        self.nodes.insert(up1, new_node);
                        node_stack.push(new_node);
                    }

                    if let Some(up2) = up2 {
                        if up2 > max {
                            continue;
                        }
                        let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(up2))));

                        (*new_node.as_ptr()).down = Some(node);
                        (*node.as_ptr()).up2 = Some(new_node);

                        self.nodes.insert(up2, new_node);
                        node_stack.push(new_node);
                    }
                } else if (*node.as_ptr()).up2.is_none() {
                    if let (_, Some(up2)) = self.up((*node.as_ptr()).value) {
                        if up2 > max {
                            continue;
                        }
                        let new_node = NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(up2))));

                        (*new_node.as_ptr()).down = Some(node);
                        (*node.as_ptr()).up2 = Some(new_node);

                        self.nodes.insert(up2, new_node);
                        node_stack.push(new_node);
                    }
                }
            }
        }
    }

    pub fn into_iter(self) -> IntoIter {
        IntoIter::new(self)
    }
}


struct IntoIter {
    current_node: NonNull<CollatzNode>,
    stack: VecDeque<NonNull<CollatzNode>>,
}

impl IntoIter {
    fn new(collatz: Collatz) -> Self {
        let head_node = collatz.tree;
        IntoIter {
            current_node: head_node,
            stack: VecDeque::from([head_node]),
        }
    }
}

impl Iterator for IntoIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }
        self.current_node = self.stack.pop_front().unwrap();

        let node = unsafe { &*self.current_node.as_ptr() };
        if let Some(node) = node.up1 {
            self.stack.push_back(node);
        }
        if let Some(node) = node.up2 {
            self.stack.push_back(node);
        }
        
        Some(node.value)
    }
}

#[cfg(test)]
mod test {
    use itertools;
    use super::*;

    #[test]
    fn single_down() {
        let collatz = Collatz::default();
        assert_eq!(collatz.down(1), 4);
        assert_eq!(collatz.down(2), 1);
        assert_eq!(collatz.down(3), 10);
        assert_eq!(collatz.down(4), 2);
    }

    #[test]
    fn single_up() {
        let collatz = Collatz::default();
        assert_eq!(collatz.up(1), (2, None));
        assert_eq!(collatz.up(2), (4, None));
        assert_eq!(collatz.up(3), (6, None));
        assert_eq!(collatz.up(4), (8, None));
        assert_eq!(collatz.up(5), (10, None));
        assert_eq!(collatz.up(6), (12, None));
        assert_eq!(collatz.up(10), (20, Some(3)));
        assert_eq!(collatz.up(16), (32, Some(5)));
    }

    #[test]
    fn generate_single_down() {
        let mut collatz = Collatz::default();
        collatz.generate_down(6);
        itertools::assert_equal(collatz.into_iter(), [1, 2, 4, 8, 16, 5, 10, 3, 6]);
    }

    #[test]
    fn generate_multiple_down() {
        let mut collatz = Collatz::default();
        collatz.generate_down(6);
        collatz.generate_down(80);
        itertools::assert_equal(collatz.into_iter(), [1, 2, 4, 8, 16, 5, 10, 3, 20, 6, 40, 80]);
    }

    #[test]
    fn small_up() {
        let mut collatz = Collatz::default();
        collatz.generate_up(16);
        itertools::assert_equal(collatz.into_iter(), [1, 2, 4, 8, 16, 5, 10, 3, 6, 12]);
    }
}
