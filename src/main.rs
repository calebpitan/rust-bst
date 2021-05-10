mod bst;

pub use self::bst::{BinarySearchTree};

fn main() {
    // let numbers = [16, 128, 175, 26, 94, 289, 185];
    let numbers = [27, 14, 35, 10, 19, 31, 42, 16, 128, 175, 26, 94, 289, 185];
    let mut bst = BinarySearchTree::<i32>::new();

    for number in numbers.iter() {
        bst.insert(*number);
    }

    println!("{:?}", bst.search(42).unwrap());

    println!("In-order");
    bst.inorder(&mut |d| println!("BST In.O.: {}", d));

    println!("\nPre-order");
    bst.preorder(&mut |d| println!("BST Pr.O.: {}", d));

    println!("\nPost-order");
    bst.postorder(&mut |d| println!("BST Po.O: {}", d));
}
