use argh::FromArgs;
use image::{GenericImageView, ImageFormat};

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {

    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}


#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {

    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize
}

fn main() {
    let args: DitherArgs = argh::from_env();
    let path_in = args.input;
    let mut path_out = args.output.unwrap_or_else(|| "output.png".to_string());
    if !path_out.contains("/img/"){
        path_out = format!("img/{}", path_out);
    }
    let mode = args.mode;
    let img = image::open(path_in).expect("Impossible d'ouvrir l'image");
    let rgb_image = img.to_rgb8();
    rgb_image.save_with_format(path_out, ImageFormat::Png).expect("Erreur de sauvegarde");
    println!("Image convertie et sauvegardée avec succès");
    // Ok(())
}

