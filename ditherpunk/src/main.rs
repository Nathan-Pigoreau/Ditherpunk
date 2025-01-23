mod args;
mod operations;

use image::ImageFormat;
use args::{DitherArgs, Mode};
use operations::affichage::*;
use operations::conversion::*;
use operations::dither::*;
use operations::palette::*;

fn main() {
    let args: DitherArgs = argh::from_env();
    let mut path_in = args.input;
    let mut path_out = args.output.unwrap_or_else(|| "output.png".to_string());
    if !path_out.starts_with("img/"){
        path_out = format!("img/{}", path_out);
    }
    let mode = args.mode;

    //Ouverture de l'image
    let img = loop {
        match image::open(&path_in) {
            Ok(img) => break img,
            Err(_) => {
                eprintln!("Impossible d'ouvrir l'image: {}. Veuillez entrer un nouveau chemin:", path_in);
                let mut new_path = String::new();
                std::io::stdin().read_line(&mut new_path).expect("Erreur de lecture du chemin");
                path_in = new_path.trim().to_string();
            }
        }
    };
    let mut rgb_image = img.to_rgb8();

    match mode {
        Mode::Seuil(_) => {
            let couleur = match demander_couleur("première") {
                Some(couleur) => couleur,
                None => return,
            };
            let couleur2 = match demander_couleur("deuxième") {
                Some(couleur) => couleur,
                None => return,
            };
            convertir_bichrome(&mut rgb_image, couleur, couleur2);
        },
        Mode::Palette(opts) => {
            let mut palette = Vec::new();
            if opts.n_couleurs == 0 {
                eprintln!("Aucune couleur selectionner l'image reste inchangée");
            } else if opts.n_couleurs == 8 {
                palette.push(image::Rgb([0, 0, 0]));
                palette.push(image::Rgb([255, 255, 255]));
                palette.push(image::Rgb([255, 0, 0]));
                palette.push(image::Rgb([0, 255, 0]));
                palette.push(image::Rgb([0, 0, 255]));
                palette.push(image::Rgb([255, 255, 0]));
                palette.push(image::Rgb([0, 255, 255]));
                palette.push(image::Rgb([255, 0, 255]));
                indexer_image_palette(&mut rgb_image, &palette);
            } else {
                for _ in 0..opts.n_couleurs {
                    let couleur = match demander_couleur("prochaine") {
                        Some(couleur) => couleur,
                        None => return,
                    };
                    palette.push(couleur);
                }
                indexer_image_palette(&mut rgb_image, &palette);
            }
        },
        Mode::TramageAleatoire(_) => {
            tramage_aleatoire(&mut rgb_image);
        },
        Mode::AfficherCouleurPixel(opts) => {
            afficher_couleur_pixel(&rgb_image, opts.x, opts.y);
        },
        Mode::ConvertirPixelsBlanc(_) => {
            convertir_pixels_blanc(&mut rgb_image);
            rgb_image.save_with_format("img/demi_blanc.png", ImageFormat::Png).expect("Erreur de sauvegarde");
            println!("Image convertie et sauvegardée avec succès");
        },
        Mode::AfficherLuminositePixel(opts) => {
            afficher_luminosite_pixel(&rgb_image, opts.x, opts.y);
        },
        Mode::Monochrome(_) => {
            convertir_monochrome(&mut rgb_image);
        },
        Mode::MatriceBayer(opts) => {
            monochrome_matrice_bayer(&mut rgb_image, opts.order);
        }
        Mode::DiffusionErreur(_) => {
            diffusion_erreur(&mut rgb_image);
        },
        Mode::DiffusionErreurPalette(opts) => {
            let mut palette = Vec::new();
            if opts.n_couleurs == 0 {
                eprintln!("Aucune couleur selectionner l'image reste inchangée");
            } else if opts.n_couleurs == 8 {
                palette.push(image::Rgb([0, 0, 0]));
                palette.push(image::Rgb([255, 255, 255]));
                palette.push(image::Rgb([255, 0, 0]));
                palette.push(image::Rgb([0, 255, 0]));
                palette.push(image::Rgb([0, 0, 255]));
                palette.push(image::Rgb([255, 255, 0]));
                palette.push(image::Rgb([0, 255, 255]));
                palette.push(image::Rgb([255, 0, 255]));
                diffusion_erreur_palette(&mut rgb_image, &palette);
            } else {
                for _ in 0..opts.n_couleurs {
                    let couleur = match demander_couleur("prochaine") {
                        Some(couleur) => couleur,
                        None => return,
                    };
                    palette.push(couleur);
                }
                diffusion_erreur_palette(&mut rgb_image, &palette);
            }
        },
        Mode::DiffusionErreurFloydSteinberg(_) => {
            diffusion_erreur_floyd_steinberg(&mut rgb_image);
        }
    }
    rgb_image.save_with_format(path_out, ImageFormat::Png).expect("Erreur de sauvegarde");
    // Ok(())
}
