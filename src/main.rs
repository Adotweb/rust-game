mod graphics;
mod texture;
pub use graphics::run;
pub use texture::*;

use pollster;

fn main() {
    pollster::block_on(run());
}
