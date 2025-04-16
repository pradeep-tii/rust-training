pub fn celsius2fahrenheit(celsius: i32) -> i32 {
    celsius * 9 / 5 + 32
}

pub fn fahrenheit2celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

pub fn fibonacci_loop(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b += temp;
    }
    a
}

pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}
