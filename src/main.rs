extern crate flate2;

use flate2::write::{GzEncoder, GzDecoder};
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() != 4 {
        eprintln!("Usage: <source> <target> <C | D>");
        return;
    }

    let flag = args[3].to_lowercase();
    if flag == "c" {
        let mut input = BufReader::new(File::open(args[1].clone()).unwrap());
        let output = File::create(args[2].clone()).unwrap();

        let mut encoder = GzEncoder::new(output, Compression::default());
        let start = Instant::now();

        copy(&mut input, &mut encoder).unwrap();
        let output = encoder.finish().unwrap();
        println!("Source len : {:?}", input.get_ref().metadata().unwrap().len());
        println!("Target len : {:?}", output.metadata().unwrap().len());
        println!("Elapsed time : {:?}", start.elapsed());
    }else if flag == "d" {
        let mut input = BufReader::new(File::open(args[1].clone()).unwrap());
        let output = File::create(args[2].clone()).unwrap();

        let mut decoder = GzDecoder::new(output);
        let start = Instant::now();

        copy(&mut input, &mut decoder).unwrap();
        let output = decoder.finish().unwrap();

        println!("Source len : {:?}", input.get_ref().metadata().unwrap().len());
        println!("Target len : {:?}", output.metadata().unwrap().len());
        println!("Elapsed time : {:?}", start.elapsed());
    }else {
        eprintln!("3rd argument should be C or D. but provided : {}", flag);
    }


}
