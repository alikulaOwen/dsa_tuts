// factorial
// n! = n * (n-1)!
// fact(n) = n*fact(n-1)

use std::collections::HashMap;

fn factorail (num: i32)-> i32 {
    if num > 1 {
        return num * factorail(num - 1);
    }else {
        return 1;
    }
}
 fn fib(num: i32)-> i32 {
    if num == 0 {
        return 0;
    }
    if num == 1{
        return 1
    }
    return  fib(num-1 ) + fib (num -2)
 }
 fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())   
}

fn fib_iterative(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=num {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
fn fib_memoized(num: i32)-> i32{

    fn fib(num: i32, hashmap: &mut HashMap<i32, i32>) -> i32 {

        if num == 0 {
            return 0;
        }
        if num == 1 {
            return 1;
        }
        if let Some(&result) = hashmap.get(&num) {
            return result;
        }
        let result = fib(num - 1, hashmap) + fib(num - 2, hashmap);
        hashmap.insert(num, result);
        result
    }
    let mut hashmap = HashMap::new();
    fib(num, &mut hashmap)

}
fn is_palindrome_recursive(s: &str) -> bool {
    let len = s.len();
    if len <= 1 {
        return true;
    }
    let first_char = s.chars().next().unwrap();
    let last_char = s.chars().rev().next().unwrap();
    if first_char != last_char {
        return false;
    }
    return is_palindrome_recursive(&s[1..len-1]);
}

fn is_palindrome_iterative (s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = bytes.len() - 1;
    while start < end {
        if bytes[start] != bytes[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    return true;
}

fn tower_of_hanoi(n: i32, from: char, to: char, aux: char) {
    if n == 1 {
        println!("Move disk 1 from rod {} to rod {}", from, to);
        return;
    }
    tower_of_hanoi(n - 1, from, aux, to);
    println!("Move disk {} from rod {} to rod {}", n, from, to);
    tower_of_hanoi(n - 1, aux, to, from);
}

fn main() {
    println!("{}", factorail(5));    
    println!("Fibonacci ==>  {}", fib(10));
    println!("Palindrome ==>  {}", is_palindrome_iterative("madam"));
    println!("Palindrome ==>  {}", is_palindrome_recursive("madam"));
    println!("Palindrome ==>  {}", is_palindrome("madarm"));
    println!("Fibonacci Iterative ==>  {}", fib_iterative(10));
    
    println!("Fibonacci Memoized ==>  {}", fib_memoized(10));

    tower_of_hanoi(3, 'A', 'C', 'B');
}

