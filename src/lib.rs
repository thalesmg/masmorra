use legion::world::ComponentError;
use legion::{Entity, World};
use std::collections::HashMap;
use thiserror::Error;

pub struct Player;

pub struct AtRoom {
    pub room: Entity,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Exit {
    pub name: String,
}

pub struct Exits {
    pub exits: HashMap<Exit, Entity>,
}

pub struct Description {
    pub short: String,
    pub long: String,
}

#[derive(Error, Debug)]
pub enum MasmorraError {
    #[error("component {0} is not available")]
    Component(#[from] ComponentError),
    #[error("Entity {0} not found")]
    EntityErr(String),
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
