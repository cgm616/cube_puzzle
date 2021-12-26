use core::fmt;

/// One puzzle piece made up of five cubes.
///
/// Each cube has a color, and no two cubes can have the same location. The
/// location is an index into a 3-dimensional space with height two, length two, and
/// width two.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Polymino {
    pub cubes: [(Location, Color); 5],
    pub grain: Axis,
}

impl Polymino {
    pub fn orient(&self, orientation: Orientation) -> Result<Polymino, &'static str> {
        if let (Axis::Short, FullRotation::Ninety | FullRotation::TwoSeventy) =
            (self.grain, orientation.short)
        {
            return Err("impossible orientation for this piece");
        }

        let mut new_poly = self.clone();

        fn rotate_grain_90(cubes: &mut [(Location, Color); 5]) {
            for cube in cubes.iter_mut() {
                let loc = cube.0;
                let new_loc;
                if loc.1 == 0 && loc.2 == 0 {
                    new_loc = (loc.0, 1, 0);
                } else if loc.1 == 1 && loc.2 == 0 {
                    new_loc = (loc.0, 1, 1);
                } else if loc.1 == 1 && loc.2 == 1 {
                    new_loc = (loc.0, 0, 1);
                } else {
                    new_loc = (loc.0, 0, 0);
                }
                cube.0 = new_loc;
            }
        }

        fn rotate_grain_180(cubes: &[(Location, Color); 5]) -> [(Location, Color); 5] {
            let mut ret = [((0, 0, 0), White); 5];
            cubes
                .iter()
                .map(|&((x, y, z), color)| ((x, 1 - y, 1 - z), color))
                .enumerate()
                .for_each(|(i, c)| ret[i] = c);
            ret
        }

        match orientation.short {
            FullRotation::Zero => {}
            FullRotation::Ninety => rotate_grain_90(&mut new_poly.cubes),
            FullRotation::OneEighty => {
                rotate_grain_90(&mut new_poly.cubes);
                rotate_grain_90(&mut new_poly.cubes);
                //rotate_grain_180(self.cubes)
            }
            FullRotation::TwoSeventy => {
                rotate_grain_90(&mut new_poly.cubes);
                rotate_grain_90(&mut new_poly.cubes);
                rotate_grain_90(&mut new_poly.cubes);
            }
        }

        if orientation.long == HalfRotation::OneEighty {
            for cube in new_poly.cubes.iter_mut() {
                cube.0 = (2 - cube.0 .0, 1 - cube.0 .1, cube.0 .2);
            }
        }

        Ok(new_poly)
    }

    pub fn normalize_grain(&mut self) {
        if self.grain == Axis::Short && !self.is_normalized() {
            for cube in self.cubes.iter_mut() {
                cube.0 = (1 - cube.0 .1, cube.0 .0, cube.0 .2);
            }
        }
    }

    pub fn undo_normalize(&mut self) {
        if self.grain == Axis::Short && self.is_normalized() {
            // the cubes are already sideways! so we need to reverse before we rotate
            for cube in self.cubes.iter_mut() {
                cube.0 = (cube.0 .1, 1 - cube.0 .0, cube.0 .2);
            }
        }
    }

    pub fn is_normalized(&self) -> bool {
        self.cubes.iter().any(|&((_, y, _), _)| y > 1)
    }
}

