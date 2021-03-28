<img src="repo/splash.png" alt="Planck ECS" />
<a href="https://crates.io/crates/planck_ecs">
    <img src="https://img.shields.io/crates/v/planck_ecs.svg" alt="Planck ECS" />
</a>

The full ECS (Entity-Component-System) library.

Support an Open Source Developer! :hearts:  
[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)


Composed of two smaller libraries:
* [world_dispatcher](https://github.com/jojolepro/world_dispatcher): the `System` part of the ECS.
* [entity_component](https://github.com/jojolepro/entity_component): the `Entity-Component` part of the ECS.

Read the [documentation](https://docs.rs/planck_ecs).

# Why would you use this ECS library?

* Compatible with all platforms, including WASM!
* Fast enough on *every* operation, not just iteration.
* Minimal amount of dependencies.
* Small code size.
* Stable, tested, benchmarked, 100% completed.
* Ability to handle system errors instead of crashing.
* Convert both functions and closures into systems!
* Safe: only 3 `unsafe` in total. (compared to hundreds in mainstream ECS libraries!)

# Usage
Add the following to you Cargo.toml file:
```
planck_ecs = "*"
```

Use it like so:
```rust
use planck_ecs::*;
fn main() {
    #[derive(Default)]
    pub struct A;

    let mut world = World::default();

    let sys = (|comps: &mut Components<A>, entities: &mut Entities| {
        let entity = entities.create();
        comps.insert(entity, A);
        Ok(())
    }).system();

    let mut dispatch = DispatcherBuilder::new().add_system(sys).build(&mut world);
    dispatch.run_seq(&world).unwrap();
    dispatch.run_seq(&world).unwrap();
    dispatch.run_seq(&world).unwrap();

    assert!(world.get::<Components<A>>().is_ok());
}
```

For more examples, see the two following repositories' example folders and documentation:
* [world_dispatcher](https://github.com/jojolepro/world_dispatcher): information on systems, world and dispatchers.
* [entity_component](https://github.com/jojolepro/entity_component): information on entities, components and joins.

### Maintainer Information

* Maintainer: Jojolepro
* Contact: jojolepro [at] jojolepro [dot] com
* Website: [jojolepro.com](https://jojolepro.com)
* Patreon: [patreon](https://patreon.com/jojolepro)

