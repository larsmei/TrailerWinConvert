use std::string::String;
use std::fs;
use std::fs::FileTimes;
use std::io::Write;
use clap::builder::Str;
use dxf::{Drawing, Point};
use clap::Parser;
use dxf::entities::{Circle, Entity, EntityType, Line, Polyline};

/// Simple converter between .cor and .dxf files.
/// Currently full circle, lines and polylines with line-segments supported
/// no support for polyline from dxf to cor
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// name of the file to convert
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    //println!("{args:?}");
    let filename = &args.file;
    let filetype = filename.as_str().chars().rev().take(3).collect::<String>();
    let filetype = filetype.chars().rev().collect::<String>();
    let dxf = String::from("dxf");
    let cor = String::from("cor");

    if filetype.eq_ignore_ascii_case(&dxf) {
        println!("reading file {} as DXF", filename);
        dxf2cor(filename);
    }
    else if filetype.eq_ignore_ascii_case(&cor) {
        println!("reading file {} as COR", filename);
        cor2dxf(filename);
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

#[derive(Debug, Clone, Copy)]
struct arc {
    radius: f64,
    x: f64,
    y: f64,
    start: f64,
    end: f64,
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
    arc,
    none,
}
fn cor2dxf(filename: &str) {
    //println!("reading file {}", filename);
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
                    //println!("parsing {} as float",line.trim());
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
    //println!("{:?}",polylines);
    //println!("{:?}",circles);
    let prefix: String = filename.split(".").take(1).collect();
    let dxfout= format!("{}_out.dxf",prefix);
    write_dxf(&dxfout, &polylines, &circles);
}

fn write_dxf(filename: &str, polylines: &Vec<polyline>, circles: &Vec<circle> ) {
    let mut drawing = Drawing::new();
    //drawing.add_entity(Entity::new(EntityType::Line(Line::default())));
    println!("writing dxf to {}", filename);
    for polyline in polylines {
        //println!("{:?}",polyline);
        let len = polyline.x.len();
        for  i in 1..len  {
            let p1 = Point::new(polyline.x[i-1],polyline.y[i-1],0.0);
            let p2 = Point::new(polyline.x[i],polyline.y[i],0.0);
            drawing.add_entity(Entity::new(EntityType::Line(Line::new(p1, p2))));
            //println!("{:?}",(polyline.x[i],polyline.y[i]));
        }

    }
    for circ in circles {
        //println!("{:?}",circ);
        let center= Point::new(circ.x,circ.y,0.0);
        drawing.add_entity(Entity::new(EntityType::Circle(Circle::new(center,circ.radius))));
    }
    drawing.save_file(filename).unwrap();
}

fn write_cor(filename: &str, polylines: &Vec<polyline>, circles: &Vec<circle>, arcs: &Vec<arc> ) {
    println!("writing cor to {}", filename);
    let mut lines: Vec<String> = Vec::new();
    for polyline in polylines {
        lines.push("PD".to_owned());
        for i in 0..polyline.x.len() {
            lines.push(format!(" {}",polyline.x[i]));
            lines.push(format!(" {}",polyline.y[i]));
        }
        lines.push("PU".to_owned());
    }
    for circle in circles {
        lines.push("CI".to_owned());
        lines.push(format!(" {}",circle.radius));
        lines.push(format!(" {}",circle.x));
        lines.push(format!(" {}",circle.y));
        lines.push("PU".to_owned());
    }
    for arc in arcs {
        lines.push("ARC".to_owned());
        lines.push(format!(" {}",arc.x));
        lines.push(format!(" {}",arc.y));
        lines.push(format!(" {}",arc.radius));
        lines.push(format!(" {}",arc.start));
        lines.push(format!(" {}",arc.end));
        lines.push("PU".to_owned());
    }
    let mut file = fs::File::create(filename).unwrap();
    file.write_all(lines.join("\r\n").as_bytes()).unwrap();
    //println!("{:?}",lines);
}

fn dxf2cor(filename: &str) {
    let mut polylines : Vec<polyline> = Vec::new();
    let mut circles : Vec<circle> = Vec::new();
    let mut arcs : Vec<arc> = Vec::new();
    let drawing = Drawing::load_file(filename).unwrap();
    for e in drawing.entities() {
        match e.specific {
            EntityType::Line(ref line) => {
                //println!("{:?}",line);
                let mut pline = polyline{x:Vec::new(),y:Vec::new()};
                pline.x.push(line.p1.x);
                pline.x.push(line.p2.x);
                pline.y.push(line.p1.y);
                pline.y.push(line.p2.y);
                polylines.push(pline);
            }
            EntityType::Circle(ref rcircle) => {
                //println!("{:?}",circle);
                circles.push(circle{radius:rcircle.radius,x:rcircle.center.x,y:rcircle.center.y});
            }
            EntityType::Arc(ref arc) => {
                //println!("{:?}",arc);
                let mut larc = arc{radius:0.0,x:0.0,y:0.0,start:0.0,end:0.0};
                larc.radius=arc.radius;
                larc.x=arc.center.x;
                larc.y=arc.center.y;
                larc.start=arc.start_angle;
                larc.end=arc.end_angle;
                arcs.push(larc);
            }
            _=> (),
        }
    }
    let prefix: String = filename.split(".").take(1).collect();
    let corout= format!("{}_out.cor",prefix);
    write_cor(&corout, &polylines, &circles, &arcs);
}