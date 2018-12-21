pub mod cfd;
pub mod fem;
pub mod fvm;
pub mod mesh;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
