use async_recursion::async_recursion;
use futures_executor::block_on;

#[async_recursion]
async fn fib(n: u32) -> u64 {
    match n {
        0 => panic!("zero is not a valid argument to fib()!"),
        1 | 2 => 1,
        3 => 2,
        _ => fib(n - 1).await + fib(n - 2).await,
    }
}

struct Node<'a, T> {
    value: T,
    left: Option<&'a Node<'a, T>>,
    right: Option<&'a Node<'a, T>>,
}

impl<T> Node<'_, T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[async_recursion]
async fn contains_value<'a, T>(value: &T, node: &Node<'a, T>) -> bool
where
    T: PartialEq,
{
    if &node.value == value {
        true
    } else {
        (node.left.is_some() && contains_value(value, node.left.unwrap()).await)
            || (node.right.is_some() && contains_value(value, node.right.unwrap()).await)
    }
}

#[async_recursion]
async fn contains_value_2<'a, 'b, T>(value: &'b T, node: &'b Node<'a, T>) -> bool
where
    T: PartialEq,
{
    contains_value(value, node).await
}

struct Empty {}

impl Empty {
    #[async_recursion]
    pub async fn fib(&self, n: u32) -> u64 {
        match n {
            0 => panic!("zero is not a valid argument to fib()!"),
            1 | 2 => 1,
            3 => 2,
            _ => self.fib(n - 1).await + self.fib(n - 2).await,
        }
    }
}

#[test]
fn basic_lifetimes_test() {
    block_on(async move {
        let mut node = Node::new(10);
        let mut left = Node::new(5);
        let left_left = Node::new(3);
        let left_right = Node::new(7);
        let mut right = Node::new(15);
        let right_left = Node::new(13);
        let right_right = Node::new(17);

        left.left = Some(&left_left);
        left.right = Some(&left_right);
        right.left = Some(&right_left);
        right.right = Some(&right_right);

        node.left = Some(&left);
        node.right = Some(&right);

        assert_eq!(contains_value(&3, &node).await, true);
        assert_eq!(contains_value(&4, &node).await, false);
        assert_eq!(contains_value(&17, &node).await, true);
        assert_eq!(contains_value(&13, &node).await, true);
        assert_eq!(contains_value(&12, &node).await, false);

        assert_eq!(contains_value_2(&3, &node).await, true);
        assert_eq!(contains_value_2(&4, &node).await, false);
        assert_eq!(contains_value_2(&17, &node).await, true);
        assert_eq!(contains_value_2(&13, &node).await, true);
        assert_eq!(contains_value_2(&12, &node).await, false);
    });
}

#[test]
fn fibonacci_works() {
    block_on(async move {
        assert_eq!(fib(3).await, 2);
        assert_eq!(fib(4).await, 3);
        assert_eq!(fib(5).await, 5);
        assert_eq!(fib(6).await, 8);
    });
}

#[test]
fn struct_method_fib() {
    block_on(async move {
        let e = Empty {};
        assert_eq!(e.fib(6).await, 8);
        assert_eq!(e.fib(5).await, 5);
        assert_eq!(e.fib(7).await, 13);
    });
}
