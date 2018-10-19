extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io;

#[derive(Serialize, Deserialize)]
enum Exit {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
}

#[derive(Serialize, Deserialize)]
struct Room {
    description: String,
    exits: Vec<Exit>,
}

fn main() {
    let world = std::fs::read_to_string("rooms.json").unwrap();
    let world: Vec<Room> = serde_json::from_str(&world).unwrap();

    let mut current_room = 0;

    loop {
        // print the world
        println!("{}", world[current_room].description);

        println!("You can exit in these directions:");
        world[current_room].exits.iter().for_each(|exit| {
            match exit {
                Exit::North(_) => println!("North"),
                Exit::South(_) => println!("South"),
                Exit::East(_) => println!("East"),
                Exit::West(_) => println!("West"),
            }
        });

        // get some input
        println!("What do you want to do?");
        let mut buffer = String::new();

        io::stdin()
            .read_line(&mut buffer)
            .unwrap();

        let buffer = buffer.to_lowercase();

        // do some stuff
        match buffer.trim() {
            "quit" => break,
            "east" => {
                for exit in &world[current_room].exits {
                    match exit {
                        Exit::East(number) => {
                            current_room = *number;
                        },
                        _ => (),
                    }
                };
            },
            "west" => {
                for exit in &world[current_room].exits {
                    match exit {
                        Exit::West(number) => {
                            current_room = *number;
                        },
                        _ => (),
                    }
                };
            },
            "north" => {
                for exit in &world[current_room].exits {
                    match exit {
                        Exit::North(number) => {
                            current_room = *number;
                        },
                        _ => (),
                    }
                };
            },
            "south" => {
                for exit in &world[current_room].exits {
                    match exit {
                        Exit::South(number) => {
                            current_room = *number;
                        },
                        _ => (),
                    }
                };
            },
            _ => {
                println!("I do not understand.");
            }
        }
    }
}