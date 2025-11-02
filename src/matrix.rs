use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    /// Creates a new matrix filled with the given value
    pub fn new(cols: usize, rows: usize, initial: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![initial; rows * cols],
            rows,
            cols,
        }
    }

    /// Creates a matrix from a vector, checking dimensions
    pub fn from_vec(data: Vec<T>, cols: usize, rows: usize) -> Result<Self, String> {
        if data.len() != rows * cols {
            return Err(format!(
                "Data length {} doesn't match dimensions {}x{}",
                data.len(),
                rows,
                cols
            ));
        }
        Ok(Self { data, rows, cols })
    }

    /// Gets the number of rows
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Gets the number of columns
    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    /// Converts 2D coordinates to linear index
    fn index_of(&self, col: usize, row: usize) -> usize {
        row * self.cols + col
    }

    /// Gets a reference to an element at (col, row)
    pub fn get(&self, col: usize, row: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[self.index_of(col, row)])
        } else {
            None
        }
    }

    /// Gets a mutable reference to an element at (col, row)
    pub fn get_mut(&mut self, col: usize, row: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            let idx = self.index_of(col, row);
            Some(&mut self.data[idx])
        } else {
            None
        }
    }

    /// Sets the value at (col, row)
    pub fn set(&mut self, col: usize, row: usize, value: T) -> Result<(), String> {
        if row < self.rows && col < self.cols {
            let idx = self.index_of(col, row);
            self.data[idx] = value;
            Ok(())
        } else {
            Err(format!("Index ({}, {}) out of bounds", row, col))
        }
    }

    /// Gets a reference to a specific row
    pub fn row(&self, row: usize) -> Option<&[T]> {
        if row < self.rows {
            let start = row * self.cols;
            Some(&self.data[start..start + self.cols])
        } else {
            None
        }
    }

    pub fn keys(&self) -> Vec<(usize, usize)> {
        let mut data = vec![];
        data.reserve_exact(self.rows*self.cols);
        for idy in 0..self.rows {
            for idx in 0..self.cols {
                data.push((idx, idy))
            }
        }
        data
    }

    /// Iterates over all elements
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Iterates over all elements mutably
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }
}

// Implement indexing with (col, row) tuples
impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        &self.data[self.index_of(col, row)]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut Self::Output {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        let idx = self.index_of(col, row);
        &mut self.data[idx]
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let mat = Matrix::new(3, 4, 0);
        assert_eq!(mat.rows(), 4);
        assert_eq!(mat.cols(), 3);
    }

    #[test]
    fn test_matrix_access() {
        let mut mat = Matrix::new(2, 3, 0);
        mat.set(1, 2, 42).unwrap();
        assert_eq!(mat.get(1, 2), Some(&42));
        assert_eq!(mat[(1, 2)], 42);
    }

    #[test]
    fn test_matrix_indexing() {
        let mut mat = Matrix::new(2, 2, 0);
        mat[(0, 0)] = 1;
        mat[(0, 1)] = 2;
        mat[(1, 0)] = 3;
        mat[(1, 1)] = 4;

        assert_eq!(mat[(0, 0)], 1);
        assert_eq!(mat[(1, 1)], 4);
    }
}
