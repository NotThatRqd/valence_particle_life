use std::time::{Duration, Instant};

use valence::{
    entity::{armor_stand::ArmorStandEntityBundle, entity::Flags},
    nbt::{compound, List},
    prelude::*,
};

use crate::minecraft_boilerplate::Equipment;

pub struct ParticleLifePlugin;

impl Plugin for ParticleLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup)
            .add_systems(Update, update_particles)
            .init_resource::<AttractionMatrix>()
            .init_resource::<Time>();
    }
}

#[derive(Resource)]
struct Time {
    last_updated: Instant,
}

impl Time {
    fn update_and_get_delta(&mut self, new_time: Instant) -> Duration {
        let delta = new_time.duration_since(self.last_updated);
        self.last_updated = new_time;
        delta
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            last_updated: Instant::now(),
        }
    }
}

#[derive(Component)]
struct Particle {
    color: ParticleColor,
    velocity: DVec3,
}

#[derive(Clone, Copy)]
enum ParticleColor {
    Red = 0,
    Blue,
}

impl ParticleColor {
    fn into_skin_value(&self) -> String {
        match self {
            Self::Red => "eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvYWMxNDYwMGFjZTUwNjk1YzdjOWJjZjA5ZTQyYWZkOWY1M2M5ZTIwZGFhMTUyNGM5NWRiNDE5N2RkMzExNjQxMiJ9fX0=",
            Self::Blue => "eyJ0ZXh0dXJlcyI6eyJTS0lOIjp7InVybCI6Imh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvNjEwZTM3NGNkYzJiYTk1YmI3MmYxYTAzNmM3N2RhMzUwOTkzNWExYWJkMjRiNjhjNmIzNTkxNjkwYjEwM2ZlZCJ9fX0="
        }.to_string()
    }
}

// TODO: make more idiomatic
#[derive(Resource)]
pub(crate) struct AttractionMatrix(pub(crate) [[f64; 2]; 2]);

impl Default for AttractionMatrix {
    fn default() -> Self {
        AttractionMatrix([[0.0, 0.0], [0.0, 0.0]])
    }
}

fn setup(
    mut commands: Commands,
    layers_query: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    let layer = layers_query.single();

    for x in -5..=5 {
        for z in -5..=5 {
            let color = if x % 2 == 0 {
                ParticleColor::Red
            } else {
                ParticleColor::Blue
            };

            let head = ItemStack::new(
                ItemKind::PlayerHead,
                1,
                Some(compound! {
                    "SkullOwner" => compound! {
                        "Id" => Uuid::default(),
                        "Properties" => compound! {
                            "textures" => List::Compound(vec![
                                compound! {
                                    "Value" => color.into_skin_value()
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
                    position: Position::new([x as f64, 63.5, z as f64]),
                    entity_flags,
                    ..Default::default()
                },
                equipment,
                Particle {
                    color,
                    velocity: DVec3::ZERO,
                },
            ));
        }
    }
}

/// The maximum number of blocks away a particle can be from another to feel a force
const R_MAX: f64 = 5.0;
const FRICTION_HALF_LIFE: f64 = 0.040;
const FORCE_FACTOR: f64 = 10.0;

fn update_particles(
    mut particles_query: Query<(&mut Particle, &mut Position)>,
    matrix: Res<AttractionMatrix>,
    mut time: ResMut<Time>,
) {
    let delta_time_in_seconds = time.update_and_get_delta(Instant::now()).as_secs_f64();

    let mut particles: Vec<(Mut<Particle>, Mut<Position>)> = particles_query.iter_mut().collect();
    for i in 0..particles.len() {
        let mut total_force = DVec3::ZERO;

        for j in 0..particles.len() {
            if j == i {
                continue;
            }

            // Distance between the two points
            let r = particles[i].1.distance(particles[j].1 .0);

            if r > 0.0 && r < R_MAX {
                let f: f64 = force(
                    r / R_MAX,
                    matrix.0[particles[i].0.color as usize][particles[j].0.color as usize],
                );
                // Scale the unit vector pointing in j's direction from i by a factor of f
                total_force += (particles[j].1 .0 - particles[i].1 .0).normalize() * f;
            }
        }

        total_force *= R_MAX;

        total_force *= FORCE_FACTOR;

        let friction_factor = (0.5_f64).powf(delta_time_in_seconds / FRICTION_HALF_LIFE);
        particles[i].0.velocity *= friction_factor;

        particles[i].0.velocity += total_force * delta_time_in_seconds;
    }

    // Update positions
    for (particle, mut position) in particles_query.iter_mut() {
        position.0 += particle.velocity * delta_time_in_seconds;
    }
}

fn force(r: f64, a: f64) -> f64 {
    let beta = 0.3;
    if r < beta {
        // Universal repulsion force
        r / beta - 1.0
    } else if beta < r && r < 1.0 {
        a * (1.0 - (2.0 * r - 1.0 - beta).abs() / (1.0 - beta))
    } else {
        0.0
    }
}
