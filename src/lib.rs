use legion::world::ComponentError;
use legion::world::EntityAccessError;
use legion::{Entity, World};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use thiserror::Error;

pub mod systems;

#[derive(Serialize, Deserialize)]
pub struct Player;

#[derive(Debug, Serialize, Deserialize)]
pub struct AtRoom {
    pub room: Entity,
}

#[derive(Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Exit {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Exits {
    pub exits: HashMap<Exit, Entity>,
}

impl Serialize for Exits {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.exits.len()))?;
        for (exit, room) in &self.exits {
            map.serialize_entry(&exit.name, &room)?;
        }
        map.end()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Description {
    pub short: String,
    pub long: String,
}

#[derive(Debug)]
pub struct WantsToMove {
    pub who: Entity,
    pub to: Exit,
}

pub struct WantsToLook {
    pub who: Entity,
}

#[derive(Debug)]
pub enum Message {
    Move { who: Entity, to: Exit },
    Look { who: Entity },
}

#[derive(Error, Debug)]
pub enum MasmorraError {
    #[error("component {0} is not available")]
    Component(#[from] ComponentError),
    #[error("Entity {0} not found")]
    EntityErr(String),
    #[error("Entity access error: {0}")]
    EntityAccessErr(#[from] EntityAccessError),
}

pub fn link(
    world: &mut World,
    room1: Entity,
    exit1: Exit,
    room2: Entity,
    exit2: Exit,
) -> Result<(), MasmorraError> {
    let mut entry1 = world
        .entry(room1)
        .ok_or_else(|| MasmorraError::EntityErr("room1".to_string()))?;
    let ex1 = entry1.get_component_mut::<Exits>()?;
    ex1.exits.insert(exit1, room2);

    let mut entry2 = world
        .entry(room2)
        .ok_or_else(|| MasmorraError::EntityErr("room2".to_string()))?;
    let ex2 = entry2.get_component_mut::<Exits>()?;
    ex2.exits.insert(exit2, room1);

    Ok(())
}

pub fn make_simple_zone(world: &mut World) -> Entity {
    let t1 = world.push((
        Exits {
            exits: HashMap::new(),
        },
        Description {
            short: "Túmulo de Rui Burgos".to_string(),
            long: concat!(
                "Aqui jaz o túmulo de Rui Burgos, ",
                "com uma lápide bem simples..."
            )
            .to_string(),
        },
    ));

    let t2 = world.push((
        Exits {
            exits: HashMap::new(),
        },
        Description {
            short: "Academia da terceira idade".to_string(),
            long: concat!(
                "Alguns aparalelhos para idosos, e talvez não para ",
                "tão idosos assim, estão distribuídos em um círculo ",
                "nesta clareira."
            )
            .to_string(),
        },
    ));

    let t3 = world.push((
        Exits {
            exits: HashMap::new(),
        },
        Description {
            short: "Um caminho na praça".to_string(),
            long: concat!(
                "Um caminho singelo, com chão de cimento levemente quebrado, ",
                "passa por entre os flamboyants desta praça.  Ao oeste, você ",
                "vê o que parece ser um conjunto de equipamentos amarelos, ",
                "enquanto ao noroeste parece haver uma espécie de lápide."
            )
            .to_string(),
        },
    ));

    link(
        world,
        t1,
        Exit {
            name: "south".to_string(),
        },
        t2,
        Exit {
            name: "north".to_string(),
        },
    )
    .expect("could not link!");
    link(
        world,
        t3,
        Exit {
            name: "northwest".to_string(),
        },
        t1,
        Exit {
            name: "southeast".to_string(),
        },
    )
    .expect("could not link!");
    link(
        world,
        t3,
        Exit {
            name: "west".to_string(),
        },
        t2,
        Exit {
            name: "east".to_string(),
        },
    )
    .expect("could not link!");

    t3
}
