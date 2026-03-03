# Jay Gomes

|       | row-major access      |column-major access       | 
| ------------- | ------------- |--------------| 
| 90-deg rotation | 1 |  4    | 
| 180-deg rotation | 1 |    4  | 

Justification: 

My implementation of my Grid is represented in a nested vector. Therefore the vectors are stored on the heap. With row-major acess the elements of the vector have very good spacial locality as the elements are right next to each other. 

column-major access will require a column major iter that reads by columns. On a row major orderd grid, this requires the column iter to make many jumps between the rows causing many cache misses



row_major access - 90° rotation 
1. 2 
2. 0
3. 0
4. 1
5. 1 
6. 95%
7. 1 
8. 10%


column_major access - 90° rotation 
1. 2 
2. 0
3. 0
4. 1
5. 1 
6. 10%
7. 1
8. 90% 

row_major access - 180° rotation 
1. 2 
2. 0
3. 0
4. 1
5. 1 
6. 95%
7. 1 
8. 90%

column_major access - 180° rotation 
1. 2 
2. 0
3. 0
4. 1
5. 1 
6. 10%
7. 1
8. 10% 


|       | row-major       |column-major      | 
| ------------- | ------------- |--------------| 
| 90-deg rotation | 2 |  2| 
| 180-deg rotation | 4 |    1  | 