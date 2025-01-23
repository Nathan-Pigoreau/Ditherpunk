use image::RgbImage;
use rand::Rng;

pub fn tramage_aleatoire(image: &mut RgbImage) {
    let mut rng = rand::thread_rng();
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let luminosite = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
            let seuil = rng.gen_range(0.0..1.0) * 255.0;
            if luminosite > seuil {
                *image.get_pixel_mut(x, y) = image::Rgb([255, 255, 255]);
            } else {
                *image.get_pixel_mut(x, y) = image::Rgb([0, 0, 0]);
            }
        }
    }
    println!("Tramage aléatoire terminé");
}

fn generate_bayer_matrix(order: u32) -> Vec<Vec<u32>> {
    if order == 0 {
        return vec![vec![0]];
    }

    let prev_matrix = generate_bayer_matrix(order - 1);
    let size = 1 << order;
    let mut matrix = vec![vec![0; size]; size];

    for y in 0..size {
        for x in 0..size {
            let quadrant = (y / (size / 2)) * 2 + (x / (size / 2));
            let value = match quadrant {
                0 => 4 * prev_matrix[y % (size / 2)][x % (size / 2)],
                1 => 4 * prev_matrix[y % (size / 2)][x % (size / 2)] + 2,
                2 => 4 * prev_matrix[y % (size / 2)][x % (size / 2)] + 3,
                3 => 4 * prev_matrix[y % (size / 2)][x % (size / 2)] + 1,
                _ => unreachable!(),
            };
            matrix[y][x] = value;
        }
    }

    matrix
}

pub fn monochrome_matrice_bayer(image: &mut RgbImage, order: u32) {
    let matrice_bayer = generate_bayer_matrix(order);
    let size = matrice_bayer.len();
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let luminosite = 0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32;
            let seuil = matrice_bayer[y as usize % size][x as usize % size];
            if luminosite > seuil as f32 {
                *image.get_pixel_mut(x, y) = image::Rgb([255, 255, 255]);
            } else {
                *image.get_pixel_mut(x, y) = image::Rgb([0, 0, 0]);
            }
        }
    }
    println!("Conversion en monochrome avec matrice de Bayer terminée");
}

pub fn diffusion_erreur(image: &mut RgbImage) {
    let width = image.width();
    let height = image.height();
    let mut errors = vec![vec![(0.0, 0.0, 0.0); width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let (r, g, b) = (pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);
            let (er, eg, eb) = errors[y as usize][x as usize];
            let new_r = (r + er).clamp(0.0, 255.0);
            let new_g = (g + eg).clamp(0.0, 255.0);
            let new_b = (b + eb).clamp(0.0, 255.0);
            let luminosite = 0.2126 * new_r + 0.7152 * new_g + 0.0722 * new_b;

            let new_pixel = if luminosite > 128.0 {
                image::Rgb([255, 255, 255])
            } else {
                image::Rgb([0, 0, 0])
            };

            let error_r = new_r - new_pixel[0] as f32;
            let error_g = new_g - new_pixel[1] as f32;
            let error_b = new_b - new_pixel[2] as f32;

            *image.get_pixel_mut(x, y) = new_pixel;

            if x + 1 < width {
                errors[y as usize][(x + 1) as usize].0 += error_r * 0.5;
                errors[y as usize][(x + 1) as usize].1 += error_g * 0.5;
                errors[y as usize][(x + 1) as usize].2 += error_b * 0.5;
            }
            if y + 1 < height {
                errors[(y + 1) as usize][x as usize].0 += error_r * 0.5;
                errors[(y + 1) as usize][x as usize].1 += error_g * 0.5;
                errors[(y + 1) as usize][x as usize].2 += error_b * 0.5;
            }
        }
    }
    println!("Diffusion d'erreur terminée");
}

pub fn diffusion_erreur_floyd_steinberg(image: &mut RgbImage) {
    let width = image.width();
    let height = image.height();
    let mut errors = vec![vec![(0.0, 0.0, 0.0); width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let (r, g, b) = (pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);
            let (er, eg, eb) = errors[y as usize][x as usize];
            let new_r = (r + er).clamp(0.0, 255.0);
            let new_g = (g + eg).clamp(0.0, 255.0);
            let new_b = (b + eb).clamp(0.0, 255.0);
            let luminosite = 0.2126 * new_r + 0.7152 * new_g + 0.0722 * new_b;

            let new_pixel = if luminosite > 128.0 {
                image::Rgb([255, 255, 255])
            } else {
                image::Rgb([0, 0, 0])
            };

            let error_r = new_r - new_pixel[0] as f32;
            let error_g = new_g - new_pixel[1] as f32;
            let error_b = new_b - new_pixel[2] as f32;

            *image.get_pixel_mut(x, y) = new_pixel;

            if x + 1 < width {
                errors[y as usize][(x + 1) as usize].0 += error_r * 7.0 / 16.0;
                errors[y as usize][(x + 1) as usize].1 += error_g * 7.0 / 16.0;
                errors[y as usize][(x + 1) as usize].2 += error_b * 7.0 / 16.0;
            }
            if y + 1 < height {
                if x > 0 {
                    errors[(y + 1) as usize][(x - 1) as usize].0 += error_r * 3.0 / 16.0;
                    errors[(y + 1) as usize][(x - 1) as usize].1 += error_g * 3.0 / 16.0;
                    errors[(y + 1) as usize][(x - 1) as usize].2 += error_b * 3.0 / 16.0;
                }
                errors[(y + 1) as usize][x as usize].0 += error_r * 5.0 / 16.0;
                errors[(y + 1) as usize][x as usize].1 += error_g * 5.0 / 16.0;
                errors[(y + 1) as usize][x as usize].2 += error_b * 5.0 / 16.0;
                if x + 1 < width {
                    errors[(y + 1) as usize][(x + 1) as usize].0 += error_r * 1.0 / 16.0;
                    errors[(y + 1) as usize][(x + 1) as usize].1 += error_g * 1.0 / 16.0;
                    errors[(y + 1) as usize][(x + 1) as usize].2 += error_b * 1.0 / 16.0;
                }
            }
        }
    }
    println!("Diffusion d'erreur avec Floyd-Steinberg terminée");
}
