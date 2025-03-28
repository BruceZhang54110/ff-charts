use front_of_house::hosting;

pub mod front_of_house;

pub fn add(left: u64, right: u64) -> u64 {
    hosting::add_to_waitlist();
    left + right
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
