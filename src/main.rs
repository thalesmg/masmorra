use std::collections::HashMap;

use legion::*;
use masmorra::{link, AtRoom, Description, Exit, Exits, Player};

fn make_simple_zone(world: &mut World) -> Entity {
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
    println!("Hello, world!");

    let mut world = World::default();

    let initial_room = make_simple_zone(&mut world);

    world.push((Player, AtRoom { room: initial_room }));

    stub_loop(&mut world);
}
