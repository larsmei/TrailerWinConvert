mod cor_file;
use std::string::String;
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
fn cor2dxf(filename: &str){
    let doc = cor_file::Document::load_file(filename);
    let mut drawing = Drawing::new();
    println!("reading cor from {}", filename);
    for e in doc.entities {
        match e {
            cor_file::Shape::Circle {radius, center} =>{
                let center= Point::new(center.x,center.y,0.0);
                drawing.add_entity(Entity::new(EntityType::Circle(Circle::new(center,radius))));  
            },
            cor_file::Shape::Arc {radius, center, start, end} =>{
                let center= Point::new(center.x,center.y,0.0);
                drawing.add_entity(Entity::new(
                    EntityType::Arc(Arc::new(center,radius,start,end))
                ));
            },
            cor_file::Shape::Polyline {points} =>{
                let len = points.len();
                for  i in 1..len  {
                    let p1 = Point::new(points[i-1].x,points[i-1].y,0.0);
                    let p2 = Point::new(points[i].x,points[i].y,0.0);
                    drawing.add_entity(Entity::new(EntityType::Line(Line::new(p1, p2))));
                }
            }
            _=> ()
        }
        
    }
    let prefix: String = filename.split(".").take(1).collect();
    let dxfout= format!("{}_out.dxf",prefix);
    println!("writing dxf to {}", dxfout);
    drawing.save_file(dxfout).unwrap();
        
}
fn dxf2cor(filename: &str) {
    let mut cor = cor_file::Document::new();
    println!("reading dxf from {}", filename);
    let drawing = Drawing::load_file(filename).unwrap();
    for e in drawing.entities() {
        match e.specific {
            EntityType::Line(ref line) => {
                let mut points:Vec<cor_file::Point> = Vec::new();
                points.push(cor_file::Point{x:line.p1.x,y:line.p1.y});
                points.push(cor_file::Point{x:line.p2.x,y:line.p2.y});
                let corline = cor_file::Shape::Polyline {points};
                cor.entities.push(corline);
            },
            EntityType::Polyline(ref line) => {
                let mut points:Vec<cor_file::Point> = Vec::new();
                for p in line.vertices() {
                    points.push(cor_file::Point{x:p.location.x,y:p.location.y});
                }
                let corline = cor_file::Shape::Polyline {points};
                cor.entities.push(corline);
            },
            EntityType::Circle(ref rcircle) => {
                cor.entities.push(cor_file::Shape::Circle {
                    radius: rcircle.radius,
                    center: cor_file::Point{x:rcircle.center.x,y:rcircle.center.y},
                });
            },
            EntityType::Arc(ref arc) => {
                cor.entities.push(cor_file::Shape::Arc {
                    radius: arc.radius,
                    center: cor_file::Point{x:arc.center.x,y:arc.center.y},
                    start: arc.start_angle,
                    end: arc.end_angle
                });
            },
            _=> (),
        }
    }
    let prefix: String = filename.split(".").take(1).collect();
    let corout= format!("{}_out.cor",prefix);
    println!("writing cor to {}", corout);
    cor.save_file(corout);
}