impl fmt::Display for Polymino {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut indices: [bool; 5] = [true; 5];
        writeln!(f, "bottom:   top:   grain: {}", self.grain)?;
        for y in 0..2 {
            for z in 0..2 {
                for x in 0..3 {
                    let mut found = false;
                    for i in 0..5 {
                        if indices[i] {
                            if self.cubes[i].0 == (x, y, z) {
                                write!(f, "{}", self.cubes[i].1)?;
                                indices[i] = false;
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        write!(f, "_")?;
                    }
                }
                write!(f, "       ")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PolyIndex(usize);

impl PolyIndex {
    pub fn new(i: usize) -> Result<PolyIndex, &'static str> {
        if i < 10 {
            Ok(PolyIndex(i))
        } else {
            Err("invalid index")
        }
    }

    pub fn to_poly(&self) -> &Polymino {
        &POLYMINOS[self.0]
    }
}

impl TryFrom<usize> for PolyIndex {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        PolyIndex::new(value)
    }
}

impl Into<usize> for PolyIndex {
    fn into(self) -> usize {
        self.0
    }
}

/// An index into 3-dimensional space.
///
/// X first, then Y, then Z.
pub type Location = (usize, usize, usize);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Black => write!(f, "B"),
            Color::White => write!(f, "W"),
        }
    }
}

/// A grain axis for a polymino.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Axis {
    Long,
    Short,
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Long => write!(f, "long"),
            Self::Short => write!(f, "short"),
        }
    }
}

/// A specific rotation of a polymino.
///
/// NOT ALL ORIENTATIONS ARE VALID FOR ALL POLYMINOS. To compute, we always
/// apply the short-direction orientation first.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Orientation {
    short: FullRotation,
    long: HalfRotation,
}

impl Orientation {
    pub fn new() -> Self {
        Orientation {
            short: FullRotation::Zero,
            long: HalfRotation::Zero,
        }
    }

    pub fn next(&self) -> Result<Self, ()> {
        let (short, long) = match (self.short, self.long) {
            (short, HalfRotation::Zero) => (short, HalfRotation::OneEighty),
            (FullRotation::Zero, HalfRotation::OneEighty) => {
                (FullRotation::Ninety, HalfRotation::Zero)
            }
            (FullRotation::Ninety, HalfRotation::OneEighty) => {
                (FullRotation::OneEighty, HalfRotation::Zero)
            }
            (FullRotation::OneEighty, HalfRotation::OneEighty) => {
                (FullRotation::TwoSeventy, HalfRotation::Zero)
            }
            (FullRotation::TwoSeventy, HalfRotation::OneEighty) => return Err(()),
        };

        Ok(Orientation { short, long })
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Orientation {
                short: FullRotation::Zero,
                long: HalfRotation::Zero,
            } => write!(f, "not rotated"),
            Orientation {
                short: FullRotation::Zero,
                long,
            } => write!(f, "rotated {} about short axis", long),
            Orientation {
                long: HalfRotation::Zero,
                short,
            } => write!(f, "rotated {} about long axis", short),
            Orientation { long, short } => write!(
                f,
                "rotated {} about long axis then {} about short axis",
                short, long
            ),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FullRotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

impl fmt::Display for FullRotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "0°"),
            Self::Ninety => write!(f, "90°"),
            Self::OneEighty => write!(f, "180°"),
            Self::TwoSeventy => write!(f, "270°"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HalfRotation {
    Zero,
    OneEighty,
}

impl fmt::Display for HalfRotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "0°"),
            Self::OneEighty => write!(f, "180°"),
        }
    }
}

use self::{Axis::*, Color::*};

pub const ONE: Polymino = Polymino {
    cubes: [
        ((0, 0, 0), Black),
        ((1, 0, 0), White),
        ((1, 1, 0), Black),
        ((2, 1, 0), White),
        ((1, 0, 1), Black),
    ],
    grain: Long,
};

pub const TWO: Polymino = Polymino {
    cubes: [
        ((0, 1, 0), White),
        ((1, 1, 0), Black),
        ((2, 1, 0), White),
        ((2, 0, 1), White),
        ((2, 1, 1), Black),
    ],
    grain: Short,
};

pub const THREE: Polymino = Polymino {
    cubes: [
        ((0, 0, 0), White),
        ((0, 1, 0), Black),
        ((1, 1, 0), White),
        ((1, 1, 1), Black),
        ((2, 1, 1), White),
    ],
    grain: Short,
};

