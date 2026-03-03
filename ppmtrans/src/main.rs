

 use csc411_image::{Read, Rgb, RgbImage, Write};
use std::time::Instant;
 use array2::Array2;

use clap::Parser;
    #[derive(Parser, Debug)]
    #[clap(author, version, about, long_about = None)]

    struct Args {

    //path to the input image
    input: Option<String>, 

    // Flip
    #[clap(short = 'f', long = "flip", required = false)]
    flip: Option<String>,

    //rotate
    #[clap(short = 'r', long = "rotate")]
    rotate: Option<u32>,

    // Transposition
    #[clap(short = 't',long = "transpose")]
    transpose: bool,
    
    #[clap(long = "row-major")]
    row_major: bool,

    #[clap(long = "col-major")]
    col_major: bool,

 }






fn main() {
   
    //let input = env::args().nth(1);
     let args = Args::parse();
    //assert!(env::args().len() == 2);
   // let img = RgbImage::read(Some(&args.input)).unwrap(); // requires panic on error
   let img = match RgbImage::read(args.input.as_deref()) {
    Ok(img) => img,
    Err(e) => {
        eprintln!("Failed to read image: {}", e);
        std::process::exit(1);
    }
};
    //println!("{:#?}", img.pixels );
    //println!("{}", img.pixels.len());
   // let w = img.width.try_into().expect("width too large for usize");
    //let h = img.height.try_into().expect("width too large for usize");

    let w = match img.width.try_into() {
    Ok(w) => w,
    Err(_) => {
        eprintln!("Image width too large for usize");
        std::process::exit(1);
    }
};
let h = match img.height.try_into() {
    Ok(h) => h,
    Err(_) => {
        eprintln!("Image height too large for usize");
        std::process::exit(1);
    }
};
    let og_img = Array2::from_row_major(h,w,img.pixels);  // construct my array from gray image
    let is_row_major = args.row_major || (!args.row_major && !args.col_major);

   


    // let mut transformed = og_img.clone();


    // if args.row_major {
    //     transformed = copy_row_major(&og_img);
    // } else if args.col_major {
    //     transformed = copy_col_major(&og_img);
    // }


     let transformed = if is_row_major {
        copy_row_major(&og_img)
    } else {
        copy_col_major(&og_img)
    };

    

    //let top_left = og_img.get(0,0).unwrap();
   // println!("Top-left pixel: {:?}", top_left);
      // let mut output = transformed.clone(); 


    let output  = if let Some(angle) = args.rotate {
                
    let now = Instant::now(); // start timer

        let rotated = match angle {
        0 => transformed.clone(),
        90 if is_row_major => rotate_90_row_major(&transformed),
        90 => rotate_90_col_major(&transformed),
        180 if is_row_major => rotate_180_row_major(&transformed),
        180 => rotate_180_col_major(&transformed),
        _ => {
            eprintln!("Unsupported rotation angle: {}", angle);
            std::process::exit(1);
        }
    };

    let elapsed = now.elapsed(); // stop timer
    eprintln!("Rotation elapsed time: {:.2?}", elapsed);

    rotated
} else {
    transformed
};




   let output_img = array2_to_image(&output);

    //output_img.write(None).unwrap();
    if let Err(e) = output_img.write(None) {
    eprintln!("Failed to write output image: {}", e);
    std::process::exit(1);
}


}


fn rotate_90_row_major(src: &Array2<Rgb>) -> Array2<Rgb> {
    let h = src.height();
    let w = src.width();
    let mut vals = vec![Rgb { red: 0, green: 0, blue: 0 }; h * w];


    for (r, c, pixel) in src.iter_row_major() {
        // Mapping:
        // (old_row, old_col) -> (new_row = old_col, new_col = height - 1 - old_row)
        let new_row = c;
        let new_col = h - 1 - r;

        let index = new_row * h + new_col;
        vals[index] = pixel.clone();
    }

    Array2::from_row_major(w, h, vals)

} 

fn rotate_90_col_major(src: &Array2<Rgb>) -> Array2<Rgb> {
    let h = src.height();
    let w = src.width();
    let mut vals = vec![Rgb { red: 0, green: 0, blue: 0 }; h * w];


     for (r, c, pixel) in src.iter_column_major() {
        // same mapping as row-major
        let new_row = c;
        let new_col = h - 1 - r;

        let index = new_row * h + new_col;
        vals[index] = pixel.clone();
    }

    Array2::from_row_major(w, h, vals)
}

fn rotate_180_row_major(src: &Array2<Rgb>) -> Array2<Rgb> {
    let h = src.height();
    let w = src.width();
    let mut vals = vec![Rgb { red: 0, green: 0, blue: 0 }; h * w];

    for (r, c, pixel) in src.iter_row_major() {
        let new_row = h - 1 - r;
        let new_col = w - 1 - c;

        let index = new_row * w + new_col;
        vals[index] = pixel.clone();
    }

    Array2::from_row_major(h, w, vals)
}

fn rotate_180_col_major(src: &Array2<Rgb>) -> Array2<Rgb> {
    let h = src.height();
    let w = src.width();
    let mut vals = vec![Rgb { red: 0, green: 0, blue: 0 }; h * w];

    for (r, c, pixel) in src.iter_column_major() {
        let new_row = h - 1 - r;
        let new_col = w - 1 - c;

        let index = new_row * w + new_col;
        vals[index] = pixel.clone();
    }

    Array2::from_row_major(h, w, vals)
}





fn copy_row_major(src: &Array2<Rgb>) -> Array2<Rgb> {
    let mut temp = Array2::new(src.height(), src.width(), Rgb { red: 0, green: 0, blue: 0 });
    for (r, c, pix) in src.iter_row_major() {
        if let Some(temp_pix) = temp.get_mut(r, c) {
            *temp_pix = pix.clone();
        }
    }
    temp
}

fn copy_col_major(src: &Array2<Rgb>) -> Array2<Rgb> {
    let mut temp = Array2::new(src.height(), src.width(), Rgb { red: 0, green: 0, blue: 0 });
    for (r, c, pix) in src.iter_column_major() {
        if let Some(temp_pix) = temp.get_mut(r, c) {
            *temp_pix = pix.clone();
        }
    }
    temp 
}

fn array2_to_image(src: &Array2<Rgb>) -> RgbImage {
    let h = src.height();
    let w = src.width();
    let mut pixels = Vec::with_capacity(h * w);

    for (_, _, pixel) in src.iter_row_major() {
        pixels.push(pixel.clone());
    }

    RgbImage {
        pixels,
        width: w as u32,
        height: h as u32,
        denominator: 255, 

    }
}

