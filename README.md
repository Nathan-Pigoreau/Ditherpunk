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
```bash
cargo run
```

