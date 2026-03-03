
## Assignment: iii
### Jay Gomes


- I recieved some help from office hours 
- I spent 7-10 hours working on the assignment 

- I was able to correctly implement my ppmtrans with the ability to make 90 and 180 degree rotations 
using either row-major access or column-major access. I did not implement any other transformation. 



|       | row-major       |column-major      | 
| ------------- | ------------- |--------------| 
| 90-deg rotation | 84.49ms |  99.48ms| 
| 180-deg rotation | 81.31ms |  108.80ms  | 

- Accessing row-major means iterating through memory sequentially, this means less jumping in memory as the row major iteration matches how the data is laid out in memory. 
- Accessing column-major means jumping around in memory, which causes cache misses and slower performance. 

My runtime show both column-major access slower than their counterparts 

180 rotation is faster than 90 for row major. 180 rotation doesnt need to compute 
new row/col and 90 requires more arithmic   