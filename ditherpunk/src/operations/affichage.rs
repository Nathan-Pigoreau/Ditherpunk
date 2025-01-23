use image::RgbImage;

pub fn afficher_couleur_pixel(image: &RgbImage, x: u32, y: u32) {
    if image.width() > x && image.height() > y {
        let pixel = image.get_pixel(x, y);
        let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
        println!("Couleur du pixel ({}, {}) : R = {}, G = {}, B = {}", x, y, r, g, b);
    } else {
        println!("Les coordonnées ({}, {}) sont en dehors des limites de l'image.", x, y);
    }
}

pub fn afficher_luminosite_pixel(image: &RgbImage, x: u32, y: u32) {
    let pixel = image.get_pixel(x, y);
    let luminosite = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
    println!("Luminosité du pixel ({}, {}) : {}", x, y, luminosite);
}
