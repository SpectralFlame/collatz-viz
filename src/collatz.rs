use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;
use std::ptr::NonNull;

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
    depth: usize,
    down: Link,
    up1: Link,
    up2: Link,
}

impl CollatzNode {
    pub fn new(value: u64) -> Self {
        CollatzNode {
            value,
            depth: 0,
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
    root: NonNull<CollatzNode>,
    nodes: HashMap<u64, NonNull<CollatzNode>>,
    _boo: PhantomData<CollatzNode>,
}

impl Default for Collatz {
    fn default() -> Self {
        let head = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(1)))) };
        let nodes = HashMap::from([(1, head)]);
        Self {
            kind: CollatzKind::Full,
            root: head,
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
            root: head,
            nodes,
            _boo: PhantomData,
        }
    }

    pub fn kind(&self) -> CollatzKind {
        self.kind
    }

    pub fn contains(&self, n: &u64) -> bool {
        self.nodes.contains_key(n)
    }

    fn get_node(&self, n: u64) -> Link {
        self.nodes.get(&n).copied()
    }

    pub fn down(&self, n: u64) -> u64 {
        match &self.kind {
            CollatzKind::Full => match n % 2 {
                0 => n / 2,
                1 => n * 3 + 1,
                _ => unreachable!(),
            },
            CollatzKind::Short => match n % 2 {
                0 => n / 2,
                1 => (n * 3 + 1) / 2,
                _ => unreachable!(),
            },
            CollatzKind::Odd => {
                match n % 8 {
                    5 => n / 4, // integer arithmetic for (n - 1) / 4
                    3 | 7 => (3 * n + 1) / 2,
                    1 => (3 * n + 1) / 4,
                    _ => unreachable!(),
                }
            }
            CollatzKind::Compact => {
                // match n % 12 {
                //     7 | 11 => (3 * n + 1) / 2,
                //     n => match n % 24 {
                //         5 => n / 4,                    // (n - 1) / 4
                //         1 | 17 => (3 * n + 1) / 4,
                //         n => match n % 96 {
                //             85 => n / 16,              // (n - 5) / 16
                //             13 | 61 => (3 * n + 1) / 8,
                //             37 => (3 * n + 1) / 16,
                //             _ => unreachable!(),
                //         }
                //     }
                // }

                match n % 96 {
                    5 | 29 | 53 | 77 => n / 4, // (n - 1) / 4
                    85 => n / 16,              // (n - 5) / 16
                    7 | 11 | 19 | 23 | 31 | 35 | 43 | 47 | 55 | 59 | 67 | 71 | 79 | 83 | 91
                    | 95 => (3 * n + 1) / 2,
                    1 | 17 | 25 | 41 | 49 | 65 | 73 | 89 => (3 * n + 1) / 4,
                    13 | 61 => (3 * n + 1) / 8,
                    37 => (3 * n + 1) / 16,
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn up(&self, n: u64) -> (u64, Option<u64>) {
        match &self.kind {
            CollatzKind::Full => {
                // n = 3 * (2m + 1) + 1 = 6m + 4        =>  n % 6 == 4
                match n % 6 {
                    4 => (n * 2, Some(n / 3)), // (n - 1) / 3
                    _ => (n * 2, None),
                }
            }
            CollatzKind::Short => {
                // n = (3(2m + 1) + 1) / 2 = 3m + 2     =>  n % 3 == 2
                match n % 3 {
                    2 => (n * 2, Some(n * 2 / 3)),
                    _ => (n * 2, None),
                }
            }
            CollatzKind::Odd => {
                // n = ((8m+5) - 1) / 4 = 2m + 1        =>  n % 2 == 1
                // n = (3(8m + 3) + 1) / 2 = 12m + 5    =>  n % 12 == 5
                // n = (3(8m + 7) + 1) / 2 = 12m + 11   =>  n % 12 == 11
                // n = (3(8m + 1) + 1) / 4 = 6m + 1     =>  n % 6 == 1
                match n % 12 {
                    1 | 7 => (n * 4 + 1, Some(n * 4 / 3)),  // (n * 4 - 1) / 3
                    5 | 11 => (n * 4 + 1, Some(n * 2 / 3)), // (n * 2 - 1) / 3
                    3 | 9 => (n * 4 + 1, None),
                    _ => unreachable!(),
                }
            }
            CollatzKind::Compact => {
                // n = ((24m + 5) - 1) / 4 = 6m + 1     =>  n % 6 == 1
                // n = ((96m + 85) - 5) / 16 = 6m + 5   =>  n % 6 == 5
                // n = (3(12m + 7) + 1) / 2 = 18m + 11  =>  n % 18 == 11
                // n = (3(12m + 11) + 1) / 2 = 18m + 17 =>  n % 18 == 17
                // n = (3(24m + 1) + 1) / 4 = 18m + 1   =>  n % 18 == 1
                // n = (3(24m + 17) + 1) / 4 = 18m + 13 =>  n % 18 == 13
                // n = (3(48m + 13) + 1) / 8 = 18m + 5  =>  n % 18 == 5
                // n = (3(96m + 37) + 1) / 16 = 18m + 7 =>  n % 18 == 7
                match n % 18 {
                    11 | 17 => (n * 16 + 5, Some(n * 2 / 3)), // (n * 2 - 1) / 3
                    1 | 13 => (n * 4 + 1, Some(n * 4 / 3)),   // (n * 4 - 1) / 3
                    5 => (n * 16 + 5, Some(n * 8 / 3)),       // (n * 8 - 1) / 3
                    7 => (n * 4 + 1, Some(n * 16 / 3)),       // (n * 16 - 1) / 3
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn generate_fill_down(&mut self, max: u64) {
        for n in (1..=max).rev() {
            self.generate_down(n);
        }
    }

    pub fn generate_down(&mut self, mut n: u64) {
        if self.contains(&n) {
            // Tree already contains `n`.
            return;
        }
        let mut prev_node: Option<NonNull<CollatzNode>> = None;
        unsafe {
            while !self.contains(&n) {
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

            // Merge created nodes to the found root node
            let root_node = self.get_node(n).unwrap();

            if let Some(prev_node) = prev_node {
                (*prev_node.as_ptr()).down = Some(root_node);
                if (*root_node.as_ptr()).up1.is_none() {
                    (*root_node.as_ptr()).up1 = Some(prev_node);
                } else {
                    (*root_node.as_ptr()).up2 = Some(prev_node);
                }
            }

            // Trace back to set the depth of newly created nodes
            let mut depth = (*root_node.as_ptr()).depth;
            while let Some(node) = prev_node {
                depth += 1;
                (*node.as_ptr()).depth = depth;
                // All nodes are new, so they are linked to `up1`
                prev_node = (*node.as_ptr()).up1;
            }
        }
    }

    pub fn generate_up(&mut self, max: u64) {
        let mut node_stack = vec![self.root];
        let mut count = 0;
        while !node_stack.is_empty() && count < 10 {
            count += 1;
            unsafe {
                let node = node_stack.pop().unwrap();

                if (*node.as_ptr()).up1.is_none() {
                    let (up1, up2) = self.up((*node.as_ptr()).value);
                    if up1 <= max && up1 != 1 {
                        let new_node =
                            NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(up1))));
                        (*new_node.as_ptr()).depth = (*node.as_ptr()).depth + 1;
                        (*new_node.as_ptr()).down = Some(node);
                        (*node.as_ptr()).up1 = Some(new_node);

                        self.nodes.insert(up1, new_node);
                        node_stack.push(new_node);
                    }

                    if let Some(up2) = up2 {
                        if up2 > max || up2 == 1 {
                            continue;
                        }

                        let new_node =
                            NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(up2))));
                        (*new_node.as_ptr()).depth = (*node.as_ptr()).depth + 1;
                        (*new_node.as_ptr()).down = Some(node);
                        (*node.as_ptr()).up2 = Some(new_node);

                        self.nodes.insert(up2, new_node);
                        node_stack.push(new_node);
                    }
                } else if (*node.as_ptr()).up2.is_none() {
                    let existing = (*node.as_ptr()).up1.unwrap();
                    node_stack.push(existing);

                    let (up1, up2) = self.up((*node.as_ptr()).value);

                    if (*existing.as_ptr()).value != up1 {
                        // up2 is linked to up1, so link up1 to up2
                        if up1 > max {
                            continue;
                        }

                        let new_node =
                            NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(up1))));
                        (*new_node.as_ptr()).depth = (*node.as_ptr()).depth + 1;
                        (*node.as_ptr()).up2 = Some(new_node);
                        (*new_node.as_ptr()).down = Some(node);

                        self.nodes.insert(up1, new_node);
                        node_stack.push(new_node);

                        // No need to check up2 anymore
                        continue;
                    }

                    if let Some(up2) = up2 {
                        if up2 > max || up2 == 1 {
                            continue;
                        }

                        let new_node =
                            NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(up2))));
                        (*new_node.as_ptr()).depth = (*node.as_ptr()).depth + 1;
                        (*new_node.as_ptr()).down = Some(node);
                        (*node.as_ptr()).up2 = Some(new_node);

                        self.nodes.insert(up2, new_node);
                        node_stack.push(new_node);
                    }
                } else {
                    node_stack.push((*node.as_ptr()).up1.unwrap());
                    node_stack.push((*node.as_ptr()).up2.unwrap());
                }
            }
        }
    }

    pub fn iter(&self) -> Iter {
        Iter::new(&self)
    }
    pub fn into_iter(self) -> IntoIter {
        IntoIter::new(self)
    }
}

