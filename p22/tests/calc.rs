use p22::calc::{celsius2fahrenheit, fahrenheit2celsius, fibonacci_loop, fibonacci_rec};

#[test]
fn test_celsius2fahrenheit() {
    assert_eq!(celsius2fahrenheit(0), 32);
    assert_eq!(celsius2fahrenheit(100), 212);
}

#[test]
fn test_fahrenheit2celsius() {
    assert_eq!(fahrenheit2celsius(32), 0);
    assert_eq!(fahrenheit2celsius(212), 100);
}

#[test]
fn test_fibonacci_loop() {
    assert_eq!(fibonacci_loop(0), 0);
    assert_eq!(fibonacci_loop(1), 1);
    assert_eq!(fibonacci_loop(5), 5);
    assert_eq!(fibonacci_loop(7), 13);
}

#[test]
fn test_fibonacci_rec() {
    assert_eq!(fibonacci_rec(0), 0);
    assert_eq!(fibonacci_rec(1), 1);
    assert_eq!(fibonacci_rec(5), 5);
    assert_eq!(fibonacci_loop(7), 13);
}
