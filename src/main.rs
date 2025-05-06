mod graphics;
mod model;
mod resources;
mod texture;
pub use graphics::run;
pub use model::*;
pub use resources::*;
pub use texture::*;

use pollster;

fn main() {
    pollster::block_on(run());
}
