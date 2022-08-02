fn main() {
    for i in 1..101 {
        let mut out = String::new();
        if i % 3 == 0 && i % 5 == 0 {
            out.push_str("FizzBuzz");
        } else if i % 5 == 0 {
            out.push_str("Buzz");
        } else if i % 3 == 0 {
            out.push_str("Fizz");
        } else {
            out.push_str(&i.to_string());
        }
        println!("{}", out);
    }
}
