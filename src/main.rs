use std::string::String;
use std::fs;
use std::io::Write;
use dxf::{Drawing, Point};
use clap::Parser;
use dxf::entities::{Arc, Circle, Entity, EntityType, Line};

/// TrailerWinConvert
/// Simple converter between .cor and .dxf files.
/// Supported entities dxf -> cor:
/// line, circle, arc
/// Supported entities cor -> dxf:
/// line, Polyline, circle, arc
/// written by Lars Meindl, lars.meindl@googlemail.com
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
    let suffix = filename.split(".").last().unwrap();
    let dxf = String::from("dxf");
    let cor = String::from("cor");

    if suffix.eq_ignore_ascii_case(&dxf) {
        println!("reading file {} as DXF", filename);
        dxf2cor(filename);
    }
    else if suffix.eq_ignore_ascii_case(&cor) {
        println!("reading file {} as COR", filename);
        cor2dxf(filename);
    }
    else {
        println!("file: {} not supported!", filename);
    }

}

#[derive(Clone, Debug)]
struct TwPolyline {
    x: Vec<f64>,
    y: Vec<f64>,
}
#[derive(Debug, Clone, Copy)]
struct TwCircle {
    radius: f64,
    x: f64,
    y:f64,
}
#[derive(Debug, Clone, Copy)]
struct TwArc {
    radius: f64,
    x: f64,
    y: f64,
    start: f64,
    end: f64,
}

#[derive(PartialEq)]
enum Reading {
    X,
    Y,
    Radius,
    Start,
    End,
}

#[derive(PartialEq)]
enum Gobject {
    Polyline,
    Circle,
    Arc,
    None,
}
fn cor2dxf(filename: &str) {
    //println!("reading file {}", filename);
    let mut polylines : Vec<TwPolyline> = Vec::new();
    let mut circles : Vec<TwCircle> = Vec::new();
    let mut arcs : Vec<TwArc> = Vec::new();
    let lines = fs::read_to_string(&filename).expect("Unable to read file");
    let mut read : Reading = Reading::X;
    let mut go : Gobject= Gobject::None;
    let mut circ = TwCircle {radius:0.0, x:0.0, y:0.0};
    let mut larc = TwArc {radius:0.0, x:0.0, y:0.0, start:0.0, end:0.0};
    for line in lines.lines() {
        if line.eq_ignore_ascii_case("pd") {
            go = Gobject::Polyline;
            polylines.push(TwPolyline {x:Vec::new(),y:Vec::new()});
            read = Reading::X;
        }
        else if line.eq_ignore_ascii_case("ci") {
            go = Gobject::Circle;
            read = Reading::Radius;
        }
        else if line.eq_ignore_ascii_case("arc") {
            go = Gobject::Arc;
            read = Reading::X;
        }
        else if line.eq_ignore_ascii_case("pu") {
            go = Gobject::None;
        }
        else {
            if go == Gobject::Circle {
                if read == Reading::Radius {
                    //println!("parsing {} as float",line.trim());
                    let radius:f64 = line.trim().parse().unwrap();
                    circ.radius=radius;
                    read = Reading::X;
                }
                else if read == Reading::X {
                    //println!("parsing {} as float",line.trim());
                    let x:f64 = line.trim().parse().unwrap();
                    circ.x=x;
                    read = Reading::Y;
                }
                else if read == Reading::Y {
                    //println!("parsing {} as float",line.trim());
                    let y:f64 = line.trim().parse().unwrap();
                    circ.y=y;
                    circles.push(circ);
                    read = Reading::Radius;
                }
            }
            if go == Gobject::Arc {
                if read == Reading::Radius {
                    let radius:f64 = line.trim().parse().unwrap();
                    larc.radius=radius;
                    read = Reading::Start;
                }
                else if read == Reading::X {
                    let x:f64 = line.trim().parse().unwrap();
                    larc.x=x;
                    read = Reading::Y;
                }
                else if read == Reading::Y {
                    let y:f64 = line.trim().parse().unwrap();
                    larc.y=y;
                    read = Reading::Radius;
                }
                else if read == Reading::Start {
                    let start:f64 = line.trim().parse().unwrap();
                    larc.start=start;
                    read = Reading::End;
                }
                else if read == Reading::End {
                    let end:f64 = line.trim().parse().unwrap();
                    larc.end=end;
                    arcs.push(larc);
                    read = Reading::X;
                }
            }
            if go == Gobject::Polyline{
                if read == Reading::X {
                    //println!("parsing {} as float",line.trim());
                    let x:f64 = line.trim().parse().unwrap();
                    let mut p = polylines.pop().unwrap();
                    p.x.push(x);
                    polylines.push(p);
                    read = Reading::Y;
                }
                else if  read == Reading::Y {
                    //println!("parsing {} as float",line.trim());
                    let y:f64 = line.trim().parse().unwrap();
                    let mut p = polylines.pop().unwrap();
                    p.y.push(y);
                    polylines.push(p);
                    read =Reading::X;
                }
            }
        }
    }
    //println!("{:?}",polylines);
    //println!("{:?}",circles);
    let prefix: String = filename.split(".").take(1).collect();
    let dxfout= format!("{}_out.dxf",prefix);
    write_dxf(&dxfout, &polylines, &circles, &arcs);
}

