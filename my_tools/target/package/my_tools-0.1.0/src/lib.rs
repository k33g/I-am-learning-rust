mod all_tools {

    pub mod small {
        pub fn add(left: usize, right: usize) -> usize {
            left + right
        }
    }

    pub mod big {
        pub fn hello() {
            println!("👋 Hello, world! 🌍");
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = all_tools::small::add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_again() {
        all_tools::big::hello();
        assert_eq!(true, true);
    }
}
