use rust_htslib::bam::{record, Header, HeaderView, IndexedReader, Read, Writer};
use rust_htslib::bam::record::Record;
use clap::Parser;
use strand_specifier_lib::{LibType, Strand};
use std::str::FromStr;
use rust_htslib::{
    bam,
    bam::record::Aux,
};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of indexed input bam file
    #[arg(short, long)]
    input: String,
    /// Name of Output bam file
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



let mut bam = bam::IndexedReader::from_path(cli.input.as_str()).unwrap();
let header = bam::Header::from_template(bam.header());
let mut out = bam::Writer::from_path(cli.output.as_str(), &header, bam::Format::Bam).unwrap();

let mut strand: Strand;
let mut record: Record;
// copy reverse reads to new BAM file
for r in bam.records() {
    record = r.unwrap();
    strand = match libtype.get_strand(record.flags()){
        Some(x) => x,
        None => strand_specifier_lib::Strand::NA
    };
    record.push_aux(b"FS", Aux::String(format!("{}", strand).as_str())).unwrap();
        out.write(&record).unwrap();
}

}


