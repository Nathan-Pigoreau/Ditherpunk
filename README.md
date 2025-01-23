# Ditherpunk: Retour au Monochrome

**Binôme : Nathan Pigoreau & Malleron Daniel**

### Description du projet

Dans ce projet, nous allons manipuler des images en utilisant la bibliothèque **Rust `image`** pour appliquer plusieurs traitements visant à transformer une image en une version réduite à une palette de couleurs. Ces traitements incluent la conversion d'images en monochrome, l'application d'une palette de couleurs, ainsi que l'utilisation de techniques de tramage telles que le **dithering**.

L'énoncé de ce TP est largement inspiré de la page [Ditherpunk](https://surma.dev/things/ditherpunk/) et nous avons pour objectif de créer une application en ligne de commande permettant de :
- Sélectionner une image en entrée.
- Sélectionner un nom pour le fichier de sortie (par défaut, `out.png`).
- Choisir et appliquer différents traitements à l'image.

### Objectifs

1. **Manipulation des images avec la bibliothèque `image`** : ouvrir, transformer, et sauvegarder des images.
2. **Passage en monochrome** : convertir une image en noir et blanc par seuil ou par palettisation.
3. **Tramage (Dithering)** : utiliser des techniques de tramage pour ajouter du bruit et des nuances dans les images.

## Installation et lancement
1. Clonez le dépôt : 
```bash
git clone https://github.com/Nathan-Pigoreau/Ditherpunk.git
```
2. Assurez-vous d'avoir **Rust** installé sur votre machine. Si ce n'est pas le cas, vous pouvez l'installer depuis [https://www.rust-lang.org/](https://www.rust-lang.org/).
3. Aller dans le dossier du projet rust :
```bash
cd  Ditherpunk/ditherpunk
```
4. Exécutez le programme :
```sh
cargo run -- --help
```

Pour plus de détails sur l'utilisation des différentes commandes et options, veuillez vous référer à la section [Utilisation](#utilisation).

## Utilisation

Vous pouvez exécuter l'application avec différentes options et sous-commandes. Voici les commandes possibles :

### Commande générale

```sh
cargo run -- --input <chemin/vers/image> --output [output] <mode>
```

- `<input>` : Le fichier d’entrée (obligatoire).
- `[output]` : Le fichier de sortie (optionnel, par défaut "output.png").
- `<mode>` : Le mode d’opération (voir ci-dessous).

### Modes d'opération

#### Seuil

Convertit l'image en utilisant un seuillage monochrome.

```sh
cargo run -- --input <chemin/vers/image> --output [output] seuil
```

Vous serez invité à entrer deux couleurs pour le seuillage.

#### Palette

Convertit l'image en utilisant une palette contenant un nombre limité de couleurs.

```sh
cargo run -- --input <chemin/vers/image> --output [output] palette --n-couleurs <nombre_de_couleurs>
```

- `--n-couleurs` : Le nombre de couleurs à utiliser (maximum 8).

Si `n-couleurs` est 8, les couleurs utilisées seront : NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA. Sinon, vous serez invité à entrer les couleurs manuellement.

#### Tramage Aléatoire

Convertit l'image en utilisant un tramage aléatoire.

```sh
cargo run -- --input <chemin/vers/image> --output [output] tramage_aleatoire
```

#### Afficher Couleur Pixel

Affiche la couleur d'un pixel donné.

```sh
cargo run -- --input <chemin/vers/image> --output [output] afficher_couleur_pixel --x <coordonnée_x> --y <coordonnée_y>
```

- `--x` : Coordonnée x du pixel.
- `--y` : Coordonnée y du pixel.

#### Convertir Pixels Blanc

Convertit un pixel sur deux en blanc.

```sh
cargo run -- --input <chemin/vers/image> --output [output] convertir_pixels_blanc
```

#### Afficher Luminosité Pixel

Affiche la luminosité d'un pixel donné.

```sh
cargo run -- --input <chemin/vers/image> --output [output] afficher_luminosite_pixel --x <coordonnée_x> --y <coordonnée_y>
```

- `--x` : Coordonnée x du pixel.
- `--y` : Coordonnée y du pixel.

#### Matrice de Bayer

Convertit l'image en utilisant une matrice de Bayer.

```sh
cargo run -- --input <chemin/vers/image> --output [output] matrice_bayer --order <ordre>
```

- `--order` : Ordre de la matrice de Bayer.

#### Diffusion d'Erreur

Convertit l'image en utilisant la diffusion d'erreur.

```sh
cargo run -- --input <chemin/vers/image> --output [output] diffusion_erreur
```

#### Diffusion d'Erreur avec Palette

Convertit l'image en utilisant la diffusion d'erreur et une palette de couleurs.

```sh
cargo run -- --input <chemin/vers/image> --output [output] diffusion_erreur_palette --n-couleurs <nombre_de_couleurs>
```

