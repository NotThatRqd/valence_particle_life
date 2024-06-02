//! This module contains systems and whatnot for that are necessary
//! for doing this in Minecraft and aren't particularly interesting.

use valence::app::Plugin;
use valence::brand::SetBrand;
use valence::command::CommandExecutionEvent;
use valence::entity::living::LivingEntity;
use valence::entity::EntityId;
use valence::prelude::*;
use valence::protocol::packets::play::entity_equipment_update_s2c::EquipmentEntry;
use valence::protocol::packets::play::EntityEquipmentUpdateS2c;
use valence::protocol::WritePacket;

pub struct MinecraftBoilerplatePlugin;

impl Plugin for MinecraftBoilerplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (init_clients, despawn_disconnected_clients, command),
        );
    }
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    biomes: Res<BiomeRegistry>,
    dimensions: Res<DimensionTypeRegistry>,
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    // We have to add chunks to the world first, they start empty.
    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -80..80 {
        for x in -80..80 {
            // This actually sets the block in the world.
            layer.chunk.set_block([x, 64, z], BlockState::GRASS_BLOCK);
        }
    }

    // This spawns the layer into the world.
    commands.spawn(layer);
}

fn init_clients(
    mut clients_query: Query<
        (
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
            &mut Client,
        ),
        Added<Client>,
    >,
    layers_query: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for (
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
        mut client,
    ) in &mut clients_query
    {
        let layer = layers_query.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.5, 100.0, 0.5]);
        *game_mode = GameMode::Creative;

        client.set_brand("§cParticle Life§r");
    }
}

#[derive(Component, Default)]
pub struct Equipment {
    pub helmet: Option<ItemStack>,
    pub chestplate: Option<ItemStack>,
    pub leggings: Option<ItemStack>,
    pub boots: Option<ItemStack>,
}

fn command(
    mut command_events: EventReader<CommandExecutionEvent>,
    mut clients_query: Query<(&mut Client, &Username)>,
    entities_query: Query<(&EntityId, &Equipment), With<LivingEntity>>,
) {
    for event in command_events.read() {
        if event.command != "update_armor" {
            continue;
        }

        let (mut client, username) = clients_query.get_mut(event.executor).unwrap();

        println!("{username} used /update_armor");

        for (entity_id, equipment_component) in &entities_query {
            let mut equipment_list = Vec::new();

            if let Some(helmet) = &equipment_component.helmet {
                equipment_list.push(EquipmentEntry {
                    slot: 5,
                    item: helmet.clone(), // TODO: don't use clone
                });
            }

            if let Some(chestplate) = &equipment_component.chestplate {
                equipment_list.push(EquipmentEntry {
                    slot: 4,
                    item: chestplate.clone(), // TODO: don't use clone
                });
            }

            if let Some(leggings) = &equipment_component.leggings {
                equipment_list.push(EquipmentEntry {
                    slot: 3,
                    item: leggings.clone(), // TODO: don't use clone
                });
            }

            if let Some(boots) = &equipment_component.boots {
                equipment_list.push(EquipmentEntry {
                    slot: 2,
                    item: boots.clone(), // TODO: don't use clone
                });
            }

            if equipment_list.is_empty() {
                continue;
            }

            let equip_p = EntityEquipmentUpdateS2c {
                entity_id: entity_id.get().into(),
                equipment: equipment_list,
            };

            client.write_packet(&equip_p);
        }
    }
}
