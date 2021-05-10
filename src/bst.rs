#[derive(Debug)]
pub struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    data: T,
}

pub struct BinarySearchTree<T> {
    root: Option<Node<T>>
}

fn inorder<T, F>(root: &Node<T>, f: &mut F) where F: FnMut(&T) {
    match &root.left {
        Some(left) => inorder(&*left, f),
        None => ()
    }

    f(&root.data);

    match &root.right {
        Some(right) => inorder(&*right, f),
        None => ()
    }
}

fn preorder<T, F>(root: &Node<T>, f: &mut F) where F: FnMut(&T) {
    f(&root.data);

    match &root.left {
        Some(left) => preorder(&*left, f),
        None => ()
    }

    match &root.right {
        Some(right) => preorder(&*right, f),
        None => ()
    }
}

fn postorder<T, F>(root: &Node<T>, f: &mut F) where F: FnMut(&T) {
    match &root.left {
        Some(left) => postorder(&*left, f),
        None => ()
    }
    
    match &root.right {
        Some(right) => postorder(&*right, f),
        None => ()
    }

    f(&root.data);
}

impl<T: Ord + Copy> Node<T> {
    fn new(data: T) -> Self {
        Node::<T> { left: None, right: None, data }
    }

    fn insert(&mut self, data: T) {
        if self.data > data {
            match self.left {
                None => self.left = Some(Box::new(Node::new(data))),
                Some(ref mut node) => node.insert(data)
            }
        } else {
            match self.right {
                None => self.right = Some(Box::new(Node::new(data))),
                Some(ref mut node) => node.insert(data)
            }
        }
    }

    // Depth First Search (DFS)
    fn search(&mut self, key: T) -> Option<&Self> {
        if key == self.data {
            return Some(self);
        }

        if key < self.data {
            match self.left {
                Some(ref mut left) => left.search(key),
                None => None,
            }
        } else {
            match self.right {
                Some(ref mut right) => right.search(key),
                None => None,
            }
        }
    }

    fn contains(&mut self, data: T) -> bool {
        if data == self.data {
            return true;
        }

        if data < self.data {
            match self.left {
                Some(ref mut left) => left.contains(data),
                None => false,
            }
        } else {
            match self.right {
                Some(ref mut right) => right.contains(data),
                None => false,
            }
        }
    }
}

impl<T: Ord + Copy> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree::<T> { root: None }
    }

    pub fn insert(&mut self, data: T) -> () {
        let node: Node::<T> = Node::new(data);

        if let Some(root) = &mut self.root {
            root.insert(data);
        } else {
            self.root = Some(node);
        }
    }

    pub fn search(&mut self, key: T) -> Option<&Node<T>> {
        if let Some(root) = &mut self.root {
            return root.search(key);
        }

        None
    }

    pub fn has(&mut self, data: T) -> bool {
        if let Some(root) = &mut self.root {
            return root.contains(data);
        }

        false    
    }

    pub fn inorder<F>(&self, f: &mut F) where F: FnMut(&T) {
        match &self.root {
            Some(root) => inorder(root, f),
            None => ()
        }
    }

    pub fn preorder<F>(&self, f: &mut F) where F: FnMut(&T) {
        match &self.root {
            Some(root) => preorder(root, f),
            None => ()
        }
    }

    pub fn postorder<F>(&self, f: &mut F) where F: FnMut(&T) {
        match &self.root {
            Some(root) => postorder(root, f),
            None => ()
        }
    }
}
