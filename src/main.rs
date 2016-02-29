fn main() {
    let fib_num = 5;

    fib_recursive(fib_num);
}

fn fib_recursive(num: i32) -> i32 {
    match num {
        0 =>  0,
        1 =>  1,
        _ => fib_recursive(num-1) + fib_recursive(num-2),
    }
}

#[test]
fn it_works() {
    assert!(fib_recursive(0) == 0);
    assert!(fib_recursive(1) == 1);
    assert!(fib_recursive(5) == 5);
    assert!(fib_recursive(6) == 8);
    assert!(fib_recursive(39) == 63245986);
}
