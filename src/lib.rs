use std::fmt;
/// This stucture store as constant all possible value that a SAM read flag can take
/// to access SamFlag::<value>
/// example: SamFlag::PAIRED
/// for better readability we use this structure instead of passing value directly,
/// code will be more verbose but will document itself and logic error will be easier
/// to avoid/catch.
/// We don't pay runtime cost as constant expression are computed at compile time.
#[non_exhaustive]
pub struct SamFlag;

impl SamFlag {
    pub const PAIRED: u16 = 1;
    pub const PROPERLY_PAIRED: u16 = 2;
    pub const READ_UNMAPPED: u16 = 4;
    pub const MATE_UNMAPPED: u16 = 8;
    pub const READ_RERVERSE: u16 = 16;
    pub const MATE_REVERSE: u16 = 32;
    pub const FIRST_IN_PAIR: u16 = 64;
    pub const SECOND_IN_PAIR: u16 = 128;
    pub const NOT_PRIMARY_ALN: u16 = 256;
    pub const FAIL_QC: u16 = 512;
    pub const DUPLICATE: u16 = 1024;
    pub const SUPPLEMRNTARY: u16 = 2048;
}

#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub enum Strand {
    Plus,
    Minus,
}

impl fmt::Display for Strand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Strand::Plus => {
                write!(f, "+")
            }
            Strand::Minus => {
                write!(f, "-")
            }
        }
    }
}

impl From<&str> for Strand {
    fn from(item: &str) -> Self {
        match item {
            "+" => Strand::Plus,
            "-" => Strand::Minus,
            "." => Strand::Plus, //FIXME
            _ => {
                println!("{}", item);
                unreachable!();
            }
        }
    }
}

pub fn check_flag(flag: u16, in_: u16, not_in: u16) -> bool {
    //binary flag check
    //assert that: - in_ is in n
    //             - not_in is not in n
    // bitwise operation
    if (not_in & flag) != 0 {
        return false;
    }
    if (in_ & flag) != in_ {
        return false;
    }
    return true;
}

//
// Does not match proper Camel case on purpose
// as to avoid confusion with the first term(f, r, ff, fr)
#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LibType {
    frFirstStrand,
    frSecondStrand,
    fFirstStrand,
    fSecondStrand,
    ffFirstStrand,
    ffSecondStrand,
    rfFirstStrand,
    rfSecondStrand,
    rFirstStrand,
    rSecondStrand,
}

// with branch prediction this should be fast.
impl LibType {
    pub fn get_strand(self: &Self, flag: u16) -> Option<Strand> {
        match self {
            LibType::frFirstStrand => {
                if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::READ_RERVERSE,
                    SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_RERVERSE,
                ) {
                    Some(Strand::Plus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_RERVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::READ_RERVERSE,
                    SamFlag::MATE_REVERSE,
                ) {
                    Some(Strand::Minus)
                } else {
                    None
                }
            }
            LibType::frSecondStrand => {
                if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::READ_RERVERSE,
                    SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_RERVERSE,
                ) {
                    Some(Strand::Minus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_RERVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::READ_RERVERSE,
                    SamFlag::MATE_REVERSE,
                ) {
                    Some(Strand::Plus)
                } else {
                    None
                }
            }
            LibType::fFirstStrand => {
                if check_flag(flag, SamFlag::FIRST_IN_PAIR + SamFlag::READ_RERVERSE, 0) {
                    Some(Strand::Plus)
                } else if check_flag(flag, SamFlag::FIRST_IN_PAIR, SamFlag::READ_RERVERSE) {
                    Some(Strand::Minus)
                } else {
                    None
                }
            }
            LibType::fSecondStrand => {
                if check_flag(flag, SamFlag::FIRST_IN_PAIR + SamFlag::READ_RERVERSE, 0) {
                    Some(Strand::Minus)
                } else if check_flag(flag, SamFlag::FIRST_IN_PAIR, SamFlag::READ_RERVERSE) {
                    Some(Strand::Plus)
                } else {
                    None
                }
            }
            LibType::ffFirstStrand => {
                if check_flag(
                    flag,
                    SamFlag::PAIRED
                        + SamFlag::FIRST_IN_PAIR
                        + SamFlag::READ_RERVERSE
                        + SamFlag::MATE_REVERSE,
                    0,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED
                        + SamFlag::SECOND_IN_PAIR
                        + SamFlag::READ_RERVERSE
                        + SamFlag::MATE_REVERSE,
                    0,
                ) {
                    Some(Strand::Plus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR,
                    SamFlag::READ_RERVERSE + SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR,
                    SamFlag::READ_RERVERSE + SamFlag::MATE_REVERSE,
                ) {
                    Some(Strand::Minus)
                } else {
                    None
                }
            }
            LibType::ffSecondStrand => {
                if check_flag(
                    flag,
                    SamFlag::PAIRED
                        + SamFlag::FIRST_IN_PAIR
                        + SamFlag::READ_RERVERSE
                        + SamFlag::MATE_REVERSE,
                    0,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED
                        + SamFlag::SECOND_IN_PAIR
                        + SamFlag::READ_RERVERSE
                        + SamFlag::MATE_REVERSE,
                    0,
                ) {
                    Some(Strand::Minus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR,
                    SamFlag::READ_RERVERSE + SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR,
                    SamFlag::READ_RERVERSE + SamFlag::MATE_REVERSE,
                ) {
                    Some(Strand::Plus)
                } else {
                    None
                }
            }
            LibType::rfFirstStrand => {
                if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_RERVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::READ_RERVERSE,
                    SamFlag::MATE_REVERSE,
                ) {
                    Some(Strand::Plus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::READ_RERVERSE,
                    SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_RERVERSE,
                ) {
                    Some(Strand::Minus)
                } else {
                    None
                }
            }
            LibType::rfSecondStrand => {
                if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_RERVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::READ_RERVERSE,
                    SamFlag::MATE_REVERSE,
                ) {
                    Some(Strand::Minus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::READ_RERVERSE,
                    SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_RERVERSE,
                ) {
                    Some(Strand::Plus)
                } else {
                    None
                }
            }
            LibType::rFirstStrand => {
                if check_flag(flag, SamFlag::FIRST_IN_PAIR + SamFlag::READ_RERVERSE, 0) {
                    Some(Strand::Plus)
                } else if check_flag(flag, SamFlag::FIRST_IN_PAIR + SamFlag::READ_RERVERSE, 0) {
                    Some(Strand::Minus)
                } else {
                    None
                }
            }
            LibType::rSecondStrand => {
                if check_flag(flag, SamFlag::FIRST_IN_PAIR + SamFlag::READ_RERVERSE, 0) {
                    Some(Strand::Minus)
                } else if check_flag(flag, SamFlag::FIRST_IN_PAIR + SamFlag::READ_RERVERSE, 0) {
                    Some(Strand::Plus)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO
    #[test]
    fn it_works() {
        let flag = 147;
        assert_eq!(Some(Strand::Minus), LibType::frFirstStrand.get_strand(flag))
    }

    #[test]
    fn it_works2() {
        let flag = 99;
        assert_eq!(Some(Strand::Minus), LibType::frFirstStrand.get_strand(flag))
    }
}