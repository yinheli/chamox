use chamox::obfuscate;

#[test]
fn test_void() {
    #[obfuscate]
    fn void_test() {
        println!("Hello, world!");
    }

    assert_eq!(void_test(), ());
}

#[test]
fn test_string() {
    #[obfuscate]
    fn string_test() -> String {
        "Hello, world!".to_string()
    }

    assert_eq!(string_test(), "Hello, world!");
}

#[test]
fn test_basic_function() {
    #[obfuscate]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(100, 200), 300);
}

#[test]
fn test_multiple_statements() {
    #[obfuscate]
    fn complex_calc() -> i32 {
        let mut result = 0;
        result += 10;
        result *= 2;
        result -= 5;
        result
    }

    assert_eq!(complex_calc(), 15);
}

#[test]
fn test_with_control_flow() {
    #[obfuscate]
    fn fibonacci(n: u32) -> u32 {
        if n <= 1 {
            return n;
        }
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }

    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(10), 55);
}

#[test]
fn test_with_loop() {
    #[obfuscate]
    fn sum_up_to(n: u32) -> u32 {
        let mut sum = 0;
        for i in 1..=n {
            sum += i;
        }
        sum
    }

    assert_eq!(sum_up_to(5), 15);
    assert_eq!(sum_up_to(10), 55);
}

#[test]
fn test_with_function_pointer() {
    #[obfuscate]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let add_ptr = add as fn(i32, i32) -> i32;
    assert_eq!(add_ptr(2, 3), 5);
}

#[test]
fn test_with_attributes() {
    #[obfuscate]
    #[inline]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    assert_eq!(add(2, 3), 5);
}