- `--n-couleurs` : Le nombre de couleurs à utiliser (maximum 8).

#### Diffusion d'Erreur Floyd-Steinberg

Convertit l'image en utilisant la diffusion d'erreur de Floyd-Steinberg.

```sh
cargo run -- --input <chemin/vers/image> --output [output] diffusion_erreur_floyd_steinberg
```

## Exemples

### Exemple 1 : Seuillage monochrome

```sh
cargo run -- --input img/IUT.jpg --output img/output.png seuil
```

### Exemple 2 : Palette de 4 couleurs

```sh
cargo run -- --input img/IUT.jpg --output img/output.png palette --n-couleurs 4
```

### Exemple 3 : Tramage aléatoire

```sh
cargo run -- --input img/IUT.jpg --output img/output.png tramage_aleatoire
```

### Exemple 4 : Afficher la couleur d'un pixel

```sh
cargo run -- --input img/IUT.jpg --output img/output.png afficher_couleur_pixel --x 32 --y 52
```

### Exemple 5 : Convertir un pixel sur deux en blanc

```sh
cargo run -- --input img/IUT.jpg --output img/output.png convertir_pixels_blanc
```

### Exemple 6 : Afficher la luminosité d'un pixel

```sh
cargo run -- --input img/IUT.jpg --output img/output.png afficher_luminosite_pixel --x 32 --y 52
```

### Exemple 7 : Matrice de Bayer

```sh
cargo run -- --input img/IUT.jpg --output img/output.png matrice_bayer --order 3
```

### Exemple 8 : Diffusion d'Erreur

```sh
cargo run -- --input img/IUT.jpg --output img/output.png diffusion_erreur
```

### Exemple 9 : Diffusion d'Erreur avec Palette

```sh
cargo run -- --input img/IUT.jpg --output img/output.png diffusion_erreur_palette --n-couleurs 4
```

### Exemple 10 : Diffusion d'Erreur Floyd-Steinberg

```sh
cargo run -- --input img/IUT.jpg --output img/output.png diffusion_erreur_floyd_steinberg
```

### Questions de cours

#### 1 La bibliothèque image

- Question 1

    Pour créer un nouveau projet Cargo avec une dépendance sur la bibliothèque `image` version 0.24, utilisez la commande suivante :
    ```sh
    cargo new ditherpunk
    cd ditherpunk
    cargo add image@0.24
    ```

- Question 2

    Pour ouvrir une image depuis un fichier, on utilise
    ```rust
    ImageReader::open("myimage.png")?.decode()?;
    ```
    On obtient un `DynamicImage`, à quoi correspond ce type? Comment obtenir une image en mode `rgb8` à partir de ce `DynamicImage`?

    Le type `DynamicImage` est une enum de la bibliothèque `image` de Rust qui peut représenter différentes formes d'images (grayscale, RGB, RGBA, etc.). Il permet de manipuler des images de manière générique sans se soucier du format spécifique.

    Pour obtenir une image en mode `rgb8` à partir de ce `DynamicImage`, vous pouvez utiliser la méthode `to_rgb8()` comme suit :
    ```rust
    let img = ImageReader::open("myimage.png")?.decode()?;
    let rgb_image = img.to_rgb8();
    ```
    Cela convertira l'image en un format RGB 8 bits par canal.

- Question 3

    Pour sauvegarder l'image obtenue au format PNG, vous pouvez utiliser la méthode `save` de la bibliothèque `image` comme suit :

    ```rust
    rgb_image.save("output.png")?;
    ```

    Si l'image de départ avait un canal alpha (transparence), celui-ci sera ignoré lors de la conversion en `rgb8`, car ce format ne supporte pas la transparence. En conséquence, l'image sauvegardée au format PNG n'aura pas de canal alpha et les zones transparentes de l'image originale seront converties en noir.

#### 2 Passage en monochrome par seuillage

- Question 6

   Pour récupérer la luminosité d'un pixel, utilisez la formule pondérée suivante, qui reflète la sensibilité de l'œil humain aux    différentes couleurs :
   
   ```markdown
   Luminosité = 0.299 * R + 0.587 * G + 0.114 * B
   ```
   
   Étapes :
   1. Extraire les valeurs R, G, et B du pixel.
   2. Appliquer la formule pour obtenir la luminosité.
   
   Cette méthode permet de convertir un pixel en couleur en une valeur de luminosité, utile pour des applications comme le passage en monochrome par seuillage.


#### 3 Passage à une palette

- Question 9

    Pour calculer la distance entre deux couleurs, nous utilisons la distance euclidienne dans l'espace des couleurs RGB. La formule de calcul est la suivante :

    ```rust
    pub fn distance_couleurs(couleur1: image::Rgb<u8>, couleur2: image::Rgb<u8>) -> f32 {
        let r = couleur1[0] as f32 - couleur2[0] as f32;
        let g = couleur1[1] as f32 - couleur2[1] as f32;
        let b = couleur1[2] as f32 - couleur2[2] as f32;
        (r * r + g * g + b * b).sqrt()
    }
    ```

    Cette méthode calcule la distance en prenant la racine carrée de la somme des carrés des différences des composantes rouge, verte et bleue des deux couleurs. Cela permet de mesurer à quel point deux couleurs sont similaires ou différentes.

