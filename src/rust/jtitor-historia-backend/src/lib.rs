#[macro_use] extern crate failure;

mod error;
mod io;
mod model;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
