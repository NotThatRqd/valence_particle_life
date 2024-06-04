use valence::{
    entity::{armor_stand::ArmorStandEntityBundle, entity::Flags},
    nbt::{compound, List},
    prelude::*,
};

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

    let head = ItemStack::new(
        ItemKind::PlayerHead,
        1,
        Some(compound! {
            "SkullOwner" => compound! {
                "Id" => Uuid::default(),
                "Properties" => compound! {
                    "textures" => List::Compound(vec![
                        compound! {
                            "Value" => "eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvYWMxNDYwMGFjZTUwNjk1YzdjOWJjZjA5ZTQyYWZkOWY1M2M5ZTIwZGFhMTUyNGM5NWRiNDE5N2RkMzExNjQxMiJ9fX0="
                        }
                    ])
                },
            }
        }),
    );

    let equipment = Equipment {
        helmet: Some(head),
        chestplate: None,
        leggings: None,
        boots: None,
    };

    let mut entity_flags = Flags::default();
    entity_flags.set_invisible(true);

    commands.spawn((
        ArmorStandEntityBundle {
            layer: EntityLayerId(layer),
            position: Position::new([0.0, 63.5, 0.0]),
            entity_flags,
            ..Default::default()
        },
        equipment,
    ));
}
