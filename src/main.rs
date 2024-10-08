use std::fs;
// extern crate dxf;
use clap::Parser;

/// Simple converter between cor and dxf
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// name of the file to convert
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    println!("{args:?}");
    let filename = &args.file;
    let filetype = filename.as_str().chars().rev().take(3).collect::<String>();
    let filetype = filetype.chars().rev().collect::<String>();
    let dxf = String::from("dxf");
    let cor = String::from("cor");

    if filetype.eq_ignore_ascii_case(&dxf) {
        println!("reading file {} as DXF", filename);
    }
    else if filetype.eq_ignore_ascii_case(&cor) {
        println!("reading file {} as COR", filename);
        read_cor_file(filename);
    }
    else {
        println!("file: {} not supported!", filename);
    }

}

enum Reading {
    x,
    y,
    radius,
}

#[derive(PartialEq)]
enum Gobject {
    polyline,
    cirle,
}
fn read_cor_file(filename: &str) {
    println!("reading file {}", filename);
    let lines = fs::read_to_string(&filename).expect("Unable to read file");
    let mut read : Reading;
    let mut go : Gobject;
    for line in lines.lines() {
        if (line.eq_ignore_ascii_case("pd")) {
            go = Gobject::polyline;
        }
        else if (line.eq_ignore_ascii_case("ci")) {
            go = Gobject::cirle;
        }
        if (go == Gobject::polyline ){

        }
    }

}