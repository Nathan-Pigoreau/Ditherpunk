use image::RgbImage;

pub fn distance_couleurs(couleur1: image::Rgb<u8>, couleur2: image::Rgb<u8>) -> f32 {
    let r = couleur1[0] as f32 - couleur2[0] as f32;
    let g = couleur1[1] as f32 - couleur2[1] as f32;
    let b = couleur1[2] as f32 - couleur2[2] as f32;
    (r * r + g * g + b * b).sqrt()
}

pub fn indexer_image_palette(image: &mut RgbImage, palette: &[image::Rgb<u8>]) {
    for pixel in image.pixels_mut() {
        let mut couleur_proche = palette[0];
        let mut distance_min = distance_couleurs(*pixel, palette[0]);
        for &couleur in palette.iter().skip(1) {
            let distance = distance_couleurs(*pixel, couleur);
            if distance < distance_min {
                couleur_proche = couleur;
                distance_min = distance;
            }
        }
        *pixel = couleur_proche;
    }
    println!("Indexation de l'image sur la palette terminée");
}

pub fn diffusion_erreur_palette(image: &mut RgbImage, palette: &[image::Rgb<u8>]) {
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

            let mut couleur_proche = palette[0];
            let mut distance_min = distance_couleurs(image::Rgb([new_r as u8, new_g as u8, new_b as u8]), palette[0]);
            for &couleur in palette.iter().skip(1) {
                let distance = distance_couleurs(image::Rgb([new_r as u8, new_g as u8, new_b as u8]), couleur);
                if distance < distance_min {
                    couleur_proche = couleur;
                    distance_min = distance;
                }
            }

            let error_r = new_r - couleur_proche[0] as f32;
            let error_g = new_g - couleur_proche[1] as f32;
            let error_b = new_b - couleur_proche[2] as f32;

            *image.get_pixel_mut(x, y) = couleur_proche;

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
    println!("Diffusion d'erreur avec palette terminée");
}

pub fn demander_couleur(nom: &str) -> Option<image::Rgb<u8>> {
    println!("Veuillez entrer la {} couleur de la palette, les couleurs disponibles sont : NOIR(N), BLANC(BC), ROUGE(R), VERT(V), BLEU(BU), JAUNE(J), CYAN(C), MAGENTA(M) :", nom);
    let mut couleur = String::new();
    std::io::stdin().read_line(&mut couleur).expect("Erreur de lecture de la couleur");
    couleur = couleur.to_uppercase();
    let couleur = couleur.trim();
    match couleur {
        "NOIR" => Some(image::Rgb([0, 0, 0])),
        "BLANC" => Some(image::Rgb([255, 255, 255])),
        "ROUGE" => Some(image::Rgb([255, 0, 0])),
        "VERT" => Some(image::Rgb([0, 255, 0])),
        "BLEU" => Some(image::Rgb([0, 0, 255])),
        "JAUNE" => Some(image::Rgb([255, 255, 0])),
        "CYAN" => Some(image::Rgb([0, 255, 255])),
        "MAGENTA" => Some(image::Rgb([255, 0, 255])),
        "N" => Some(image::Rgb([0, 0, 0])),
        "BC" => Some(image::Rgb([255, 255, 255])),
        "R" => Some(image::Rgb([255, 0, 0])),
        "V" => Some(image::Rgb([0, 255, 0])),
        "BU" => Some(image::Rgb([0, 0, 255])),
        "J" => Some(image::Rgb([255, 255, 0])),
        "C" => Some(image::Rgb([0, 255, 255])),
        "M" => Some(image::Rgb([255, 0, 255])),
        "QUIT" => None,
        _ => {
            eprintln!("Couleur inconnue. Les couleurs disponibles sont : NOIR(N), BLANC(BC), ROUGE(R), VERT(V), BLEU(BU), JAUNE(J), CYAN(C), MAGENTA(M)");
            None
        }
    }
}
