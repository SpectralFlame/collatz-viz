pub mod viz;

use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;
use std::ops::Range;
use std::ptr::NonNull;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CollatzKind {
    Full = 0,
    Short = 1,
    Odd = 2,
    Compact = 3,
}

impl From<i32> for CollatzKind {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Full,
            1 => Self::Short,
            2 => Self::Odd,
            3 => Self::Compact,
            _ => Self::Full,
        }
    }
}

type Node = NonNull<CollatzNode>;

#[derive(Debug, PartialEq)]
struct NodeData {
    value: u64,
    // Orbit stats
    depth: usize,
    highest_point: u64,
}
impl From<(usize, u64, u64)> for NodeData {
    fn from(value: (usize, u64, u64)) -> Self {
        Self {
            value: value.1,
            depth: value.0,
            highest_point: value.2,
        }
    }
}

struct CollatzNode {
    data: NodeData,
    // Node pointers
    down: Option<Node>,
    up1: Option<Node>,
    up2: Option<Node>,
}

impl CollatzNode {
    pub fn new(value: u64) -> Self {
        CollatzNode {
            data: NodeData {
                value,
                depth: 0,
                highest_point: value,
            },
            down: None,
            up1: None,
            up2: None,
        }
    }
}

impl std::fmt::Display for CollatzNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.data.depth, self.data.value)
    }
}

