use argh::FromArgs;

#[derive(FromArgs)]
/// Arguments for the ditherpunk application.
pub struct DitherArgs {
    #[argh(option)]
    /// input file path
    pub input: String,

    #[argh(option)]
    /// output file path
    pub output: Option<String>,

    #[argh(subcommand)]
    pub mode: Mode,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Mode {
    Seuil(SeuilArgs),
    Palette(PaletteArgs),
    TramageAleatoire(TramageAleatoireArgs),
    AfficherCouleurPixel(AfficherCouleurPixelArgs),
    ConvertirPixelsBlanc(ConvertirPixelsBlancArgs),
    AfficherLuminositePixel(AfficherLuminositePixelArgs),
    Monochrome(MonochromeArgs),
    MatriceBayer(MatriceBayerArgs),
    DiffusionErreur(DiffusionErreurArgs),
    DiffusionErreurPalette(DiffusionErreurPaletteArgs),
    DiffusionErreurFloydSteinberg(DiffusionErreurFloydSteinbergArgs),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
pub struct SeuilArgs {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
pub struct PaletteArgs {

    /// le nombre de couleurs à utiliser (maximum 8)
    #[argh(option)]
    pub n_couleurs: usize
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="tramage_aleatoire")]
/// Rendu de l’image par tramage aléatoire.
pub struct TramageAleatoireArgs {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="afficher_couleur_pixel")]
/// Affiche la couleur d'un pixel donné.
pub struct AfficherCouleurPixelArgs {
    /// coordonnée x du pixel
    #[argh(option)]
    pub x: u32,

    /// coordonnée y du pixel
    #[argh(option)]
    pub y: u32,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="convertir_pixels_blanc")]
/// Convertit un pixel sur deux en blanc de l'image passer en paramètre.
pub struct ConvertirPixelsBlancArgs {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="afficher_luminosite_pixel")]
/// Affiche la luminosité d'un pixel donné.
pub struct AfficherLuminositePixelArgs {
    /// coordonnée x du pixel
    #[argh(option)]
    pub x: u32,

    /// coordonnée y du pixel
    #[argh(option)]
    pub y: u32,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="monochrome")]
/// Convertit l'image en monochrome.
pub struct MonochromeArgs {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="matrice_bayer")]
/// Convertit l'image en monochrome avec une matrice de Bayer.
pub struct MatriceBayerArgs {
    /// ordre de la matrice de Bayer
    #[argh(option)]
    pub order: u32
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="diffusion_erreur")]
/// Convertit l'image en monochrome avec diffusion d'erreur.
pub struct DiffusionErreurArgs {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="diffusion_erreur_palette")]
/// Convertit l'image en monochrome avec diffusion d'erreur et une palette de couleurs.
pub struct DiffusionErreurPaletteArgs {
    /// le nombre de couleurs à utiliser (maximum 8)
    #[argh(option)]
    pub n_couleurs: usize
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="diffusion_erreur_floyd_steinberg")]
/// Convertit l'image en monochrome avec diffusion d'erreur de Floyd-Steinberg.
pub struct DiffusionErreurFloydSteinbergArgs {}
