use minecraft_boilerplate::MinecraftBoilerplatePlugin;
use particle_life::ParticleLifePlugin;
use valence::prelude::*;

mod minecraft_boilerplate;
mod particle_life;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((MinecraftBoilerplatePlugin, ParticleLifePlugin))
        .run();
}
