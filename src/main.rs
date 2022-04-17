use legion::*;
use masmorra::{make_simple_zone, AtRoom, Description, Exit, Exits, Player};

fn stub_loop(world: &mut World) {
    let mut query = <(&Player, &mut AtRoom)>::query();
    let (mut qworld, mut sub_world) = world.split_for_query(&query);

    for chunk in query.iter_chunks_mut(&mut qworld) {
        for (_player, AtRoom { room: at_room }) in chunk {
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

            match exits.exits.get(&Exit { name: response }) {
                None => {
                    println!("Não tem saída pra esse lado!\n");
                }
                Some(other_room) => {
                    *at_room = *other_room;
                }
            }
        }
    }

    stub_loop(world)
}

fn main() {
    println!("Alô, Mundo!");

    let mut world = World::default();

    let initial_room = make_simple_zone(&mut world);

    world.push((Player, AtRoom { room: initial_room }));

    stub_loop(&mut world);
}
