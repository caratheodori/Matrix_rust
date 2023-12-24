#![allow(non_snake_case)]

#[derive(Debug)]
pub struct Matrix<T>{
    state: Vec<Vec<T>>,
    row: usize,
    col: usize
}
impl<T :Clone + Default> Matrix<T>{
    pub fn new(row: usize, col: usize, initial :T) -> Matrix<T>{
        Matrix{
            state: vec![vec![initial.clone(); row]; col],
            row: row,
            col: col
        }
    }
    pub fn get_row(&self) -> usize{
        return self.row;
    }  
    pub fn get_col(&self) -> usize{
        return self.col;
    }
    pub fn get_state(&self) -> &Vec<Vec<T>>{
        return &self.state;
    }
    /*fn set_state(&mut self, state: Vec<Vec<T>>) -> Result<(),&'static str>{
        if self.get_col() == state.len() && self.get_row() == state[0].len(){
            self.state = state;
            Ok(())
        }else {
            return Err("Both the matrix and the state are not the same size");
        }
    }*/
}

pub trait Add<RHS = Self>{
    type Output;
    fn Add(&self,rhs: RHS) -> Self::Output;
}
impl<T> Add for Matrix<T> where T: std::ops::Add<Output = T> + Clone + Default{
    type Output = Matrix<T>;
    fn Add(&self,rhs: Self) -> Self::Output {
        assert!(self.get_row() == rhs.get_row() && self.get_col() == rhs.get_col(), "The two matrices are not the same size");

        let v: Vec<Vec<T>> = self.get_state().clone();
        let v2: Vec<Vec<T>> = rhs.get_state().clone();
        let mut v3: Vec<Vec<T>> = vec![vec![T::default(); self.get_col()]; self.get_row()];
        for i in 0..self.get_col() {
            for j in 0..self.get_row(){
                v3[i][j] = v[i][j].clone() + v2[i][j].clone();
            }
        }
        Matrix{state: v3, row: self.get_row(), col: self.get_col()}
    }
}


    
