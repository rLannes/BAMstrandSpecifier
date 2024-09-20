This is an opinionated library and executable that assigns the strand of the fragment they are from to each read in an SF tag. SF stands for Strand Fragment.
The SF tag can take the follwing value: ["+", "-", "."].
Whenever possible the orientation of the mate is used to determine the strand.

## Installation

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone this repository
3. Inside the BamStrandSpecifier folder, run:
    cargo build --release
4. The binary will be available at `BamStrandSpecifier/target/release/strand_specifier`

## Usage

### Command-line

```
strand_specifier --input <InputBam> --output <OutputBam> --LibType <Optional one of the following:
                                                        [frFirstStrand (default), frSecondStrand,
                                                        fFirstStrand, fSecondStrand,
                                                        ffFirstStrand, ffSecondStrand,
                                                        rfFirstStrand, rfSecondStrand,
                                                        rFirstStrand, rSecondStrand] >
```

### Library


The library exposes the following:

- The "SamFlag" structure store all the possible Value a Sam flag can take, it allows to make a code more readable when working with falg.
- The "Strand" Enum represent the different value a strand can take ['+', '-', '.'].
- The 'check_flag' function that allow to test if a flag is set, or not set in another flag.
- Finaly "LibType" an Enum that represent the different library type and implement get_strand().

```
    use strand_specifier_lib::{LibType, Strand, check_flag, SamFlag};

    let current_lib = "frFirstStrand";
    let libtype = match LibType::from_str(current_lib){
        Ok(lib) => lib,
        Err(_) => panic!("incorrect lib type {}", current_lib)
    };

    let samflag = 186;  //(u16);
    // return a fragment orientation for a flag:
    let strand = match libtype.get_strand(samflags){
        Some(x) => x,
        None => Strand::NA
    };

    // For the flags 84 check if the flag 2 is set AND the flag 256 is NOT set.
    // you can pass multiple condition at once eg. 2 + 16 + 64.
    assert_eq!(true, check_flag(84, 2, 256));

    // you can Also you the SamFlag structure for better lisibility.
    assert_eq!(true, check_flag(18, SamFlag::Paired +  SamFlag::READ_RERVERSE, SamFlag::NOT_PRIMARY_ALN));

```


Let me know if you have any issues/comments, or ideas!


