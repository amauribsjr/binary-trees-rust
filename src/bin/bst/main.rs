mod bst;

use bst::BinarySearchTree;
use bst::read_i32;

use std::io;
use std::io::Write;

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