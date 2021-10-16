extern crate image;
extern crate img_hash;

use img_hash::{ HasherConfig };

fn main() {
    let image1 = image::open("./images/food.jpeg").unwrap();
    let image2 = image::open("./images/food01.jpg").unwrap();
    let image3 = image::open("./images/food02.jpg").unwrap();


    let hasher = HasherConfig::new().to_hasher();

    let hash1 = hasher.hash_image(&image1);
    let hash2 = hasher.hash_image(&image2);
    let hash3 = hasher.hash_image(&image3);


    println!("Image1 hash: {}", hash1.to_base64());
    println!("Image2 hash: {}", hash2.to_base64());
    println!("Image3 hash: {}", hash3.to_base64());
     
    println!("Hamming Distance, image 1 to image 2: {}", hash1.dist(&hash2));
    println!("Hamming Distance, image 2 to image 3: {}", hash2.dist(&hash3));
    println!("Hamming Distance, image 1 to image 3: {}", hash1.dist(&hash3));

}