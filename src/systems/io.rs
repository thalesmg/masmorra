use crate::{AtRoom, Description, Exit, Exits, MasmorraError, WantsToLook};
use crossbeam_channel::Sender;
use legion::systems::CommandBuffer;
use legion::world::SubWorld;
use legion::{system, Entity, EntityStore};

#[system(for_each)]
#[read_component(AtRoom)]
#[read_component(Description)]
#[read_component(Exits)]
pub fn look(
    cmd: &mut CommandBuffer,
    world: &SubWorld,
    ent: &Entity,
    WantsToLook { who }: &WantsToLook,
    ret: &Sender<String>,
) {
    match try_look(world, *who, ret) {
        Ok(()) => (),
        Err(err) => {
            dbg!(err);
            ()
        }
    }
    cmd.remove(*ent);
}

fn try_look(world: &SubWorld, who: Entity, ret: &Sender<String>) -> Result<(), MasmorraError> {
    let who = world.entry_ref(who)?;
    let AtRoom { room } = who.get_component::<AtRoom>()?;
    let room = world.entry_ref(*room)?;
    let desc = room.get_component::<Description>()?;
    let exits = room.get_component::<Exits>()?;
    let exit_names = exits
        .exits
        .keys()
        .map(|Exit { name }| name.clone())
        .collect::<Vec<_>>()
        .join(", ");
    let msg = format!("{}\n\n{}\nExits: {}\n", desc.short, desc.long, exit_names);
    // todo: handle error
    ret.send(msg).unwrap();
    Ok(())
}
