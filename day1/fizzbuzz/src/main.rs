fn main() {
    for i in 1..=100 {
        println!("{}",  fizzbuzz(i));
    }
}

fn fizzbuzz(i: u32) -> String {
    if i % 15 == 0 {
        return String::from("FizzBuzz")
    }
    if i % 3 == 0 {
        return String::from("Fizz")
    }
    if i % 5 == 0 {
        return String::from("Buzz")
    }
    return format!("{}", i)
}

#[test]
fn test_fizzbuzz() {
    assert_eq!("1", fizzbuzz(1));
    assert_eq!("Fizz", fizzbuzz(3));
    assert_eq!("Buzz", fizzbuzz(5));
    assert_eq!("FizzBuzz", fizzbuzz(15));
}
