pub mod board;
pub mod poly;

use board::{Board, Move};
use poly::{Location, Orientation};

fn main() {
    let moves = search();
    let mut board = Board::new();
    for move_ in moves {
        let poly = move_.polymino;
        let orientation = move_.orientation;
        let location = move_.location;
        board.push(poly, orientation, location).unwrap();
        println!("{}", board);
    }
}

fn search() -> Vec<Move> {
    let mut board = Board::new();

    let mut poly_index = 0;
    let mut orientation = Orientation::new(); // TODO: figure this out
    let mut location = (0, 0, 0);
    while !board.complete() {
        // println!(
        //     "trying polymino {} at {:?} in orientation {:?}",
        //     poly_index, location, orientation
        // );
        if poly_index == 0 && location == (2, 3, 0) {
            println!(
                "trying polymino {} at {:?} in orientation {:?}",
                poly_index, location, orientation
            );
            print!("{}", board);
        }
        match board.push(
            poly_index.try_into().expect("should never go past 10"),
            orientation,
            location,
        ) {
            Ok(_) => {
                println!(
                    "placing polymino {} at {:?} in orientation {:?}",
                    poly_index, location, orientation
                );
                poly_index += 1;
                orientation = Orientation::new(); // reset orientation
                location = (0, 0, 0);
            }
            Err(_) => {
                match next_orient_or_loc(orientation, location) {
                    Ok((new_orient, new_loc)) => {
                        orientation = new_orient;
                        location = new_loc;
                        continue;
                    }
                    Err(()) => {
                        println!(
                            "removing polymino {}",
                            poly_index.checked_sub(1).expect("Should never wrap around")
                        );
                        // if last location and orientation, then it is impossible to place this poly anywhere on board! so the current board state is invalid
                        // so pop last poly and try a new place/orientation for it
                        let mut last_move =
                            board.pop().expect("Should be able to find a solution!");
                        poly_index -= 1;
                        while let Err(()) =
                            next_orient_or_loc(last_move.orientation, last_move.location)
                        {
                            last_move =
                                board.pop().expect("Should be able to find a solution! (2)");
                            poly_index -= 1;
                        }

                        let (new_orient, new_loc) =
                            next_orient_or_loc(last_move.orientation, last_move.location).unwrap();
                        orientation = new_orient;
                        location = new_loc;
                    }
                }
            }
        }
    }

    board.moves
}

fn next_orient_or_loc(
    orientation: Orientation,
    location: Location,
) -> Result<(Orientation, Location), ()> {
    match orientation.next() {
        Ok(new) => Ok((new, location)),
        Err(()) => {
            // no more orientations!
            match location {
                (3, 3, _) => {
                    // exhausted all locations! (because every polymino takes up at least two by two by two)
                    Err(())
                }
                (4, y, _) => Ok((Orientation::new(), (0, y + 1, 0))),
                (x, y, _) => Ok((Orientation::new(), (x + 1, y, 0))),
            }
        }
    }
}