pub const FOUR: Polymino = Polymino {
    cubes: [
        ((0, 0, 0), Black),
        ((1, 0, 0), White),
        ((2, 0, 0), Black),
        ((0, 1, 0), White),
        ((1, 0, 1), Black),
    ],
    grain: Long,
};

pub const FIVE: Polymino = Polymino {
    cubes: [
        ((0, 0, 0), Black),
        ((1, 0, 0), White),
        ((1, 1, 0), Black),
        ((1, 0, 1), Black),
        ((2, 0, 1), White),
    ],
    grain: Long,
};

pub const SIX: Polymino = Polymino {
    cubes: [
        ((0, 0, 0), White),
        ((1, 0, 0), Black),
        ((2, 0, 0), White),
        ((0, 1, 0), Black),
        ((0, 0, 1), Black),
    ],
    grain: Short,
};

pub const SEVEN: Polymino = Polymino {
    cubes: [
        ((0, 0, 0), Black),
        ((0, 1, 0), White),
        ((1, 1, 0), Black),
        ((2, 1, 0), White),
        ((1, 1, 1), White),
    ],
    grain: Long,
};

pub const EIGHT: Polymino = Polymino {
    cubes: [
        ((0, 1, 0), White),
        ((1, 1, 0), Black),
        ((2, 1, 0), White),
        ((0, 0, 1), White),
        ((0, 1, 1), Black),
    ],
    grain: Short,
};

pub const NINE: Polymino = Polymino {
    cubes: [
        ((1, 0, 0), Black),
        ((2, 0, 0), White),
        ((0, 1, 0), Black),
        ((1, 1, 0), White),
        ((2, 0, 1), Black),
    ],
    grain: Short,
};

pub const TEN: Polymino = Polymino {
    cubes: [
        ((0, 0, 0), White),
        ((1, 0, 0), Black),
        ((2, 0, 0), White),
        ((1, 1, 0), White),
        ((1, 1, 1), Black),
    ],
    grain: Long,
};

pub const POLYMINOS: [Polymino; 10] = [ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_op_orient() {
        for orig in POLYMINOS {
            let new = orig.orient(Orientation::new()).unwrap();
            assert_eq!(orig, new);
        }
    }

    #[test]
    fn no_op_normalize() {
        for orig in POLYMINOS {
            let mut new = orig.clone();
            assert!(!new.is_normalized());

            new.normalize_grain();
            assert!(new.is_normalized() || new.grain == Axis::Long);

            new.undo_normalize();
            assert!(!new.is_normalized());

            assert_eq!(orig, new);
        }
    }

    #[test]
    fn short_three_sixty() {
        for orig in POLYMINOS {
            match orig.grain {
                Axis::Long => {
                    let one_eighty_turn = Orientation {
                        short: FullRotation::OneEighty,
                        long: HalfRotation::Zero,
                    };
                    let one_eighty = orig.orient(one_eighty_turn).unwrap();
                    assert_eq!(orig, one_eighty.orient(one_eighty_turn).unwrap());
                }
                Axis::Short => {
                    let one_eighty_turn = Orientation {
                        short: FullRotation::OneEighty,
                        long: HalfRotation::Zero,
                    };
                    let one_eighty = orig.orient(one_eighty_turn).unwrap();
                    let orig_oriented = orig.orient(Orientation::new()).unwrap();
                    let fully_turned = one_eighty.orient(one_eighty_turn).unwrap();
                    assert_eq!(orig_oriented, fully_turned);
                }
            }
        }
    }

    #[test]
    fn long_three_sixty() {
        for orig in POLYMINOS {
            println!("------ turning polymino ------");
            println!("{}", orig);
            let one_eighty_turn = Orientation {
                short: FullRotation::Zero,
                long: HalfRotation::OneEighty,
            };
            let mut new = orig.orient(one_eighty_turn).unwrap();
            println!("one turn:\n{}", new);
            new = new.orient(one_eighty_turn).unwrap();
            println!("second turn:\n{}", new);
            assert_eq!(orig, new);
        }
    }
}
