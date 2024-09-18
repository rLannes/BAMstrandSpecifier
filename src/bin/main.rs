use rust_htslib::bam::{Header, HeaderView, IndexedReader, Read, Writer};
use rust_htslib::bam::record::Record;
use clap::Parser;
use strand_specifier_lib::LibType;
use std::str::FromStr;



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of Input file
    #[arg(short, long)]
    input: String,
    /// Name of Output file
    #[arg(short, long)]
    output: String,
    /// Name of LibType Input file must be one of the following:
    /// [frFirstStrand (default), frSecondStrand,
    /// fFirstStrand, fSecondStrand,
    /// ffFirstStrand, ffSecondStrand,
    /// rfFirstStrand, rfSecondStrand,
    /// rFirstStrand, rSecondStrand]
    #[arg(short, default_value_t = format!("{}", "frFirstStrand"), long)]
    LibType: String,

}
fn main(){


    let cli = Args::parse();
    let libtype = match LibType::from_str(&cli.LibType){
        Ok(lib) => lib,
        Err(_) => panic!("incorrect lib type {}", cli.LibType)
    };

}


