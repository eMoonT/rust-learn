pub mod animal;
#[cfg(test)]
mod tests {
    use crate::animal::{cat, dog};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn use_cat() {
        assert_eq!(true, cat::is_cat());
    }

    #[test]
    fn use_dog() {
        assert_eq!(false, dog::is_dog());
    }
}
