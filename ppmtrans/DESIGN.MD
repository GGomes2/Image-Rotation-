
# Jay Gomes
# ppmtrans Design 

ppmstrans will read an ppm image and perform a transformations on it and return a
transformed image to stdout. The program will use the csc411_image crate to read in the image and work with RGB images. 
I will use my Array2 to iterate and transform my image. 


# modules 
- main: will handle command line arguments
- array2: will provide the artitecture for my image data and iterations 
- fn 90_tran: A function to rotate my image 90 degrees 
- fn 180_tran: A function to rotate my image 180 degrees 

# invariants 

- After all transformations the image will mantain the same length (w * h)
- every tranformation creates a new image 
- pixel values are not changed only moved 


# transformations 

- Rotate 90°	(x, y) → (y, height - x - 1)	Rotates image 90° clockwise
- Rotate 180°	(x, y) → (width - x - 1, height - y - 1)	Rotates image 180°
- Transpose (x, y) → (y, x) Reflects across diagonal
- other trnasformations will have print an error message