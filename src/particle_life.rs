use valence::{entity::armor_stand::ArmorStandEntityBundle, prelude::*};

use crate::minecraft_boilerplate::Equipment;

pub struct ParticleLifePlugin;

impl Plugin for ParticleLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
    }
}

fn setup(
    mut commands: Commands,
    layers_query: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    let layer = layers_query.single();

    let helmet = ItemStack::new(ItemKind::GoldenHelmet, 1, None);
    let equipment = Equipment {
        helmet: Some(helmet),
        chestplate: None,
        leggings: None,
        boots: None,
    };

    commands.spawn((
        ArmorStandEntityBundle {
            layer: EntityLayerId(layer),
            position: Position::new([0.0, 100.0, 0.0]),
            ..Default::default()
        },
        equipment,
    ));
}
