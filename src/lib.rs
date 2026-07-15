//! Core library for inferring RNA-seq read strand from SAM/BAM flags.
//!
//! [`SamFlag`] names the individual SAM flag bits, [`LibType`] represents the
//! supported stranded/unstranded library layouts (e.g. `frFirstStrand`,
//! `ffSecondStrand`), and [`LibType::get_strand`] uses a library type's flag
//! combination to classify a read's [`Strand`] (`Plus`, `Minus`, or `NA`).

use std::fmt;
use std::str::FromStr;

/// Stores, as associated constants, every value a SAM read flag bit can take,
/// accessed as `SamFlag::<value>` (e.g. `SamFlag::PAIRED`).
///
/// Using named constants instead of passing raw bit values around makes the
/// code self-documenting and easier to review for logic errors; since these
/// are constant expressions, there is no runtime cost.
#[non_exhaustive]
pub struct SamFlag;

impl SamFlag {
    pub const PAIRED: u16 = 1;
    pub const PROPERLY_PAIRED: u16 = 2;
    pub const READ_UNMAPPED: u16 = 4;
    pub const MATE_UNMAPPED: u16 = 8;
    pub const READ_REVERSE: u16 = 16;
    pub const MATE_REVERSE: u16 = 32;
    pub const FIRST_IN_PAIR: u16 = 64;
    pub const SECOND_IN_PAIR: u16 = 128;
    pub const NOT_PRIMARY_ALN: u16 = 256;
    pub const FAIL_QC: u16 = 512;
    pub const DUPLICATE: u16 = 1024;
    pub const SUPPLEMENTARY: u16 = 2048;
}

/// The strand a read is inferred to originate from: `Plus`, `Minus`, or
/// `NA` when it cannot be determined from the read's flags.
#[derive(Clone, Debug, Copy, Eq, Hash)]
pub enum Strand {
    Plus,
    Minus,
    NA,
}

impl Strand {
    /// Returns `true` if the strand could not be determined (`Strand::NA`).
    pub fn is_na(&self) -> bool {
        match self {
            Strand::NA => true,
            _ => false,
        }
    }
}

impl PartialEq for Strand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Strand::NA, Strand::NA) => true,
            (Strand::Plus, Strand::Plus) => true,
            (Strand::Minus, Strand::Minus) => true,
            (_, _) => false,
        }
    }
}

/// Formats a strand using the standard BED/GFF single-character convention:
/// `+` for `Plus`, `-` for `Minus`, and `.` for `NA`.
impl fmt::Display for Strand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Strand::Plus => {
                write!(f, "+")
            }
            Strand::Minus => {
                write!(f, "-")
            }
            Strand::NA => {
                write!(f, ".")
            }
        }
    }
}

/// Parses `"+"`, `"-"`, or `"."` into a [`Strand`].
///
/// # Panics
/// Panics (via `unreachable!()`) if `item` is any other string.
impl From<&str> for Strand {
    fn from(item: &str) -> Self {
        match item {
            "+" => Strand::Plus,
            "-" => Strand::Minus,
            "." => Strand::NA,
            _ => {
                println!("a strand must be - + or . NOT => {}", item);
                unreachable!();
            }
        }
    }
}

/// Error returned when a string does not match any known [`LibType`] name.
#[derive(Debug, PartialEq, Eq)]
pub struct ParseLibType;

/// Parses a [`LibType`] variant from its name (e.g. `"frFirstStrand"`).
///
/// Returns `Err(ParseLibType)` if the string does not match any known
/// library type name.
impl FromStr for LibType {
    type Err = ParseLibType;
    fn from_str(str: &str) -> Result<Self, ParseLibType> {
        let libtype = match str {
            "Unstranded" => LibType::Unstranded,
            "PairedUnstranded" => LibType::PairedUnstranded,
            "frFirstStrand" => LibType::frFirstStrand,
            "frSecondStrand" => LibType::frSecondStrand,
            "fFirstStrand" => LibType::fFirstStrand,
            "fSecondStrand" => LibType::fSecondStrand,
            "ffFirstStrand" => LibType::ffFirstStrand,
            "ffSecondStrand" => LibType::ffSecondStrand,
            "rfFirstStrand" => LibType::rfFirstStrand,
            "rfSecondStrand" => LibType::rfSecondStrand,
            "rFirstStrand" => LibType::rFirstStrand,
            "rSecondStrand" => LibType::rSecondStrand,
            _ => LibType::Invalid,
        };
        if libtype == LibType::Invalid {
            return Err(ParseLibType);
        }
        Ok(libtype)
    }
}