struct Iter {
    current_node: NonNull<CollatzNode>,
    stack: VecDeque<NonNull<CollatzNode>>,
}

impl Iter {
    fn new(collatz: &Collatz) -> Self {
        let head_node = collatz.root;
        Iter {
            current_node: head_node,
            stack: VecDeque::from([head_node]),
        }
    }
}

impl Iterator for Iter {
    type Item = (usize, u64);

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

        Some((node.depth, node.value))
    }
}

struct IntoIter {
    current_node: NonNull<CollatzNode>,
    stack: VecDeque<NonNull<CollatzNode>>,
}

impl IntoIter {
    fn new(collatz: Collatz) -> Self {
        let head_node = collatz.root;
        IntoIter {
            current_node: head_node,
            stack: VecDeque::from([head_node]),
        }
    }
}

impl Iterator for IntoIter {
    type Item = (usize, u64);

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

        Some((node.depth, node.value))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools;

    #[test]
    fn single_down_full() {
        let collatz = Collatz::default();
        assert_eq!(collatz.down(1), 4);
        assert_eq!(collatz.down(2), 1);
        assert_eq!(collatz.down(3), 10);
        assert_eq!(collatz.down(4), 2);
    }

    #[test]
    fn single_down_short() {
        let collatz = Collatz::new(CollatzKind::Short);
        assert_eq!(collatz.down(1), 2);
        assert_eq!(collatz.down(2), 1);
        assert_eq!(collatz.down(3), 5);
        assert_eq!(collatz.down(4), 2);
    }

