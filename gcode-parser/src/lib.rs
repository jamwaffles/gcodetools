// TODO: Enforce documentation for both pub and private things

#[macro_use]
mod helpers;
mod line;
mod parsers;
mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