/// Checks a SAM flag bitmask against required and forbidden bits.
///
/// Returns `true` only if every bit set in `in_` is also set in `flag`, and
/// no bit set in `not_in` is set in `flag`.
pub fn check_flag(flag: u16, in_: u16, not_in: u16) -> bool {
    if (not_in & flag) != 0 {
        return false;
    }
    if (in_ & flag) != in_ {
        return false;
    }
    true
}

/// A sequencing library's strandedness/orientation layout (e.g. produced by
/// tools like RSEM/salmon/STAR), used by [`LibType::get_strand`] to infer a
/// read's originating strand from its SAM flags.
///
/// Variant names deliberately do not follow proper CamelCase, to avoid
/// confusion with their first term (`f`, `r`, `ff`, `fr`).
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
    Unstranded,
    PairedUnstranded,
    Invalid,
}

/// Parses a [`LibType`] variant from its name (e.g. `"frFirstStrand"`).
///
/// Unlike [`LibType::from_str`], this does not return a `Result`.
///
/// # Panics
/// Panics (via `unreachable!()`) if `item` does not match any known
/// library type name (`"Unstranded"` and `"Invalid"` are not accepted here).
impl From<&str> for LibType {
    fn from(item: &str) -> Self {
        match item {
            "Unstranded" => LibType::Unstranded,
            "PairedUnstranded" => LibType::PairedUnstranded,
            "frFirstStrand" => LibType::frFirstStrand,
            "frSecondStrand" => LibType::frSecondStrand,
            "fFirstStrand" => LibType::fFirstStrand,
            "fSecondStrand" => LibType::fSecondStrand,
            "ffFirstStrand" => LibType::ffFirstStrand,
            "ffSecondStrand" => LibType::ffSecondStrand,
            "rfFirstStrand" => LibType::rfFirstStrand,
            "rfSecondStrand" => LibType::rfSecondStrand,
            "rFirstStrand" => LibType::rFirstStrand,
            "rSecondStrand" => LibType::rSecondStrand,
            _ => {
                println!("{}", item);
                unreachable!();
            }
        }
    }
}

// with branch prediction this should be fast.

