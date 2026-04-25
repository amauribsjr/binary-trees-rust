# Binary Trees in Rust

BST · AVL · Red-Black Tree

---

## Overview

This project presents three classical binary tree data structures implemented in Rust:

* Binary Search Tree (BST)
* AVL Tree
* Red-Black Tree (RBT)

The objective is to provide a clear and faithful implementation of each structure, preserving the original algorithms while adapting them to Rust’s ownership and safety model.

The code prioritizes:

* clarity over abstraction
* explicit algorithmic steps
* implemented entirely in safe Rust (no unsafe blocks)
* close correspondence with traditional textbook implementations

---

## Project Structure
```
src/
└── bin/
    ├── bst/
    │   ├── main.rs
    │   └── bst.rs
    ├── avl/
    │   ├── main.rs
    │   └── avl.rs
    └── rbt/
        ├── main.rs
        └── rbt.rs
```
Each tree is implemented as a separate binary target.

* main.rs handles the command-line interface
* the corresponding .rs file contains the full data structure logic

---

## Implemented Structures

### 1. Binary Search Tree (BST)

A standard binary search tree without any balancing mechanism.

Properties:

* Left subtree contains values smaller than the node
* Right subtree contains values greater than the node
* Duplicate values are not inserted

Operations:

* Iterative insertion
* Search
* Removal using in-order successor
* Height calculation
* In-order traversal
* Level-order visualization

Complexity:

Search: average O(log n), worst O(n)
Insert: average O(log n), worst O(n)
Remove: average O(log n), worst O(n)

---

### 2. AVL Tree

A self-balancing binary search tree.

Key idea:
Each node stores its height and maintains a balance factor defined as:

balance = height(left) - height(right)

Allowed values are -1, 0, or 1. Any violation triggers rebalancing through rotations.

Rotations:

* Left rotation
* Right rotation
* Left-Right rotation
* Right-Left rotation

Operations:

* Recursive insertion with rebalancing
* Recursive removal with rebalancing
* Height tracking at each node
* Search
* Traversal and visualization

Complexity:

Search: O(log n)
Insert: O(log n)
Remove: O(log n)

---

### 3. Red-Black Tree (RBT)

A balanced binary search tree based on node coloring.

Properties:

1. Each node is either red or black
2. The root is always black
3. Red nodes cannot have red children
4. Every path from a node to its descendant leaves contains the same number of black nodes

Implementation approach:

Instead of pointer-based references, this implementation uses a vector of nodes and index-based links.

Each node stores:

* parent index
* left child index
* right child index

A missing child is represented as None, which behaves like a black NIL node.

This approach avoids:

* unsafe code
* borrowing conflicts
* complex lifetime management

Operations:

* Insertion with fix-up (recoloring and rotations)
* Removal with fix-up (case-based balancing)
* Search
* Height calculation
* Traversals

Complexity:

Search: O(log n)
Insert: O(log n)
Remove: O(log n)

---

## Public API Behavior

The main operations return boolean values when appropriate:

- insert(value) returns true if the value was inserted and false if it already exists
- search(value) returns true if the value is found
- remove(value) returns true if the value was removed and false if it does not exist

This design keeps the data structure logic independent from user interaction, allowing the CLI to handle all output messages.

---

## Design Decisions

1. No unsafe code
   All implementations rely strictly on safe Rust constructs.

2. Explicit ownership model

* BST and AVL use Box<Node>
* RBT uses Vec<Node> with indices

3. Conservative implementation style
   The code follows traditional algorithmic structure instead of relying on idiomatic shortcuts. This makes it easier to compare with implementations in languages such as C or Java.

4. Separation of concerns

* Data structure logic is isolated in bst.rs, avl.rs, and rbt.rs
* User interaction is handled exclusively in main.rs

---

## How to Run

From the project root directory:

cargo run --bin bst
cargo run --bin avl
cargo run --bin rbt

---

## CLI Features

Each executable provides:

* Insert value
* Search value
* Remove value
* Calculate tree height
* Print tree in-order
* Print tree by level

---

## Output Representation

In-order traversal:
Displays sorted values.

Example:
1 3 5 7 9

Level view:
Displays the tree structure using spacing and placeholders (n or ---).

Red-Black Tree:
Nodes include color information.

Example:
10B 15R 20B

---

## Test implementation

The test suite covers:

- insertion and search
- duplicate handling
- removal of existing and missing values
- height calculation for empty trees
- AVL balancing behavior under ordered insertions

Tests are implemented for all three structures (BST, AVL, and RBT).

---

## Educational Notes

This project is intended as a study reference.

Key aspects to observe:

* how recursion interacts with ownership
* how rotations restructure the tree
* how AVL and RBT maintain balance differently
* how Rust enforces memory safety in pointer-like structures

---

## Author:

* **Amauri B. S. Junior**

---

## License

This project is intended for educational use.
