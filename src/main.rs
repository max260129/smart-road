use std::env;
use std::path::PathBuf;
use ggez::{ContextBuilder, event, conf::WindowMode, conf::WindowSetup};
use crate::game::Game;

mod game;

fn main() {
    // Déterminer le répertoire des ressources
    let resource_dir = match env::var("CARGO_MANIFEST_DIR") {
        Ok(manifest_dir) => {
            let mut path = PathBuf::from(manifest_dir);
            path.push("assets");
            path
        }
        Err(_) => PathBuf::from("./assets"),
    };

    // Créer un Contexte.
    let (mut ctx, event_loop) = ContextBuilder::new("Game", "ML")
        .add_resource_path(resource_dir) // Ajouter le chemin du répertoire des ressources
        .window_setup(WindowSetup {
            title: "Road Intersections".to_owned(), // Titre de la fenêtre
            vsync: true, // Synchronisation verticale activée
            ..Default::default() // Les autres configurations sont par défaut
        })
        .window_mode(WindowMode {
            width: 1008.0, // Largeur de la fenêtre
            height: 1008.0, // Hauteur de la fenêtre
            ..Default::default() // Les autres configurations sont par défaut
        })
        .build() // Construire le Contexte
        .expect("Impossible de créer le contexte ggez!");

    // Créer une instance du gestionnaire d'événements.
    // Normalement, vous devriez lui fournir l'objet Contexte
    // à utiliser lors de la configuration de votre jeu.
    let my_game = Game::new(&mut ctx);

    // Exécuter!
    event::run(ctx, event_loop, my_game);
}

