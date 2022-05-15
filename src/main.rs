use legion::{systems::CommandBuffer, *};
use masmorra::systems::movement::movement_system;
use masmorra::{make_simple_zone, AtRoom, Description, Exit, Exits, Player, WantsToMove};

fn stub_loop(mut world: World, mut resources: Resources, mut schedule: Schedule) {
    loop {
        let mut cmd = CommandBuffer::new(&world);
        let mut query = <(Entity, &Player, &mut AtRoom)>::query();
        let (mut qworld, mut sub_world) = world.split_for_query(&query);

        query.for_each_mut(
            &mut qworld,
            |(player_ent, _player, AtRoom { room: at_room })| {
                let room = sub_world.entry_mut(*at_room).expect("no such room!");
                let desc = room
                    .get_component::<Description>()
                    .expect("room without description!");
                let exits = room.get_component::<Exits>().expect("room without exits!");
                let exit_names = exits
                    .exits
                    .keys()
                    .map(|Exit { name }| name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("{}\n\n{}\n", desc.short, desc.long);
                println!("Exits: {}\n", exit_names);
                use std::io;
                use std::io::prelude::*;
                print!("> ");
                io::stdout().flush().expect("couldn't flush stdout");
                let mut response = String::new();
                io::stdin()
                    .read_line(&mut response)
                    .expect("couldn't read from stdin!");
                println!();

                response = response.trim().to_string();
                if !response.is_empty() {
                    cmd.push((WantsToMove {
                        who: *player_ent,
                        to: Exit { name: response },
                    },));
                }
            },
        );

        cmd.flush(&mut world, &mut resources);
        schedule.execute(&mut world, &mut resources);
    }
}

fn main() {
    println!("Alô, Mundo!");

    let mut world = World::default();

    let initial_room = make_simple_zone(&mut world);

    world.push((Player, AtRoom { room: initial_room }));

    let resources = Resources::default();
    let schedule = Schedule::builder().add_system(movement_system()).build();

    stub_loop(world, resources, schedule);
}
