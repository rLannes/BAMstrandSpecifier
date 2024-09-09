use rust_htslib::bam;
use rust_htslib::bam::{Header, HeaderView, IndexedReader, Read};
use rust_htslib::errors::Error;
use rust_htslib::bam::record::Record;

/// This stucture store as constant all possible value that a SAM read flag can take
/// to access SamFlag::<value> 
/// example: SamFlag::PAIRED
/// for better readability we use this structure instead of passing value directly,
/// code will be more verbose but will document itself and logic error will be easier 
/// to avoid/catch.
/// 
#[non_exhaustive]
struct SamFlag;

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
// as to avoid confusion with the first term
#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq)]
pub enum LibType {
    frFirstStrand,
    frSecondStrand,
    fFirstStrand,
    fSecondStrand,
    ffFirstStrand,
    frSecondStrand,

}



impl LibType{
    pub fn get_strand(self: &Self, flag: u16) -> Option<Strand>{
        match self{

            LibType::frFirstStrand => {
                //SamFlag::FIRST_IN_PAIR + SamFlag::PAIRED + SamFlag::READ_RERVERSE
                // need to test both approach. but it should be equivalent
                if  check_flag(flag, 81, 32) ||
                    check_flag(flag, 161, 16) {
                   Some(Strand::Plus)     
                }
                else if  check_flag(flag, 97, 16) ||
                    check_flag(flag, 145, 32){
                    Some(Strand::Minus)
                }
                else{None}
            },
            LibType::frSecondStrand => {
                if  check_flag(flag, 81, 32) ||
                    check_flag(flag, 161, 16) {
                   Some(Strand::Minus)     
                }
                else if  check_flag(flag, 97, 16) ||
                    check_flag(flag, 145, 32){
                    Some(Strand::Plus)
                }
                else{None}
            },
            LibType::fFirstStrand => {
                if  check_flag(flag, 81,0)
                    {
                   Some(Strand::Plus)     
                }
                else if  check_flag(flag, 65, 16){
                    Some(Strand::Minus)
                }
                else{None}
            },
            LibType::fSecondStrand => {
                if  check_flag(flag, 81, 0)
                    {
                   Some(Strand::Minus)     
                }
                else if  check_flag(flag, 65, 16){
                    Some(Strand::Plus)
                }
                else{None}
            },
             LibType::ffFirstStrand =>{
                if check_flag(flag, 81, 32) ||
                   check_flag(flag, 145, 32){
                    Some(Strand::Plus)
                }
                else if check_flag(flag, 161, 16) ||
                 check_flag(flag, 97, 16) {
                    Some(Strand::Minus) 
                }
                else{None}
            },
            LibType::ffSecondStrand =>{
                if check_flag(flag, 81, 32) ||
                   check_flag(flag, 145, 32){
                    Some(Strand::Minus)
                }
                else if check_flag(flag, 161, 16) ||
                 check_flag(flag, 97, 16) {
                    Some(Strand::Plus) 
                }
                else{None}
            },
            //LibType:: = >

            _ => None
        }
    }
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