- Question 11

    Si la palette est vide, le message "Aucune couleur sélectionnée, l'image reste inchangée" sera affiché. Cela signifie que l'image d'entrée ne subira aucune transformation et sera sauvegardée telle quelle. Ce choix a été fait pour éviter des erreurs ou des comportements imprévisibles lors de l'application des traitements. En l'absence de couleurs définies dans la palette, il est logique de conserver l'image originale sans modification.

#### 5 Utilisation de la matrice de Bayer comme trame

- Question 13
    Déterminer 𝐵3 :
    
    Pour déterminer la matrice de Bayer d'ordre 3, nous devons  appliquer la définition récursive donnée.
    
    1. 𝐵0 = (0)
    
    2. 𝐵1 = (1/4) * 
    ```
    | 0  3 |
    | 2  1 |
    ```
    
    3. 𝐵2 = (1/16) * 
    ```
    |  0  12  3  15 |
    |  8   4 11   7 |
    |  2  14  1  13 |
    | 10   6  9   5 |
    ```
    4. 𝐵3 = (1/64) * 
    ```
    |  0  48  12  60  3  51  15  63 |
    | 32  16  44  28 35  19  47  31 |
    |  8  56   4  52 11  59   7  55 |
    | 40  24  36  20 43  27  39  23 |
    |  2  50  14  62  1  49  13  61 |
    | 34  18  46  30 33  17  45  29 |
    | 10  58   6  54  9  57   5  53 |
    | 42  26  38  22 41  25  37  21 |
    ```
    (Arrondi à 0.001 près)
    ```
    =
    | 0.000 0.750 0.188 0.938 0.047 0.797 0.234 0.984 |
    | 0.500 0.250 0.688 0.438 0.547 0.297 0.734 0.484 |
    | 0.125 0.875 0.063 0.813 0.172 0.922 0.109 0.859 |
    | 0.625 0.375 0.563 0.313 0.672 0.422 0.609 0.359 |
    | 0.031 0.781 0.219 0.969 0.016 0.766 0.203 0.953 |
    | 0.531 0.281 0.719 0.469 0.516 0.266 0.703 0.453 |
    | 0.156 0.906 0.094 0.844 0.141 0.891 0.078 0.828 |
    | 0.656 0.406 0.594 0.344 0.641 0.391 0.578 0.328 |
    ```

- Question 14

    Pour représenter la matrice de Bayer, nous pouvons utiliser un type de données en deux dimensions, tel qu'un vecteur de vecteurs (`Vec<Vec<f64>>`) en Rust. Cela permet de stocker les valeurs de la matrice de manière flexible et dynamique.
    Pour créer une matrice de Bayer d'ordre arbitraire, nous pouvons utiliser une fonction récursive qui génère la matrice en suivant la définition récursive donnée.

#### 6 Diffusion d'erreur

- Question 17
    
    Pour une palette de couleurs, l'erreur commise à chaque pixel est représentée par la différence entre la couleur réelle du pixel et la couleur de la palette qui a été choisie pour le remplacer. Cette erreur est un vecteur à trois composantes (rouge, vert, bleu).

    Pour diffuser cette erreur, nous utilisons une matrice de diffusion d'erreur. Par exemple, pour la matrice de diffusion d'erreur suivante :

    ```
    | *  | 7/16 |
    | 3/16 | 5/16 | 1/16 |
    ```

    L'erreur est répartie de la manière suivante :
    - 7/16 de l'erreur est ajoutée au pixel à droite du pixel courant.
    - 3/16 de l'erreur est ajoutée au pixel en bas à gauche du pixel courant.
    - 5/16 de l'erreur est ajoutée au pixel en bas du pixel courant.
    - 1/16 de l'erreur est ajoutée au pixel en bas à droite du pixel courant.

    Cette diffusion d'erreur permet de répartir l'erreur de quantification sur les pixels voisins, ce qui permet d'obtenir une image plus lisse et moins bruitée.

    Pour implémenter cette diffusion d'erreur, nous devons :
    1. Calculer l'erreur de quantification pour chaque pixel.
    2. Répartir cette erreur sur les pixels voisins en utilisant les coefficients de la matrice de diffusion d'erreur.
    3. Mettre à jour les valeurs des pixels voisins en ajoutant l'erreur diffusée.

    Cette méthode permet d'obtenir des images avec une meilleure qualité visuelle en réduisant les artefacts de quantification.

## Auteurs

- [Pigoreau Nathan](https://github.com/Nathan-Pigoreau)
- [Malleron Daniel]()
