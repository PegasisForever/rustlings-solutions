// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Execute `rustlings hint option1` for hints :)


pub fn pop_too_much() -> bool {
    let mut list = vec![3];

    let last = list.pop().unwrap();
    println!("The last item in the list is {:?}", last);

    if let Some(num) = list.pop() {
        println!("The last item in the list is {:?}", num);
    } else {
        println!("The list is empty!")
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert!(pop_too_much());
    }
}