pub struct Collatz {
    kind: CollatzKind,
    root: Node,
    nodes: HashMap<u64, Node>,
    ranges: Vec<Range<u64>>,
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
            ranges: vec![Range { start: 1, end: 2 }],
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
            ranges: vec![Range { start: 1, end: 2 }],
            _boo: PhantomData,
        }
    }

    pub fn kind(&self) -> CollatzKind {
        self.kind
    }

    pub fn contains(&self, n: &u64) -> bool {
        // self.ranges.iter().any(|r| r.contains(n))
        if self.ranges[0].contains(n) {
            true
        } else {
            self.nodes.contains_key(n)
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    fn get_node(&self, n: u64) -> Option<Node> {
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
        for n in (self.ranges[0].end..=max).rev() {
            match self.kind {
                CollatzKind::Odd | CollatzKind::Compact if n % 2 == 0 => continue,
                CollatzKind::Compact if n % 3 == 0 => continue,
                _ => (),
            }
            self.generate_down(n);
        }
        self.ranges[0].end = max + 1;
    }

    pub fn generate_down(&mut self, mut n: u64) {
        if self.contains(&n) {
            // Tree already contains `n`.
            return;
        }
        let mut prev_node: Option<Node> = None;
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

            // Merge created nodes to the found node
            let merge_node = self.get_node(n).unwrap();

            if let Some(prev_node) = prev_node {
                (*prev_node.as_ptr()).down = Some(merge_node);
                if (*merge_node.as_ptr()).up1.is_none() {
                    (*merge_node.as_ptr()).up1 = Some(prev_node);
                } else {
                    (*merge_node.as_ptr()).up2 = Some(prev_node);
                }
            }

            // Trace back to set the depth and highest_point of newly created nodes
            let mut depth = (*merge_node.as_ptr()).data.depth;
            let mut highest_point = (*merge_node.as_ptr()).data.highest_point;
            while let Some(node) = prev_node {
                depth += 1;
                highest_point = highest_point.max((*node.as_ptr()).data.value);
                (*node.as_ptr()).data.depth = depth;
                (*node.as_ptr()).data.highest_point = highest_point;
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
                    let (up1, up2) = self.up((*node.as_ptr()).data.value);
                    if up1 <= max && up1 != 1 {
                        let new_node =
                            NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(up1))));
                        (*new_node.as_ptr()).data.depth = (*node.as_ptr()).data.depth + 1;
                        (*new_node.as_ptr()).data.highest_point = up1.max((*node.as_ptr()).data.highest_point);
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
                        (*new_node.as_ptr()).data.depth = (*node.as_ptr()).data.depth + 1;
                        (*new_node.as_ptr()).data.highest_point = up2.max((*node.as_ptr()).data.highest_point);
                        (*new_node.as_ptr()).down = Some(node);
                        (*node.as_ptr()).up2 = Some(new_node);

                        self.nodes.insert(up2, new_node);
                        node_stack.push(new_node);
                    }
                } else if (*node.as_ptr()).up2.is_none() {
                    let existing = (*node.as_ptr()).up1.unwrap();
                    node_stack.push(existing);

                    let (up1, up2) = self.up((*node.as_ptr()).data.value);

                    if (*existing.as_ptr()).data.value != up1 {
                        // up2 is linked to up1, so link up1 to up2
                        if up1 > max {
                            continue;
                        }

                        let new_node =
                            NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(up1))));
                        (*new_node.as_ptr()).data.depth = (*node.as_ptr()).data.depth + 1;
                        (*new_node.as_ptr()).data.highest_point = up1.max((*node.as_ptr()).data.highest_point);
                        (*node.as_ptr()).up2 = Some(new_node);
                        (*new_node.as_ptr()).down = Some(node);

                        self.nodes.insert(up1, new_node);
                        node_stack.push(new_node);

                        // No need to check `up2` anymore
                        continue;
                    }

                    if let Some(up2) = up2 {
                        if up2 > max || up2 == 1 {
                            continue;
                        }

                        let new_node =
                            NonNull::new_unchecked(Box::into_raw(Box::new(CollatzNode::new(up2))));
                        (*new_node.as_ptr()).data.depth = (*node.as_ptr()).data.depth + 1;
                        (*new_node.as_ptr()).data.highest_point = up2.max((*node.as_ptr()).data.highest_point);
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

    // NOTE: Should the orbit of `n` be calculated instead of panicking?
    pub fn get_depth(&self, n: u64) -> usize {
        let node = self
            .get_node(n)
            .expect("the requested node has not yet been generated");
        unsafe { (*node.as_ptr()).data.depth }
    }

    // NOTE: Should the orbit of `a` and `b` be calculated instead of panicking?
    pub fn find_common_ancestor(&self, a: u64, b: u64) -> u64 {
        let node_a = self
            .get_node(a)
            .expect("the requested node has not yet been generated");
        let node_b = self
            .get_node(b)
            .expect("the requested node has not yet been generated");

        unsafe {
            let node = self.find_common_ancestor_unsafe(node_a, node_b);
            (*node.as_ptr()).data.value
        }
    }

    unsafe fn find_common_ancestor_unsafe(&self, mut a: Node, mut b: Node) -> Node {
        while (*a.as_ptr()).data.depth < (*b.as_ptr()).data.depth {
            b = (*b.as_ptr()).down.unwrap();
        }
        while (*a.as_ptr()).data.depth > (*b.as_ptr()).data.depth {
            a = (*a.as_ptr()).down.unwrap();
        }
        while (*a.as_ptr()).data.value != (*b.as_ptr()).data.value {
            a = (*a.as_ptr()).down.unwrap();
            b = (*b.as_ptr()).down.unwrap();
        }
        return a;
    }

    pub fn iter_orbit(&self, n: u64) -> IterOrbit {
        unsafe { IterOrbit::new(self.get_node(n).unwrap().as_ref()) }
    }
    pub fn iter(&self) -> Iter {
        Iter::new(&self)
    }
}

struct IterOrbit<'a> {
    current_node: &'a CollatzNode,
}

impl<'a> IterOrbit<'a> {
    fn new(node: &'a CollatzNode) -> IterOrbit<'a> {
        Self {
            current_node: node,
        }
    }
}

impl<'a> Iterator for IterOrbit<'a> {
    type Item = &'a NodeData;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.current_node.down.is_none() {
                return None;
            }
            let data = &self.current_node.data;
            self.current_node = self.current_node.down.unwrap().as_ref();
            Some(data)
        }
    }
}

struct Iter<'a> {
    current_node: &'a CollatzNode,
    stack: VecDeque<&'a CollatzNode>,
}

