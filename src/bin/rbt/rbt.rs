use std::collections::VecDeque;

const RED: bool = true;
const BLACK: bool = false;

type Link = Option<usize>;

#[derive(Clone, Debug)]
struct Node {
    key: i32,
    is_red: bool,
    left: Link,
    right: Link,
    parent: Link,
}

impl Node {
    fn new(key: i32) -> Self {
        Self {
            key,
            is_red: RED,
            left: None,
            right: None,
            parent: None,
        }
    }
}

pub struct RBT {
    nodes: Vec<Node>,
    root: Link,
}

impl RBT {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
        }
    }

    fn color(&self, node: Link) -> bool {
        match node {
            Some(index) => self.nodes[index].is_red,
            None => BLACK,
        }
    }

    fn set_color(&mut self, node: Link, color: bool) {
        if let Some(index) = node {
            self.nodes[index].is_red = color;
        }
    }

    fn parent(&self, node: usize) -> Link {
        self.nodes[node].parent
    }

    fn grandparent(&self, node: usize) -> Link {
        self.parent(node).and_then(|parent| self.nodes[parent].parent)
    }

    pub fn insert(&mut self, key: i32) -> bool {
        if self.find_node(key).is_some() {
            return false;
        }
    
        let z = self.nodes.len();
        self.nodes.push(Node::new(key));
    
        let mut y: Link = None;
        let mut x = self.root;
    
        while let Some(x_index) = x {
            y = x;
            if self.nodes[z].key < self.nodes[x_index].key {
                x = self.nodes[x_index].left;
            } else {
                x = self.nodes[x_index].right;
            }
        }
    
        self.nodes[z].parent = y;
    
        if y.is_none() {
            self.root = Some(z);
        } else if self.nodes[z].key < self.nodes[y.unwrap()].key {
            self.nodes[y.unwrap()].left = Some(z);
        } else {
            self.nodes[y.unwrap()].right = Some(z);
        }
    
        self.insert_fixup(z);
        true
    }

    fn insert_fixup(&mut self, mut z: usize) {
        while self.color(self.parent(z)) == RED {
            let parent = self.parent(z).unwrap();
            let grandparent = self.grandparent(z).unwrap();

            if Some(parent) == self.nodes[grandparent].left {
                let uncle = self.nodes[grandparent].right;

                if self.color(uncle) == RED {
                    self.set_color(Some(parent), BLACK);
                    self.set_color(uncle, BLACK);
                    self.set_color(Some(grandparent), RED);

                    z = grandparent;
                } else {
                    if Some(z) == self.nodes[parent].right {
                        z = parent;
                        self.rotate_left(z);
                    }

                    let parent = self.parent(z).unwrap();
                    let grandparent = self.grandparent(z).unwrap();

                    self.set_color(Some(parent), BLACK);
                    self.set_color(Some(grandparent), RED);

                    self.rotate_right(grandparent);
                }
            } else {
                let uncle = self.nodes[grandparent].left;

                if self.color(uncle) == RED {
                    self.set_color(Some(parent), BLACK);
                    self.set_color(uncle, BLACK);
                    self.set_color(Some(grandparent), RED);

                    z = grandparent;
                } else {
                    if Some(z) == self.nodes[parent].left {
                        z = parent;
                        self.rotate_right(z);
                    }

                    let parent = self.parent(z).unwrap();
                    let grandparent = self.grandparent(z).unwrap();

                    self.set_color(Some(parent), BLACK);
                    self.set_color(Some(grandparent), RED);

                    self.rotate_left(grandparent);
                }
            }
        }

        self.set_color(self.root, BLACK);
    }

    fn rotate_left(&mut self, x: usize) {
        let y = self.nodes[x]
            .right
            .expect("rotate_left requires a right child");

        let y_left = self.nodes[y].left;

        self.nodes[x].right = y_left;

        if let Some(y_left_index) = y_left {
            self.nodes[y_left_index].parent = Some(x);
        }

        let x_parent = self.nodes[x].parent;

        self.nodes[y].parent = x_parent;

        if x_parent.is_none() {
            self.root = Some(y);
        } else if self.nodes[x_parent.unwrap()].left == Some(x) {
            self.nodes[x_parent.unwrap()].left = Some(y);
        } else {
            self.nodes[x_parent.unwrap()].right = Some(y);
        }

        self.nodes[y].left = Some(x);
        self.nodes[x].parent = Some(y);
    }

    fn rotate_right(&mut self, x: usize) {
        let y = self.nodes[x]
            .left
            .expect("rotate_right requires a left child");

        let y_right = self.nodes[y].right;

        self.nodes[x].left = y_right;

        if let Some(y_right_index) = y_right {
            self.nodes[y_right_index].parent = Some(x);
        }

        let x_parent = self.nodes[x].parent;

        self.nodes[y].parent = x_parent;

        if x_parent.is_none() {
            self.root = Some(y);
        } else if self.nodes[x_parent.unwrap()].right == Some(x) {
            self.nodes[x_parent.unwrap()].right = Some(y);
        } else {
            self.nodes[x_parent.unwrap()].left = Some(y);
        }

        self.nodes[y].right = Some(x);
        self.nodes[x].parent = Some(y);
    }

    pub fn search(&self, key: i32) -> bool {
        self.find_node(key).is_some()
    }

    fn find_node(&self, key: i32) -> Link {
        let mut current = self.root;

        while let Some(index) = current {
            if key == self.nodes[index].key {
                return Some(index);
            } else if key < self.nodes[index].key {
                current = self.nodes[index].left;
            } else {
                current = self.nodes[index].right;
            }
        }

        None
    }

    pub fn calculate_height(&self) -> i32 {
        self.calculate_height_node(self.root)
    }

    fn calculate_height_node(&self, node: Link) -> i32 {
        match node {
            None => 0,
            Some(index) => {
                let left_height = self.calculate_height_node(self.nodes[index].left);
                let right_height = self.calculate_height_node(self.nodes[index].right);

                1 + left_height.max(right_height)
            }
        }
    }

    pub fn remove(&mut self, key: i32) -> bool {
        let z = self.find_node(key);
    
        if z.is_none() {
            return false;
        }
    
        self.delete(z.unwrap());
        true
    }

    fn transplant(&mut self, u: usize, v: Link) {
        let u_parent = self.nodes[u].parent;

        if u_parent.is_none() {
            self.root = v;
        } else if Some(u) == self.nodes[u_parent.unwrap()].left {
            self.nodes[u_parent.unwrap()].left = v;
        } else {
            self.nodes[u_parent.unwrap()].right = v;
        }

        if let Some(v_index) = v {
            self.nodes[v_index].parent = u_parent;
        }
    }

    fn delete(&mut self, z: usize) {
        let mut y = z;
        let x: Link;
        let x_parent: Link;
        let mut y_original_color = self.nodes[y].is_red;

        if self.nodes[z].left.is_none() {
            x = self.nodes[z].right;
            x_parent = self.nodes[z].parent;

            self.transplant(z, self.nodes[z].right);
        } else if self.nodes[z].right.is_none() {
            x = self.nodes[z].left;
            x_parent = self.nodes[z].parent;

            self.transplant(z, self.nodes[z].left);
        } else {
            y = self.smallest_node(self.nodes[z].right.unwrap());
            y_original_color = self.nodes[y].is_red;
            x = self.nodes[y].right;

            if self.nodes[y].parent == Some(z) {
                x_parent = Some(y);

                if let Some(x_index) = x {
                    self.nodes[x_index].parent = Some(y);
                }
            } else {
                x_parent = self.nodes[y].parent;

                self.transplant(y, self.nodes[y].right);

                self.nodes[y].right = self.nodes[z].right;

                if let Some(right) = self.nodes[y].right {
                    self.nodes[right].parent = Some(y);
                }
            }

            self.transplant(z, Some(y));

            self.nodes[y].left = self.nodes[z].left;

            if let Some(left) = self.nodes[y].left {
                self.nodes[left].parent = Some(y);
            }

            self.nodes[y].is_red = self.nodes[z].is_red;
        }

        if y_original_color == BLACK {
            self.delete_fixup(x, x_parent);
        }
    }

    fn delete_fixup(&mut self, mut x: Link, mut x_parent: Link) {
        while x != self.root && self.color(x) == BLACK {
            let Some(parent) = x_parent else {
                break;
            };

            if x == self.nodes[parent].left {
                let mut w = self.nodes[parent].right;

                if self.color(w) == RED {
                    self.set_color(w, BLACK);
                    self.set_color(Some(parent), RED);
                    self.rotate_left(parent);

                    w = self.nodes[parent].right;
                }

                let w_left_black = self.color(w.and_then(|i| self.nodes[i].left)) == BLACK;
                let w_right_black = self.color(w.and_then(|i| self.nodes[i].right)) == BLACK;

                if w_left_black && w_right_black {
                    self.set_color(w, RED);

                    x = Some(parent);
                    x_parent = self.nodes[parent].parent;
                } else {
                    let w_right_black = self.color(w.and_then(|i| self.nodes[i].right)) == BLACK;

                    if w_right_black {
                        if let Some(w_index) = w {
                            self.set_color(self.nodes[w_index].left, BLACK);
                            self.set_color(w, RED);
                            self.rotate_right(w_index);
                        }

                        w = self.nodes[parent].right;
                    }

                    if let Some(w_index) = w {
                        self.nodes[w_index].is_red = self.nodes[parent].is_red;
                        self.nodes[parent].is_red = BLACK;
                        self.set_color(self.nodes[w_index].right, BLACK);
                    }

                    self.rotate_left(parent);

                    x = self.root;
                    x_parent = None;
                }
            } else {
                let mut w = self.nodes[parent].left;

                if self.color(w) == RED {
                    self.set_color(w, BLACK);
                    self.set_color(Some(parent), RED);
                    self.rotate_right(parent);

                    w = self.nodes[parent].left;
                }

                let w_right_black = self.color(w.and_then(|i| self.nodes[i].right)) == BLACK;
                let w_left_black = self.color(w.and_then(|i| self.nodes[i].left)) == BLACK;

                if w_right_black && w_left_black {
                    self.set_color(w, RED);

                    x = Some(parent);
                    x_parent = self.nodes[parent].parent;
                } else {
                    let w_left_black = self.color(w.and_then(|i| self.nodes[i].left)) == BLACK;

                    if w_left_black {
                        if let Some(w_index) = w {
                            self.set_color(self.nodes[w_index].right, BLACK);
                            self.set_color(w, RED);
                            self.rotate_left(w_index);
                        }

                        w = self.nodes[parent].left;
                    }

                    if let Some(w_index) = w {
                        self.nodes[w_index].is_red = self.nodes[parent].is_red;
                        self.nodes[parent].is_red = BLACK;
                        self.set_color(self.nodes[w_index].left, BLACK);
                    }

                    self.rotate_right(parent);

                    x = self.root;
                    x_parent = None;
                }
            }
        }

        self.set_color(x, BLACK);
    }

    fn smallest_node(&self, mut node: usize) -> usize {
        while let Some(left) = self.nodes[node].left {
            node = left;
        }

        node
    }

    pub fn print_tree(&self) {
        self.print_tree_node(self.root);
    }

    fn print_tree_node(&self, current: Link) {
        if let Some(index) = current {
            self.print_tree_node(self.nodes[index].left);

            print!(
                "{}{} ",
                self.nodes[index].key,
                if self.nodes[index].is_red {
                    "[R]"
                } else {
                    "[B]"
                }
            );

            self.print_tree_node(self.nodes[index].right);
        }
    }

    pub fn print_by_level(&self) {
        if self.root.is_none() {
            println!("Tree is empty.");
            return;
        }

        let height = self.calculate_height();
        let mut queue: VecDeque<Link> = VecDeque::new();

        queue.push_back(self.root);

        for level in 0..height {
            let level_size = queue.len();
            let spaces = 2_i32.pow((height - level - 1) as u32) - 1;

            Self::print_spaces(spaces * 4);

            for _ in 0..level_size {
                let current = queue.pop_front().unwrap();

                if let Some(index) = current {
                    let color = if self.nodes[index].is_red { "R" } else { "B" };

                    print!("{:>3}{}", self.nodes[index].key, color);

                    queue.push_back(self.nodes[index].left);
                    queue.push_back(self.nodes[index].right);
                } else {
                    print!(" --- ");

                    queue.push_back(None);
                    queue.push_back(None);
                }

                Self::print_spaces((spaces * 2 + 1) * 4);
            }

            println!("\n");
        }
    }

    fn print_spaces(count: i32) {
        for _ in 0..count {
            print!(" ");
        }
    }
}