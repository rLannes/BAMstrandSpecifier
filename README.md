This is an opinionated library and executable that assigns the strand of the fragment they are from to each read in an SF tag. SF stands for Strand Fragment.
The SF tag can take the follwing value: ["+", "-", "."]

As for any rust code, you need to install rust https://www.rust-lang.org/tools/install 
Then you need to compile the code. Inside the BamstrandSpecifier Folder run:
- cargo build --release
- the binary is then at BamstrandSpecifier/target/release/strand_specifier

how to use the binary:

```
strand_specifier --input <InputBam> --output <OutputBam> --LibType <Optional one of the following:
                                                        [frFirstStrand (default), frSecondStrand,
                                                        fFirstStrand, fSecondStrand,
                                                        ffFirstStrand, ffSecondStrand,
                                                        rfFirstStrand, rfSecondStrand,
                                                        rFirstStrand, rSecondStrand] >
```

The library exposes:

The SamFlag structure store all the possible Value a Sam flag can take, it allows to make a code more readable when working with falg.
The Strand Enum represent the different value a strand can take ['+', '-', '.'].
Finaly LibType and Enum that represent the different library type and implement get_strand().

```
    let current_lib = "frFirstStrand";
    let libtype = match LibType::from_str(current_lib){
        Ok(lib) => lib,
        Err(_) => panic!("incorrect lib type {}", current_lib)
    };

    // return a fragment orientation for a flag:
    let strand = match libtype.get_strand(record.flags()){
        Some(x) => x,
        None => strand_specifier_lib::Strand::NA
    };


```


Let me know if you have any issues/comments, or ideas!
