use image::RgbImage;

pub fn convertir_pixels_blanc(image: &mut RgbImage) {
    let mut modification = false;
    println!("Conversion d'un pixel sur deux en blanc en cours...");
    for (_x, _y, pixel) in image.enumerate_pixels_mut() {
        if modification {
            *pixel = image::Rgb([255, 255, 255]);
            modification = false;
        } else {
            modification = true;
        }
    }
    println!("Conversion terminée");
}

pub fn convertir_monochrome(image: &mut RgbImage) {
    for pixel in image.pixels_mut() {
        let luminosite = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
        if luminosite > 128.0 {
            *pixel = image::Rgb([255, 255, 255]);
        } else {
            *pixel = image::Rgb([0, 0, 0]);
        }
    }
    println!("Conversion en monochrome terminée");
}

pub fn convertir_bichrome(image: &mut RgbImage, couleur1: image::Rgb<u8>, couleur2: image::Rgb<u8>) {
    for pixel in image.pixels_mut() {
        let luminosite = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
        if luminosite > 128.0 {
            *pixel = couleur1;
        } else {
            *pixel = couleur2;
        }
    }
    println!("Conversion terminée");
}
