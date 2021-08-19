use game::Game;
use ggez::event;
use ggez::graphics;
use ggez::ContextBuilder;
use nalgebra::Vector2;

mod audio;
mod card;
mod components;
mod game;
mod tileset;

trait Collision {
    fn inside(&self, pos: Vector2<i32>) -> bool;
}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("freecell", "Freecell")
        .add_resource_path(resource_dir)
        .build()
        .unwrap();
    graphics::set_window_title(&mut ctx, "Freecell");
    let game = Game::new(&mut ctx);
    event::run(ctx, event_loop, game);
}
