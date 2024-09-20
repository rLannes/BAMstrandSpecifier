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