fn write_dxf(filename: &str, polylines: &Vec<TwPolyline>, circles: &Vec<TwCircle>, arcs: &Vec<TwArc>) {
    let mut drawing = Drawing::new();
    //drawing.add_entity(Entity::new(EntityType::Line(Line::default())));
    println!("writing dxf to {}", filename);
    for polyline in polylines {
        //println!("{:?}",Polyline);
        let len = polyline.x.len();
        for  i in 1..len  {
            let p1 = Point::new(polyline.x[i-1],polyline.y[i-1],0.0);
            let p2 = Point::new(polyline.x[i],polyline.y[i],0.0);
            drawing.add_entity(Entity::new(EntityType::Line(Line::new(p1, p2))));
            //println!("{:?}",(Polyline.x[i],Polyline.y[i]));
        }

    }
    for circ in circles {
        //println!("{:?}",circ);
        let center= Point::new(circ.x,circ.y,0.0);
        drawing.add_entity(Entity::new(EntityType::Circle(Circle::new(center,circ.radius))));
    }
    for larc in arcs {
        let center= Point::new(larc.x,larc.y,0.0);
        drawing.add_entity(Entity::new(EntityType::Arc(Arc::new(center,larc.radius,larc.start,larc.end))));
    }
    drawing.save_file(filename).unwrap();
}

fn write_cor(filename: &str, polylines: &Vec<TwPolyline>, circles: &Vec<TwCircle>, arcs: &Vec<TwArc> ) {
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
    let mut polylines : Vec<TwPolyline> = Vec::new();
    let mut circles : Vec<TwCircle> = Vec::new();
    let mut arcs : Vec<TwArc> = Vec::new();
    let drawing = Drawing::load_file(filename).unwrap();
    for e in drawing.entities() {
        match e.specific {
            EntityType::Line(ref line) => {
                //println!("{:?}",line);
                let mut pline = TwPolyline {x:Vec::new(),y:Vec::new()};
                pline.x.push(line.p1.x);
                pline.x.push(line.p2.x);
                pline.y.push(line.p1.y);
                pline.y.push(line.p2.y);
                polylines.push(pline);
            }
            EntityType::Circle(ref rcircle) => {
                //println!("{:?}",circle);
                circles.push(TwCircle {radius:rcircle.radius,x:rcircle.center.x,y:rcircle.center.y});
            }
            EntityType::Arc(ref arc) => {
                //println!("{:?}",arc);
                let mut larc = TwArc {radius:0.0,x:0.0,y:0.0,start:0.0,end:0.0};
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