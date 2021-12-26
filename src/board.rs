use super::poly::{Color, Location, Orientation, PolyIndex, Polymino};
use core::fmt;

/// The board is the five by five by two space in which to place polyminos.
///
/// The struct contains a "stack" of moves that lead to the current state,
/// which is also stored (but can be recreated entirely from the stack if
/// necessary). The state is indexed by X then Y then Z (i.e. by visual column,
/// visual row, then physical column when looking down at the board).
#[derive(Clone, Debug)]
pub struct Board {
    pub moves: Vec<Move>,
    state: [[[Option<Color>; 2]; 5]; 5],
}

impl Board {
    pub fn new() -> Self {
        Board {
            moves: Vec::new(),
            state: [[[None; 2]; 5]; 5],
        }
    }

    pub fn complete(&self) -> bool {
        // TODO: probably a better way to do this
        for x in 0..5 {
            for y in 0..5 {
                for z in 0..2 {
                    if let Some(color) = self.state[x][y][z] {
                        if !Board::color_fits((x, y, z), color) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn push(
        &mut self,
        poly: PolyIndex,
        orientation: Orientation,
        location: Location,
    ) -> Result<(), &str> {
        let mut oriented = poly.to_poly().orient(orientation)?;
        oriented.normalize_grain();

        let possible = oriented.cubes.iter().all(|&(cube_loc, color)| {
            (location.0 + cube_loc.0 < 5)
                && (location.1 + cube_loc.1 < 5)
                && (location.2 + cube_loc.2 < 2)
                && self.state[location.0 + cube_loc.0][location.1 + cube_loc.1]
                    [location.2 + cube_loc.2]
                    .is_none()
                && Board::color_fits(
                    (
                        location.0 + cube_loc.0,
                        location.1 + cube_loc.1,
                        location.2 + cube_loc.2,
                    ),
                    color,
                )
        });

        // TODO: check for gaps that can't be filled

        if possible {
            for (cube_loc, color) in oriented.cubes {
                self.state[location.0 + cube_loc.0][location.1 + cube_loc.1]
                    [location.2 + cube_loc.2] = Some(color);
            }
            oriented.undo_normalize();
            let change = Move {
                location,
                orientation,
                polymino: poly,
            };
            self.moves.push(change);

            Ok(())
        } else {
            Err("not possible to place")
        }
    }

    pub fn pop(&mut self) -> Option<Move> {
        let last_move = self.moves.pop()?;
        let mut poly = last_move
            .polymino
            .to_poly()
            .orient(last_move.orientation)
            .unwrap();
        poly.normalize_grain();

        for (cube_loc, _) in poly.cubes {
            self.state[last_move.location.0 + cube_loc.0][last_move.location.1 + cube_loc.1]
                [last_move.location.2 + cube_loc.2] = None;
        }

        Some(last_move)
    }

    pub fn color_fits(location: Location, color: Color) -> bool {
        (location.0 + location.1 + location.2) % 2
            == match color {
                Color::Black => 0,
                Color::White => 1,
            }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "board state:    moves:")?;
        for y in 0..5 {
            for z in 0..2 {
                for x in 0..5 {
                    match self.state[x][y][z] {
                        Some(color) => write!(f, "{}", color)?,
                        None => write!(f, "_")?,
                    }
                }
                write!(f, "   ")?;
            }
            if y < self.moves.len() {
                writeln!(f, "   - {}", self.moves[y])?;
            } else {
                writeln!(f, "")?;
            }
        }
        if self.moves.len() > 5 {
            for move_ in self.moves.iter().skip(5) {
                writeln!(f, "                   - {}", move_)?;
            }
        }
        Ok(())
    }
}

/// A move is one placement of a polymino on the board.
///
/// The location of placement is always the cube occupied by the leftmost, lowest,
/// deepest cube (i.e. <0, 0, 0> on the polymino).
#[derive(Clone, Debug)]
pub struct Move {
    pub location: Location,
    pub orientation: Orientation,
    pub polymino: PolyIndex,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "polymino {} at {:?} {}",
            <PolyIndex as Into<usize>>::into(self.polymino),
            self.location,
            self.orientation
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::poly::{self, Orientation, POLYMINOS};

    #[test]
    fn push_and_pop() {
        for i in 0..POLYMINOS.len() {
            for j in 0..7 {
                let mut board = Board::new();

                let mut location = (1, 1, 0);
                let mut orientation = Orientation::new();
                for _ in 0..j {
                    orientation = orientation.next().unwrap();
                }
                if let Err(_) = board.push(i.try_into().unwrap(), orientation, location) {
                    location = (1, 2, 0);
                    board
                        .push(i.try_into().unwrap(), orientation, location)
                        .unwrap();
                }
                println!("{}", board);

                let last_move = board.pop().unwrap();
                assert_eq!(last_move.location, location);
                assert_eq!(last_move.orientation, orientation);
                let poly_index: usize = last_move.polymino.into();
                assert_eq!(poly_index, i);

                let not_valid = board.pop();
                assert!(not_valid.is_none());

                let any_cubes = board
                    .state
                    .iter()
                    .flat_map(|col| col.iter())
                    .flat_map(|col| col.iter())
                    .all(|x| x.is_none());
            }
        }
    }
}
