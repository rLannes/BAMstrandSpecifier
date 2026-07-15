# BAMstrandSpecifier

A Rust library for resolving the strand of origin of RNA-seq reads from their SAM bitwise flags and library preparation protocol. If you write custom BAM-processing code and need to know which strand a fragment comes from, this crate gives you a single function call — `LibType::get_strand(flag)` — instead of reimplementing the flag logic yourself every time.

The crate also provides named constants for all SAM flag bits (`SamFlag::PAIRED`, `SamFlag::READ_REVERSE`, …) and a general-purpose flag-testing helper (`check_flag`), both of which are useful on their own when manipulating BAM records in Rust.

A command-line tool is included as well. It walks a BAM file and writes a custom **SF** (Strand Fragment) tag to each read, with values `+`, `-`, or `.`. This can be convenient for quick inspection in a genome browser or for piping into downstream scripts, but the main value of this project is the library itself.

## Supported library types

The `--LibType` flag follows the same naming convention used by Salmon and Kallisto. The first letter(s) describe the relative orientation of the mates; the suffix indicates which read carries the sense strand.

| LibType | Layout | Paired | Description |
|---|---|---|---|
| frFirstStrand | fr (inward) | yes | First-read is on the reverse strand of the RNA (e.g. dUTP, Illumina TruSeq Stranded) |
| frSecondStrand | fr (inward) | yes | First-read is on the sense strand of the RNA (e.g. Ligation method) |
| fFirstStrand | f (forward only) | no | Single-end, read is reverse-complemented relative to the RNA |
| fSecondStrand | f (forward only) | no | Single-end, read matches the sense strand of the RNA |
| ffFirstStrand | ff (same direction) | yes | Both mates on the same strand; first-read reverse = sense |
| ffSecondStrand | ff (same direction) | yes | Both mates on the same strand; first-read forward = sense |
| rfFirstStrand | rf (outward) | yes | Outward-facing mates; first-read forward, mate reverse = sense |
| rfSecondStrand | rf (outward) | yes | Outward-facing mates; reversed assignment |
| rFirstStrand | r (reverse only) | no | Single-end reverse; read reversed = sense |
| rSecondStrand | r (reverse only) | no | Single-end reverse; reversed assignment |
| Unstranded | — | no | Always assigns `.` (no strand information) |
| PairedUnstranded | — | yes | Always assigns `.`; requires both mates mapped |

If you are unsure which type your data uses, `frFirstStrand` (dUTP / Illumina TruSeq Stranded) is by far the most common for recent Illumina paired-end RNA-seq.

## Installation

1. Install Rust: <https://www.rust-lang.org/tools/install>
2. Clone this repository.
3. Inside the `BAMstrandSpecifier` folder, run:

```bash
cargo build --release
```

4. The binary will be at `BAMstrandSpecifier/target/release/strand_specifier`.

## Usage

### As a library

Add the crate to your `Cargo.toml`, then:

```rust
use std::str::FromStr;
use strand_specifier_lib::{LibType, Strand, check_flag, SamFlag};

// Parse a library type from a string
let current_lib = "frFirstStrand";
let libtype = match LibType::from_str(current_lib) {
    Ok(lib) => lib,
    Err(_) => panic!("Incorrect library type: {}", current_lib),
};

// Get the fragment strand for a given SAM flag
let samflag: u16 = 163;
let strand = match libtype.get_strand(samflag) {
    Some(s) => s,
    None => Strand::NA,
};

// Low-level flag checking:
// check_flag(flag, must_be_set, must_not_be_set)
//
// Check that PROPERLY_PAIRED is set AND NOT_PRIMARY_ALN is not set
assert_eq!(true, check_flag(82, SamFlag::PROPERLY_PAIRED, SamFlag::NOT_PRIMARY_ALN));

// You can combine multiple conditions with +
assert_eq!(
    true,
    check_flag(
        19,
        SamFlag::PAIRED + SamFlag::PROPERLY_PAIRED,
        SamFlag::NOT_PRIMARY_ALN
    )
);
```

### Library API at a glance

- **`SamFlag`** — A struct with named constants for every SAM flag bit (e.g. `SamFlag::PAIRED`, `SamFlag::READ_REVERSE`). Using these instead of raw integers makes flag logic self-documenting. All constants are `u16` and evaluated at compile time, so there is no runtime cost.

- **`Strand`** — An enum with variants `Plus`, `Minus`, and `NA`. Implements `Display` (`+`, `-`, `.`) and can be constructed from `&str`.

- **`LibType`** — An enum representing the library preparation protocol. Its method `get_strand(flag: u16) -> Option<Strand>` returns the fragment strand for a given SAM flag, or `None` if the flag combination is not consistent with the library type. Whenever the read is paired, the orientation of the mate is used alongside the read's own orientation, which makes the assignment more robust than relying on the read alone.

- **`check_flag(flag, must_be_set, must_not_be_set) -> bool`** — A low-level helper that tests whether all bits in `must_be_set` are present in `flag` and all bits in `must_not_be_set` are absent.

```rust
// You can also use the SamFlag constants for better legibility.
assert_eq!(true, check_flag(18, SamFlag::PROPERLY_PAIRED + SamFlag::READ_REVERSE, SamFlag::NOT_PRIMARY_ALN));
```

A CLI wrapper is provided for convenience. It reads a BAM, assigns an SF tag to every read, and writes a new BAM:

```
strand_specifier \
    --input <InputBam> \
    --output <OutputBam> \
    --LibType <LibraryType>
```

`--LibType` is optional and defaults to `frFirstStrand`. See the table above for accepted values.

## Input requirements

- The input must be a valid BAM file.
- For paired-end library types (`fr*`, `ff*`, `rf*`, `PairedUnstranded`), reads should be paired. Unpaired reads or reads whose mate is unmapped will receive no SF tag (the function returns `None`).
- The BAM does not need to be sorted or indexed.



## Contributing

If you find a bug, have a question, or want to contribute test data for a library type that has not been validated yet, please open a GitHub Issue.

## License

MIT 