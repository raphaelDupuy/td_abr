/// Simple binary search tree
///
/// For every node of value `v`, all elements in the left sub-tree are smaller
/// than `v` and all elements in the right sub-tree are larger than `v`.
#[derive(Debug)]
pub struct Tree<T: Ord>(Option<Box<Node<T>>>);

/// Internal Node representation with a `value` and the left and right sub-trees.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: Ord> Tree<T> {
    /// Returns an empty tree
    pub fn new() -> Self {
        Tree(None)
    }

    /// Returns a tree containing a single value
    fn leaf(value: T) -> Self {
        Tree(Some(Box::new(Node {
            value,
            left: Tree(None),
            right: Tree(None),
        })))
    }

    /// Inserts `value` into the tree.
    /// Returns `false` iff the `value` was already contained in the tree.
    pub fn insert(&mut self, value: T) -> bool {
        panic!("Not implemented")
    }

    /// Returns true if and only if `value` belongs to the tree.
    pub fn contains(&self, value: T) -> bool {
        panic!("Not implemented")
    }

    /// Deletes `value` from the tree.
    /// When the value is not found the tree, `false` is returned.
    pub fn delete(&mut self, value: T) {
        panic!("Not implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_some_simple_trees() {
        let empty_tree = Tree::<i32>(None);
        println!("{:#?}", empty_tree);
        assert!(empty_tree.0.is_none());

        let twelve_as_root = Tree(Some(Box::new(Node {
            value: 12,
            left: Tree(None),
            right: Tree(None),
        })));
        println!("{:#?}", twelve_as_root);
        assert_eq!(twelve_as_root.0.unwrap().value, 12);

        let simple_tree = Tree(Some(Box::new(Node {
            value: 12,
            left: Tree(Some(Box::new(Node {
                value: 8,
                left: Tree(None),
                right: Tree(None),
            }))),
            right: Tree(Some(Box::new(Node {
                value: 27,
                left: Tree(None),
                right: Tree(None),
            }))),
        })));
        println!("{:#?}", simple_tree);
        let root = simple_tree.0.unwrap();
        assert_eq!(root.value, 12);
        assert_eq!(root.left.0.unwrap().value, 8);
        assert_eq!(root.right.0.unwrap().value, 27);
    }

    #[test]
    fn should_build_an_empty_tree() {
        let empty_tree = Tree::<i32>::new();
        assert!(empty_tree.0.is_none());
    }

    #[test]
    fn should_build_a_single_node() {
        let twelve_as_root = Tree::leaf(12);
        assert_eq!(twelve_as_root.0.unwrap().value, 12);
    }

    fn setup_a_tree() -> Tree<i32> {
        Tree(Some(Box::new(Node {
            value: 17,
            left: Tree(Some(Box::new(Node {
                value: 8,
                left: Tree(Some(Box::new(Node {
                    value: 3,
                    left: Tree(None),
                    right: Tree(None),
                }))),
                right: Tree(None),
            }))),
            right: Tree(Some(Box::new(Node {
                value: 27,
                left: Tree(Some(Box::new(Node {
                    value: 22,
                    left: Tree(None),
                    right: Tree(None),
                }))),
                right: Tree(Some(Box::new(Node {
                    value: 55,
                    left: Tree(None),
                    right: Tree(Some(Box::new(Node {
                        value: 83,
                        left: Tree(None),
                        right: Tree(None),
                    }))),
                }))),
            }))),
        })))
    }

    #[test]
    fn should_contains_a_value() {
        let t = setup_a_tree();
        assert!(t.contains(3));
        assert!(t.contains(55));
        assert!(!t.contains(120));
    }

    #[test]
    fn should_insert_a_new_node() {
        let mut t = setup_a_tree();
        assert!(!t.contains(33));
        t.insert(33);
        assert!(t.contains(33));
    }
}
