use crate::{AtRoom, Exits, MasmorraError, WantsToMove};
use legion::world::SubWorld;
use legion::{system, systems::CommandBuffer, Entity, EntityStore};

#[system(for_each)]
#[read_component(Exits)]
#[write_component(AtRoom)]
#[read_component(Entity)]
pub fn movement(
    cmd: &mut CommandBuffer,
    world: &mut SubWorld,
    ent: &Entity,
    wanna_move: &WantsToMove,
) {
    match find_destination(world, wanna_move) {
        Ok(to) => {
            let _ = try_move(world, wanna_move, to);
        }
        Err(err) => {
            dbg!(err);
        }
    }
    cmd.remove(*ent);
}

fn find_destination(
    world: &mut SubWorld,
    wanna_move: &WantsToMove,
) -> Result<Entity, MasmorraError> {
    let mut who = world.entry_mut(wanna_move.who)?;
    let at = who.get_component_mut::<AtRoom>()?;
    let room = at.room;
    let room = world.entry_ref(room)?;
    let exits = room.get_component::<Exits>()?;
    exits
        .exits
        .get(&wanna_move.to)
        .ok_or(MasmorraError::EntityErr("exit not found".to_string()))
        .and_then(|r| Ok(*r))
}

fn try_move(
    world: &mut SubWorld,
    wanna_move: &WantsToMove,
    to: Entity,
) -> Result<(), MasmorraError> {
    let mut who = world.entry_mut(wanna_move.who)?;
    let mut at = who.get_component_mut::<AtRoom>()?;
    at.room = to;
    Ok(())
}
