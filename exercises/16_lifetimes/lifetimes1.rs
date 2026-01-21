// the rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. remember, references are borrows and do
// not own their own data. what if their owner goes out of scope?

// todo: fix the compiler error by updating the function signature.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // you can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
