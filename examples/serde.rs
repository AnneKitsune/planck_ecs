use planck_ecs::*;
use serde::Serialize;

struct Animal;

#[derive(Clone, Serialize)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Serialize)]
struct SerializeMe {
    t: &'static str,
    positions: Vec<Position>,
}

fn main() {
    let mut world = World::default();

    let create_animals = (|animals: &mut Components<Animal>,
                           positions: &mut Components<Position>,
                           entities: &mut Entities| {
        for i in 0..10 {
            let entity = entities.create();
            animals.insert(entity, Animal);
            positions.insert(
                entity,
                Position {
                    x: i as f32,
                    y: i as f32,
                },
            );
        }
        Ok(())
    })
    .system();

    let mut setup = DispatcherBuilder::new()
        .add_system(create_animals)
        .build(&mut world);
    setup.run_seq(&world).unwrap();

    let animals = world.get::<Components<Animal>>().unwrap();
    let positions = world.get::<Components<Position>>().unwrap();
    let cloned_positions = join!(&animals && &positions)
        .filter_map(|(_, position)| position)
        .cloned()
        .collect::<Vec<_>>();

    let serialize_me = SerializeMe {
        t: "Animals",
        positions: cloned_positions,
    };
    let serialized = serde_json::to_string_pretty(&serialize_me).unwrap();
    println!("serialized = {}", serialized);
}