    #[test]
    fn single_down_odd() {
        let collatz = Collatz::new(CollatzKind::Odd);
        assert_eq!(collatz.down(1), 1);
        assert_eq!(collatz.down(3), 5);
        assert_eq!(collatz.down(5), 1);
        assert_eq!(collatz.down(7), 11);
    }

    #[test]
    fn single_down_compact() {
        let collatz = Collatz::new(CollatzKind::Compact);
        assert_eq!(collatz.down(1), 1);
        assert_eq!(collatz.down(5), 1);
        assert_eq!(collatz.down(7), 11);
        assert_eq!(collatz.down(11), 17);
    }

    #[test]
    fn single_up_full() {
        let collatz = Collatz::default();
        assert_eq!(collatz.up(1), (2, None));
        assert_eq!(collatz.up(2), (4, None));
        assert_eq!(collatz.up(3), (6, None));
        assert_eq!(collatz.up(4), (8, Some(1)));
        assert_eq!(collatz.up(5), (10, None));
        assert_eq!(collatz.up(6), (12, None));
        assert_eq!(collatz.up(10), (20, Some(3)));
        assert_eq!(collatz.up(16), (32, Some(5)));
        assert_eq!(collatz.up(22), (44, Some(7)));
    }

    #[test]
    fn single_up_short() {
        let collatz = Collatz::new(CollatzKind::Short);
        assert_eq!(collatz.up(1), (2, None));
        assert_eq!(collatz.up(2), (4, Some(1)));
        assert_eq!(collatz.up(3), (6, None));
        assert_eq!(collatz.up(4), (8, None));
        assert_eq!(collatz.up(5), (10, Some(3)));
        assert_eq!(collatz.up(11), (22, Some(7)));
    }

