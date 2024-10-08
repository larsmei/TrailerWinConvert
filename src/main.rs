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

#[derive(Clone, Debug)]
struct polyline {
    x: Vec<f64>,
    y: Vec<f64>,
}
#[derive(Debug, Clone, Copy)]
struct circle {
    radius: f64,
    x: f64,
    y:f64,
}

#[derive(PartialEq)]
enum Reading {
    x,
    y,
    radius,
}

#[derive(PartialEq)]
enum Gobject {
    polyline,
    cirle,
    none,
}
fn read_cor_file(filename: &str) {
    println!("reading file {}", filename);
    let mut polylines : Vec<polyline> = Vec::new();
    let mut circles : Vec<circle> = Vec::new();
    let lines = fs::read_to_string(&filename).expect("Unable to read file");
    let mut read : Reading = Reading::x;
    let mut go : Gobject= Gobject::none;
    let mut circ = circle{radius:0.0, x:0.0, y:0.0};
    for line in lines.lines() {
        if (line.eq_ignore_ascii_case("pd")) {
            go = Gobject::polyline;
            polylines.push(polyline{x:Vec::new(),y:Vec::new()});
            read = Reading::x;
        }
        else if (line.eq_ignore_ascii_case("ci")) {
            go = Gobject::cirle;
            read = Reading::radius;
        }
        else if (line.eq_ignore_ascii_case("pu")) {
            go = Gobject::none;
        }
        else {
            if (go == Gobject::cirle){
                if (read == Reading::radius) {
                    //println!("parsing {} as float",line.trim());
                    let radius:f64 = line.trim().parse().unwrap();
                    circ.radius=radius;
                    read = Reading::x;
                }
                else if (read == Reading::x) {
                    //println!("parsing {} as float",line.trim());
                    let x:f64 = line.trim().parse().unwrap();
                    circ.x=x;
                    read = Reading::y;
                }
                else if (read == Reading::y) {
                    println!("parsing {} as float",line.trim());
                    let y:f64 = line.trim().parse().unwrap();
                    circ.y=y;
                    circles.push(circ);
                    read = Reading::radius;
                }
            }
            if (go == Gobject::polyline ){
                if (read == Reading::x) {
                    //println!("parsing {} as float",line.trim());
                    let x:f64 = line.trim().parse().unwrap();
                    let mut p = polylines.pop().unwrap();
                    p.x.push(x);
                    polylines.push(p);
                    read = Reading::y;
                }
                else if ( read == Reading::y) {
                    //println!("parsing {} as float",line.trim());
                    let y:f64 = line.trim().parse().unwrap();
                    let mut p = polylines.pop().unwrap();
                    p.y.push(y);
                    polylines.push(p);
                    read =Reading::x;
                }
            }
        }
    }
    println!("{:?}",polylines);
    println!("{:?}",circles);

}