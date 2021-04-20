struct TreeNode<T>{
    element: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

fn main(){
    let _a = 10;
    /*let t = vec![1; 10];
    let b: Box<&[u32]> = Box::new(&t);
    //let a = t;
    //let c = &b;
    print!("{:?}",b);
    //print!("{:?}",c); */

    let jup_tree = Some(Box::new(TreeNode{
        element: "aaa",
        left: None,
        right: None
    }));

    let mars_tree =TreeNode{
        element: "bbb",
        left: jup_tree,
        right: None
    }; 
}