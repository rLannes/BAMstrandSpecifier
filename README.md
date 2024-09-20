This is an opinionated library and executable that assigns the strand of the fragment they are from to each read in an SF tag. SF stands for Strand Fragment.
The SF tag can take the follwing value: ["+", "-", "."]

how to use the binary:

```
strand_specifier --input <InputBam> --output <OutputBam> --LibType <Optional one of the following:
                                                        [frFirstStrand (default), frSecondStrand,
                                                        fFirstStrand, fSecondStrand,
                                                        ffFirstStrand, ffSecondStrand,
                                                        rfFirstStrand, rfSecondStrand,
                                                        rFirstStrand, rSecondStrand] >
```
