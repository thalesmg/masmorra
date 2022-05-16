use crate::{AtRoom, Exits, MasmorraError, WantsToLook, WantsToMove};
use crossbeam_channel::Sender;
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
    ret: Option<&Sender<String>>,
) {
    match find_destination(world, wanna_move) {
        Ok(to) => {
            let res = try_move(world, wanna_move, to);
            if let (Ok(()), Some(ret)) = (res, ret) {
                cmd.push((
                    WantsToLook {
                        who: wanna_move.who,
                    },
                    ret.clone(),
                ));
            }
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
        .ok_or_else(|| MasmorraError::EntityErr("exit not found".to_string()))
        .map(|r| *r)
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
