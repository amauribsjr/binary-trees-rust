use std::collections::VecDeque;
use std::io::{self, Write};

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: i32) {
        if self.root.is_none() {
            self.root = Some(Box::new(Node::new(value)));
            return;
        }

        let mut current = self.root.as_mut();

        while let Some(node) = current {
            if value < node.value && node.left.is_none() {
                node.left = Some(Box::new(Node::new(value)));
                break;
            } else if value > node.value && node.right.is_none() {
                node.right = Some(Box::new(Node::new(value)));
                break;
            } else if value == node.value {
                println!("Value already exists, it won't be inserted.");
                break;
            } else if value > node.value {
                current = node.right.as_mut();
            } else {
                current = node.left.as_mut();
            }
        }
    }

    fn search(&self, value: i32) -> Option<&Node> {
        let mut current = self.root.as_deref();

        while let Some(node) = current {
            if value < node.value {
                current = node.left.as_deref();
            } else if value > node.value {
                current = node.right.as_deref();
            } else {
                return Some(node);
            }
        }

        None
    }

    fn calculate_height(&self) -> i32 {
        Self::calculate_height_node(self.root.as_deref())
    }

    fn calculate_height_node(current: Option<&Node>) -> i32 {
        match current {
            None => 0,
            Some(node) => {
                let left_height = Self::calculate_height_node(node.left.as_deref());
                let right_height = Self::calculate_height_node(node.right.as_deref());
                1 + left_height.max(right_height)
            }
        }
    }

    fn remove(&mut self, value: i32) {
        self.root = Self::remove_node(self.root.take(), value);
    }

    fn remove_node(current: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        let mut node = current?;

        if value < node.value {
            node.left = Self::remove_node(node.left.take(), value);
            Some(node)
        } else if value > node.value {
            node.right = Self::remove_node(node.right.take(), value);
            Some(node)
        } else {
            if node.left.is_none() && node.right.is_none() {
                return None;
            }

            if node.left.is_none() {
                return node.right;
            }

            if node.right.is_none() {
                return node.left;
            }

            let successor_value = Self::smallest_value(node.right.as_deref().unwrap());
            node.value = successor_value;
            node.right = Self::remove_node(node.right.take(), successor_value);

            Some(node)
        }
    }

    fn smallest_value(mut node: &Node) -> i32 {
        while let Some(left) = node.left.as_deref() {
            node = left;
        }

        node.value
    }

    fn print_tree(&self) {
        Self::print_tree_node(self.root.as_deref());
    }

    fn print_tree_node(current: Option<&Node>) {
        if let Some(node) = current {
            Self::print_tree_node(node.left.as_deref());
            print!("{} ", node.value);
            Self::print_tree_node(node.right.as_deref());
        }
    }

    fn print_by_level(&self) {
        if self.root.is_none() {
            return;
        }

        let height = self.calculate_height();
        let mut queue: VecDeque<Option<&Node>> = VecDeque::new();

        queue.push_back(self.root.as_deref());

        let max_width = 2_i32.pow(height as u32) - 1;

        for level in 0..height {
            let level_size = queue.len();
            let spaces = max_width / 2_i32.pow((level + 1) as u32);

            Self::print_spaces(spaces);

            for _ in 0..level_size {
                let current = queue.pop_front().unwrap();

                if let Some(node) = current {
                    print!("{}", node.value);
                    queue.push_back(node.left.as_deref());
                    queue.push_back(node.right.as_deref());
                } else {
                    print!("n");
                    queue.push_back(None);
                    queue.push_back(None);
                }

                Self::print_spaces(spaces * 2 + 1);
            }

            println!();
        }
    }

    fn print_spaces(count: i32) {
        for _ in 0..count {
            print!(" ");
        }
    }
}

fn read_i32() -> Result<i32, String> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    input.trim().parse::<i32>().map_err(|e| e.to_string())
}

fn main() {
    let mut bst = BinarySearchTree::new();
    let mut choice = -1;

    println!("BSTree");

    while choice != 0 {
        println!(
            "\n[1]- Add node \
             \n[2]- Search node by value \
             \n[3]- Calculate height \
             \n[4]- Remove node \
             \n[5]- Show complete tree (increases size significantly from height > 5) \
             \n[6]- Show tree in ascending order \
             \n[0]- Exit"
        );

        print!("> ");
        io::stdout().flush().unwrap();

        match read_i32() {
            Ok(value) => choice = value,
            Err(e) => {
                println!("error found {}", e);
                continue;
            }
        }

        match choice {
            0 => {}

            1 => {
                print!("Insert node value: ");
                io::stdout().flush().unwrap();

                match read_i32() {
                    Ok(value) => bst.insert(value),
                    Err(e) => println!("error found {}", e),
                }
            }

            2 => {
                print!("Insert the value you want to search: ");
                io::stdout().flush().unwrap();

                match read_i32() {
                    Ok(value) => match bst.search(value) {
                        Some(found_node) => println!("Value {} found.", found_node.value),
                        None => println!("Value {} not found.", value),
                    },
                    Err(e) => println!("error found {}", e),
                }
            }

            3 => {
                println!("Tree height: {}", bst.calculate_height());
            }

            4 => {
                print!("Insert the value of the node you want to remove: ");
                io::stdout().flush().unwrap();

                match read_i32() {
                    Ok(value) => bst.remove(value),
                    Err(e) => println!("error found {}", e),
                }
            }

            5 => {
                bst.print_by_level();
                println!();
            }

            6 => {
                println!();
                bst.print_tree();
                println!();
            }

            _ => println!("Insert a valid value."),
        }
    }
}
