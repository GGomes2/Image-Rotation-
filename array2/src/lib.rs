
/// Jay Gomes

///A fixed data structure that is 2 dementional that 
// contain data of type T where each element exists at 
// an indes(r,c): row and column.

#[derive(Clone)]
 pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>, 

 }
 
 impl<T: Clone> Array2<T> {

    /// construct a array2 with a single value 
    /// arguments are the dimentions (h,w) and the list of values 
    pub fn new(height: usize, width: usize, data: T )-> Array2<T> {
        let data = vec![vec![data; width]; height];
        Array2 {height,width,data}
    }



       /// constructs an Array2 from elements in row-major-order
       /// the vector given is split into chunks based on the width
       /// and returns a nested vector  
    pub fn from_row_major(height: usize, width: usize, vals: Vec<T>) -> Self {
     
        let data: Vec<Vec<T>> = vals.chunks(width)
        .map(|s| s.to_vec())
        .collect(); 

        Array2 {height,width,data}

    }
        /// Takes the dimentions and the list of values and returns
        /// the nested vector
        /// 
        /// the first elemnt of every vector is my first row
        /// the second element of very vector is my send row ... 
        /// this works but need a less expensive approach
    pub fn from_column_major(height: usize, width: usize, vals: Vec<T>) -> Self {
        
    
        let mut data: Vec<Vec<T>> = Vec::with_capacity(height);

        let vec_vals: Vec<Vec<T>> = vals.chunks(width).map(|s| s.to_vec()).collect(); 
         for i in 0.. height { // loop through the vectors   
            let mut temp = Vec::with_capacity(width);
            for j in 0.. width { // for every vector add the j index to temp  so 
                    temp.push(vec_vals[i][j].clone());
            }
            data.push(temp);
       }
        Array2 {height,width,data}
    }

    /// Takes the array and the coordinates (x,y) and returns an
    /// option, of either none or refrence to the element 
    pub fn get(&self, x: usize, y: usize,) -> Option<&T>{

        if x < self.height && y < self.width {    // check for our bounds 
           Some(&self.data[x][y])  // the element is retuned 
        } else{
         None // out of bounds 
        }

    
    }


    pub fn get_mut(&mut self, x: usize, y: usize,) -> Option<&mut T>{

        if x < self.height && y < self.width {    // check for our bounds 
           Some(&mut self.data[x][y])  // the element is retuned 
        } else{
         None // out of bounds 
        }

    
    }

    /// The function will take in an instance of the array 
    /// as an agrgument, The functions returns an iterator over
    /// the refrences the elemnts of the array in row-major order
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)>{  

            self.data.iter().enumerate().flat_map(|(r_idx, row)| {
                row.iter()
                .enumerate()
                .map(move |(c_idx, val)| (r_idx, c_idx, val))
            })
    }

        /// The function will take in an  instance of the array 
        /// as an agrgument, The functions returns an iterator over
        /// the refrences the elemnts of the array in column-major order
    pub fn iter_column_major(&self) -> impl Iterator<Item =(usize, usize, &T)>{
        
    
        let mut vals = Vec::with_capacity(self.height * self.width);

        for c in 0.. self.width{
            for r in 0.. self.height{
                vals.push((r, c, &self.data[r][c])); 

            }
        }
        vals.into_iter()
    }
        

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

}

