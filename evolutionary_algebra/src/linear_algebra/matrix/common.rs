
pub type MatIndex = (usize, usize);
pub trait MatOps<E> {
    type Output;
    fn t(&self) -> Self::Output;
    //pub fn t_mut(&mut self);
    fn set_v(&mut self, row: usize, col: usize, v: E);
    fn get_v(&self, row: usize, col: usize) -> E;
    //fn slice(&self, row_from: usize, row_to: usize, col_from: usize, col_to: usize)-> Self::Output;
    fn row(&self) -> usize;
    fn col(&self) -> usize;
}