impl<'a> Iter<'a> {
    fn new(collatz: &'a Collatz) -> Self {
        unsafe {
            let head_node = collatz.root.as_ref();
            Iter {
                current_node: head_node,
                stack: VecDeque::from([head_node]),
            }
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a NodeData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }
        self.current_node = self.stack.pop_front().unwrap();

        let node = self.current_node;
        unsafe {
            if let Some(node) = node.up1 {
                self.stack.push_back(node.as_ref());
            }
            if let Some(node) = node.up2 {
                self.stack.push_back(node.as_ref());
            }
        }

        Some(&node.data)
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
                (0, 1, 1).into(),
                (1, 2, 2).into(),
                (2, 4, 4).into(),
                (3, 8, 8).into(),
                (4, 16, 16).into(),
                (5, 5, 16).into(),
                (6, 10, 16).into(),
                (7, 3, 16).into(),
                (8, 6, 16).into(),
            ],
        );
        itertools::assert_equal(
            collatz.into_iter(),
            [
                (0, 1, 1).into(),
                (1, 2, 2).into(),
                (2, 4, 4).into(),
                (3, 8, 8).into(),
                (4, 16, 16).into(),
                (5, 5, 16).into(),
                (6, 10, 16).into(),
                (7, 3, 16).into(),
                (8, 6, 16).into(),
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
                (0, 1, 1).into(),
                (1, 2, 2).into(),
                (2, 4, 4).into(),
                (3, 8, 8).into(),
                (4, 16, 16).into(),
                (5, 5, 16).into(),
                (6, 10, 16).into(),
                (7, 3, 16).into(),
                (7, 20, 20).into(),
                (8, 6, 16).into(),
                (8, 40, 40).into(),
                (9, 80, 80).into(),
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
                (0, 1, 1).into(),
                (1, 2, 2).into(),
                (2, 4, 4).into(),
                (3, 8, 8).into(),
                (4, 16, 16).into(),
                (5, 5, 16).into(),
                (6, 10, 16).into(),
                (7, 20, 20).into(),
                (7, 3, 16).into(),
                (8, 40, 40).into(),
                (8, 6, 16).into(),
                (9, 13, 40).into(),
                (10, 26, 40).into(),
                (11, 52, 52).into(),
                (12, 17, 52).into(),
                (13, 34, 52).into(),
                (14, 11, 52).into(),
                (15, 22, 52).into(),
                (16, 7, 52).into(),
                (17, 14, 52).into(),
                (18, 28, 52).into(),
                (19, 9, 52).into(),
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
                (0, 1, 1).into(),
                (1, 2, 2).into(),
                (2, 4, 4).into(),
                (3, 8, 8).into(),
                (4, 16, 16).into(),
                (5, 32, 32).into(),
                (5, 5, 16).into(),
                (6, 10, 16).into(),
                (7, 20, 20).into(),
                (7, 3, 16).into(),
                (8, 6, 16).into(),
                (9, 12, 16).into(),
                (10, 24, 24).into(),
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
                (0, 1, 1).into(),
                (1, 2, 2).into(),
                (2, 4, 4).into(),
                (3, 8, 8).into(),
                (4, 16, 16).into(),
                (5, 5, 16).into(),
                (6, 10, 16).into(),
                (7, 20, 20).into(),
                (7, 3, 16).into(),
                (8, 40, 40).into(),
                (8, 6, 16).into(),
                (9, 80, 80).into(),
                (9, 12, 16).into(),
            ],
        );
    }

    #[test]
    fn common_ancestor() {
        let mut collatz = Collatz::default();
        collatz.generate_down(22);
        collatz.generate_down(69);
        collatz.generate_down(70);
        assert_eq!(collatz.find_common_ancestor(69, 70), 40);
        assert_eq!(collatz.find_common_ancestor(22, 69), 52);
        assert_eq!(collatz.find_common_ancestor(69, 69), 69);
    }

    #[test]
    fn get_depth() {
        let mut collatz = Collatz::default();
        collatz.generate_down(69);
        collatz.generate_down(420);
        assert_eq!(collatz.get_depth(69), 14);
        assert_eq!(collatz.get_depth(420), 40);
    }
}