    #[test]
    fn single_up_odd() {
        let collatz = Collatz::new(CollatzKind::Odd);
        assert_eq!(collatz.up(1), (5, Some(1)));
        assert_eq!(collatz.up(3), (13, None));
        assert_eq!(collatz.up(5), (21, Some(3)));
        assert_eq!(collatz.up(7), (29, Some(9)));
        assert_eq!(collatz.up(9), (37, None));
    }

    #[test]
    fn single_up_compact() {
        let collatz = Collatz::new(CollatzKind::Compact);
        assert_eq!(collatz.up(1), (5, Some(1)));
        assert_eq!(collatz.up(5), (85, Some(13)));
        assert_eq!(collatz.up(7), (29, Some(37)));
        assert_eq!(collatz.up(11), (181, Some(7)));
        assert_eq!(collatz.up(13), (53, Some(17)));
        assert_eq!(collatz.up(17), (277, Some(11)));
    }

    #[test]
    fn generate_single_down() {
        let mut collatz = Collatz::default();
        collatz.generate_down(6);
        itertools::assert_equal(
            collatz.iter(),
            [
                (0, 1),
                (1, 2),
                (2, 4),
                (3, 8),
                (4, 16),
                (5, 5),
                (6, 10),
                (7, 3),
                (8, 6),
            ],
        );
        itertools::assert_equal(
            collatz.into_iter(),
            [
                (0, 1),
                (1, 2),
                (2, 4),
                (3, 8),
                (4, 16),
                (5, 5),
                (6, 10),
                (7, 3),
                (8, 6),
            ],
        );
    }

    #[test]
    fn generate_multiple_down() {
        let mut collatz = Collatz::default();
        collatz.generate_down(6);
        collatz.generate_down(80);
        itertools::assert_equal(
            collatz.into_iter(),
            [
                (0, 1),
                (1, 2),
                (2, 4),
                (3, 8),
                (4, 16),
                (5, 5),
                (6, 10),
                (7, 3),
                (7, 20),
                (8, 6),
                (8, 40),
                (9, 80),
            ],
        );
    }

    #[test]
    fn generate_fill_down() {
        let mut collatz = Collatz::default();
        collatz.generate_fill_down(10);
        itertools::assert_equal(
            collatz.into_iter(),
            [
                (0, 1),
                (1, 2),
                (2, 4),
                (3, 8),
                (4, 16),
                (5, 5),
                (6, 10),
                (7, 20),
                (7, 3),
                (8, 40),
                (8, 6),
                (9, 13),
                (10, 26),
                (11, 52),
                (12, 17),
                (13, 34),
                (14, 11),
                (15, 22),
                (16, 7),
                (17, 14),
                (18, 28),
                (19, 9),
            ],
        );
    }

    #[test]
    fn generate_single_up() {
        let mut collatz = Collatz::default();
        collatz.generate_up(32);
        itertools::assert_equal(
            collatz.into_iter(),
            [
                (0, 1),
                (1, 2),
                (2, 4),
                (3, 8),
                (4, 16),
                (5, 32),
                (5, 5),
                (6, 10),
                (7, 20),
                (7, 3),
                (8, 6),
                (9, 12),
                (10, 24),
            ],
        );
    }

    #[test]
    fn generate_mixed() {
        let mut collatz = Collatz::default();
        collatz.generate_down(80);
        collatz.generate_up(16);
        itertools::assert_equal(
            collatz.into_iter(),
            [
                (0, 1),
                (1, 2),
                (2, 4),
                (3, 8),
                (4, 16),
                (5, 5),
                (6, 10),
                (7, 20),
                (7, 3),
                (8, 40),
                (8, 6),
                (9, 80),
                (9, 12),
            ],
        );
    }
}
