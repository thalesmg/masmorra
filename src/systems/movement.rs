use crate::{AtRoom, Exits, MasmorraError, WantsToMove};
use legion::world::SubWorld;
use legion::{system, systems::CommandBuffer, Entity, EntityStore};

#[system(for_each)]
#[read_component(Exits)]
pub fn movement(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    who: &Entity,
    wanna_move: &WantsToMove,
    at: &mut AtRoom,
) {
    let _ = try_move(world, at, wanna_move);
    cmd.remove_component::<WantsToMove>(*who);
}

fn try_move(
    world: &SubWorld,
    at: &mut AtRoom,
    wanna_move: &WantsToMove,
) -> Result<(), MasmorraError> {
    let room = world.entry_ref(at.room)?;
    let exits = room.get_component::<Exits>()?;
    if let Some(to) = exits.exits.get(&wanna_move.to) {
        at.room = *to;
    }
    Ok(())
}
