use crossbeam_channel::{unbounded, Receiver, Sender};
use legion::*;
use masmorra::systems::io::look_system;
use masmorra::systems::movement::movement_system;
use masmorra::{make_simple_zone, AtRoom, Exit, Message, Player, WantsToLook, WantsToMove};
use std::thread;

fn world_loop(rx: Receiver<(Sender<String>, Message)>, mut world: World) {
    use Message::*;
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(movement_system())
        .flush()
        .add_system(look_system())
        .build();
    loop {
        match rx.recv().unwrap() {
            (ret, Look { who }) => {
                world.push((WantsToLook { who }, ret));
            }
            (ret, Move { who, to }) => {
                world.push((WantsToMove { who, to }, ret));
            }
        }
        schedule.execute(&mut world, &mut resources);
    }
}

fn print_loop(io_rx: Receiver<String>) {
    use std::io;
    use std::io::prelude::*;
    loop {
        let msg = io_rx.recv().unwrap();
        println!("\n{}", msg);
        print!("> ");
        io::stdout().flush().expect("couldn't flush stdout");
    }
}

fn main() {
    println!("Alô, Mundo!");

    let mut world = World::default();

    let initial_room = make_simple_zone(&mut world);

    let player_ent = world.push((Player, AtRoom { room: initial_room }));

    let (world_tx, world_rx) = unbounded();

    thread::spawn(move || {
        world_loop(world_rx, world);
    });

    let (io_tx, io_rx) = unbounded();
    thread::spawn(move || {
        print_loop(io_rx);
    });
    // initial look
    world_tx
        .send((io_tx.clone(), Message::Look { who: player_ent }))
        .unwrap();

    use std::io;
    let mut response = String::new();
    loop {
        io::stdin()
            .read_line(&mut response)
            .expect("couldn't read from stdin!");
        response = response.trim().to_string();
        match response.as_str() {
            "" => continue,
            "quit" => break,
            "look" => world_tx
                .send((io_tx.clone(), Message::Look { who: player_ent }))
                .unwrap(),
            exit => world_tx
                .send((
                    io_tx.clone(),
                    Message::Move {
                        who: player_ent,
                        to: Exit {
                            name: exit.to_string(),
                        },
                    },
                ))
                .unwrap(),
        };
        response.clear();
    }
}
