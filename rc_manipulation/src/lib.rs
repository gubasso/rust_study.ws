use std::rc::Rc;
use std::cell::RefCell;

pub fn add(left: usize, right: usize) -> usize {

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = Some(Rc::new(RefCell::new("air".to_owned())));
        let y = x.unwrap();
        y.borrow_mut().push_str(" flow");
        *y.borrow_mut() = "earth".to_owned();
        println!("{:?}", y);
        // assert_eq!(x.unwrap(), "air");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