impl LibType {
    /// Infers the strand of a read from its SAM `flag` bits, given this
    /// library's layout.
    ///
    /// Returns `Some(Strand)` (which may be `Strand::NA` for layouts that are
    /// always unstranded) when the flag combination is recognized for this
    /// library type, or `None` when it is not. Returning `Option` rather than
    /// always producing a `Strand` is intentional, so callers cannot ignore
    /// the possibility that a read's strand could not be assigned.
    pub fn get_strand(self: &Self, flag: u16) -> Option<Strand> {
        match self {
            LibType::Unstranded => {
                if check_flag(flag, SamFlag::PAIRED, SamFlag::READ_UNMAPPED) {
                    Some(Strand::NA)
                } else {
                    None
                }
            }
            LibType::PairedUnstranded => {
                if check_flag(
                    flag,
                    SamFlag::PAIRED,
                    SamFlag::READ_UNMAPPED + SamFlag::MATE_UNMAPPED,
                ) {
                    Some(Strand::NA)
                } else {
                    None
                }
            }
            LibType::frFirstStrand => {
                if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::READ_REVERSE,
                    SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_REVERSE,
                ) {
                    Some(Strand::Plus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::READ_REVERSE,
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
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::READ_REVERSE,
                    SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_REVERSE,
                ) {
                    Some(Strand::Minus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::READ_REVERSE,
                    SamFlag::MATE_REVERSE,
                ) {
                    Some(Strand::Plus)
                } else {
                    None
                }
            }
            LibType::fFirstStrand => {
                if check_flag(flag, SamFlag::READ_REVERSE, 0) {
                    Some(Strand::Plus)
                } else if check_flag(flag, 0, SamFlag::READ_REVERSE) {
                    Some(Strand::Minus)
                } else {
                    None
                }
            }
            LibType::fSecondStrand => {
                if check_flag(flag, SamFlag::READ_REVERSE, 0) {
                    Some(Strand::Minus)
                } else if check_flag(flag, 0, SamFlag::READ_REVERSE) {
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
                        + SamFlag::READ_REVERSE
                        + SamFlag::MATE_REVERSE,
                    0,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED
                        + SamFlag::SECOND_IN_PAIR
                        + SamFlag::READ_REVERSE
                        + SamFlag::MATE_REVERSE,
                    0,
                ) {
                    Some(Strand::Plus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR,
                    SamFlag::READ_REVERSE + SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR,
                    SamFlag::READ_REVERSE + SamFlag::MATE_REVERSE,
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
                        + SamFlag::READ_REVERSE
                        + SamFlag::MATE_REVERSE,
                    0,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED
                        + SamFlag::SECOND_IN_PAIR
                        + SamFlag::READ_REVERSE
                        + SamFlag::MATE_REVERSE,
                    0,
                ) {
                    Some(Strand::Minus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR,
                    SamFlag::READ_REVERSE + SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR,
                    SamFlag::READ_REVERSE + SamFlag::MATE_REVERSE,
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
                    SamFlag::READ_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::READ_REVERSE,
                    SamFlag::MATE_REVERSE,
                ) {
                    Some(Strand::Plus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::READ_REVERSE,
                    SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_REVERSE,
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
                    SamFlag::READ_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::READ_REVERSE,
                    SamFlag::MATE_REVERSE,
                ) {
                    Some(Strand::Minus)
                } else if check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::READ_REVERSE,
                    SamFlag::MATE_REVERSE,
                ) || check_flag(
                    flag,
                    SamFlag::PAIRED + SamFlag::SECOND_IN_PAIR + SamFlag::MATE_REVERSE,
                    SamFlag::READ_REVERSE,
                ) {
                    Some(Strand::Plus)
                } else {
                    None
                }
            }
            LibType::rFirstStrand => {
                if check_flag(flag, SamFlag::FIRST_IN_PAIR + SamFlag::READ_REVERSE, 0) {
                    Some(Strand::Plus)
                } else if check_flag(flag, SamFlag::FIRST_IN_PAIR + SamFlag::READ_REVERSE, 0) {
                    Some(Strand::Minus)
                } else {
                    None
                }
            }
            LibType::rSecondStrand => {
                if check_flag(flag, SamFlag::FIRST_IN_PAIR + SamFlag::READ_REVERSE, 0) {
                    Some(Strand::Minus)
                } else if check_flag(flag, SamFlag::FIRST_IN_PAIR + SamFlag::READ_REVERSE, 0) {
                    Some(Strand::Plus)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

// ──────────────────────────────────────────────────────────────
//  Unit tests for the SAM‑flag / strand / library‑type helpers
// ──────────────────────────────────────────────────────────────

// ──────────────────────────────────────────────────────────────
//  Unit tests for the SAM‑flag / strand / library‑type helpers
// ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*; // pulls in everything from the parent module

    // -----------------------------------------------------------------
    //  SamFlag constants
    // -----------------------------------------------------------------
    #[test]
    fn samflag_values_are_correct() {
        assert_eq!(SamFlag::PAIRED, 1);
        assert_eq!(SamFlag::PROPERLY_PAIRED, 2);
        assert_eq!(SamFlag::READ_UNMAPPED, 4);
        assert_eq!(SamFlag::MATE_UNMAPPED, 8);
        assert_eq!(SamFlag::READ_REVERSE, 16);
        assert_eq!(SamFlag::MATE_REVERSE, 32);
        assert_eq!(SamFlag::FIRST_IN_PAIR, 64);
        assert_eq!(SamFlag::SECOND_IN_PAIR, 128);
        assert_eq!(SamFlag::NOT_PRIMARY_ALN, 256);
        assert_eq!(SamFlag::FAIL_QC, 512);
        assert_eq!(SamFlag::DUPLICATE, 1024);
        assert_eq!(SamFlag::SUPPLEMENTARY, 2048);
    }

    // -----------------------------------------------------------------
    //  Strand – basic behaviour
    // -----------------------------------------------------------------
    #[test]
    fn strand_is_na_works() {
        assert!(Strand::NA.is_na());
        assert!(!Strand::Plus.is_na());
        assert!(!Strand::Minus.is_na());
    }

    #[test]
    fn strand_partial_eq() {
        assert_eq!(Strand::Plus, Strand::Plus);
        assert_eq!(Strand::Minus, Strand::Minus);
        assert_eq!(Strand::NA, Strand::NA);
        assert_ne!(Strand::Plus, Strand::Minus);
        assert_ne!(Strand::Plus, Strand::NA);
    }

    #[test]
    fn strand_display() {
        assert_eq!(format!("{}", Strand::Plus), "+");
        assert_eq!(format!("{}", Strand::Minus), "-");
        assert_eq!(format!("{}", Strand::NA), ".");
    }

    #[test]
    fn strand_from_str() {
        assert_eq!(Strand::from("+"), Strand::Plus);
        assert_eq!(Strand::from("-"), Strand::Minus);
        assert_eq!(Strand::from("."), Strand::NA);
    }

    // -----------------------------------------------------------------
    //  ParseLibType – FromStr implementation
    // -----------------------------------------------------------------
    #[test]
    fn parse_libtype_successful() {
        use std::str::FromStr;

        let ok = LibType::from_str("frFirstStrand").unwrap();
        assert_eq!(ok, LibType::frFirstStrand);

        let ok = LibType::from_str("Unstranded").unwrap();
        assert_eq!(ok, LibType::Unstranded);
    }

    #[test]
    fn parse_libtype_invalid_returns_err() {
        use std::str::FromStr;

        let err = LibType::from_str("does_not_exist");
        assert!(err.is_err());
    }

    // -----------------------------------------------------------------
    //  check_flag – core bit‑mask logic
    // -----------------------------------------------------------------
    #[test]
    fn check_flag_rejects_when_not_in_present() {
        // not_in = READ_UNMAPPED (bit 4)
        let flag = SamFlag::READ_UNMAPPED | SamFlag::PAIRED;
        assert!(!check_flag(flag, SamFlag::PAIRED, SamFlag::READ_UNMAPPED));
    }

    #[test]
    fn check_flag_rejects_when_missing_required_bits() {
        // in_ = PAIRED + FIRST_IN_PAIR, but flag only has PAIRED
        let flag = SamFlag::PAIRED;
        assert!(!check_flag(
            flag,
            SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR,
            0
        ));
    }

    #[test]
    fn check_flag_accepts_valid_combination() {
        // flag contains both required bits and none of the forbidden bits
        let flag = SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::READ_REVERSE;
        assert!(check_flag(
            flag,
            SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR,
            SamFlag::MATE_REVERSE
        ));
    }

    // -----------------------------------------------------------------
    //  LibType::get_strand – representative cases
    // -----------------------------------------------------------------
    #[test]
    fn get_strand_unstranded_always_na_ifmapped() {
        let flag = SamFlag::PAIRED; // flag value does not matter for Unstranded
        assert_eq!(LibType::Unstranded.get_strand(flag), Some(Strand::NA));
    }

    #[test]
    fn get_strand_paired_unstranded_conditions() {
        // PairedUnstranded returns NA only when the read is paired
        // and both reads are unmapped.
        let flag = SamFlag::PAIRED;
        assert_eq!(LibType::PairedUnstranded.get_strand(flag), Some(Strand::NA));

        // Missing one of the required bits → None
        let flag = SamFlag::PAIRED + SamFlag::READ_UNMAPPED;
        assert_eq!(LibType::PairedUnstranded.get_strand(flag), None);
    }

    #[test]
    fn get_strand_frfirststrand_plus_and_minus() {
        // Scenario that should yield Strand::Plus
        let flag = SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::READ_REVERSE; // mate is forward
        assert_eq!(LibType::frFirstStrand.get_strand(flag), Some(Strand::Plus));

        // Scenario that should yield Strand::Minus
        let flag = SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR + SamFlag::MATE_REVERSE; // read is forward, mate reverse
        assert_eq!(LibType::frFirstStrand.get_strand(flag), Some(Strand::Minus));
    }

    #[test]
    fn get_strand_ffirststrand_plus_and_minus() {
        // Both reads paired, both strands reversed → Plus
        let flag = SamFlag::PAIRED
            + SamFlag::FIRST_IN_PAIR
            + SamFlag::READ_REVERSE
            + SamFlag::MATE_REVERSE;
        assert_eq!(LibType::ffFirstStrand.get_strand(flag), Some(Strand::Plus));

        // Only read reversed, mate forward → Minus
        let flag = SamFlag::PAIRED + SamFlag::FIRST_IN_PAIR;
        assert_eq!(LibType::ffFirstStrand.get_strand(flag), Some(Strand::Minus));
    }

    #[test]
    fn get_strand_ffirststrand_simple() {
        // Read reversed → Plus
        let flag = SamFlag::READ_REVERSE;
        assert_eq!(LibType::fFirstStrand.get_strand(flag), Some(Strand::Plus));

        // Read not reversed → Minus
        let flag = 0;
        assert_eq!(LibType::fFirstStrand.get_strand(flag), Some(Strand::Minus));
    }

    // You can continue adding tests for the remaining LibType variants
    // (rf*, r*, etc.) following the same pattern.
}
