#![allow(dead_code)]
use std::{fs};
use std::io::{Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Point{
    pub x: f64,
    pub y: f64
}
#[derive(Debug, Clone)]
pub enum Shape{
    None,
    Arc{radius:f64, center:Point, start:f64,end:f64},
    Circle{radius:f64, center:Point},
    Polyline{points: Vec<Point>}
}
pub struct Document{ 
    pub entities:Vec<Shape>,
}
impl Document {
    pub fn  new()->Self{
        Document{entities:Vec::new()}
    }
    pub fn print(&self){
        for i in &self.entities {
            println!("{:?}",i);
        }
    }
    pub fn load_file(path: impl AsRef<Path>) -> Document {
        let mut doc= Document::new();
        if let Ok(file) = fs::read_to_string(path) {
            let mut iter= file.lines();
            while let Some(line) = iter.next() {
                match line {
                    c if c.eq_ignore_ascii_case("CI") =>{
                        doc.entities.push(read_circle(&mut iter));
                    },
                    c if c.eq_ignore_ascii_case("ARC") =>{
                        doc.entities.push(read_arc(&mut iter));
                    },
                    c if c.eq_ignore_ascii_case("PD") =>{
                        doc.entities.push(read_polyline(&mut iter));
                    },
                    _ => ()
                };
            }
        } else { println!("COR_FILE: Error reading from input-file"); }
        doc
    }
    pub fn save_file(self, path: impl AsRef<Path>) {
        let mut lines:Vec<String> = Vec::new();
        for s in self.entities{
            match s { 
                Shape::Circle{radius, center} => {
                    lines.push(format!("CI"));
                    lines.push(format!(" {}",radius));
                    lines.push(format!(" {}",center.x));
                    lines.push(format!(" {}",center.y));
                    lines.push(format!("PU"));
                },
                Shape::Arc{radius, center, start, end} => {
                    lines.push(format!("ARC"));
                    lines.push(format!(" {}",center.x));
                    lines.push(format!(" {}",center.y));
                    lines.push(format!(" {}",radius));
                    lines.push(format!(" {}",start));
                    lines.push(format!(" {}",end));
                    lines.push(format!("PU"));
                },
                Shape::Polyline{points} => {
                    lines.push(format!("PD"));
                    for p in points {
                        lines.push(format!(" {}",p.x));
                        lines.push(format!(" {}",p.y));
                    }
                    lines.push(format!("PU"));
                },
                _ => ()
            }
        }
        let mut file = fs::File::create(path).expect("Error creating outputfile");
        file.write_all(lines.join("\r\n").as_bytes()).expect("Error writing to outputfile");
    }
}
fn read_polyline(line: &mut core::str::Lines)->Shape {
    let mut points:Vec<Point>=Vec::new();
    let mut coords:Vec<f64>=Vec::new();
    while let Some(l) = line.next(){
        match l {
            coord if coord.trim().parse::<f64>().is_ok() => 
                coords.push(coord.trim().parse().unwrap()),
            _ => break
        }
    }
    for val in coords.chunks_exact(2){
        points.push(
            Point{
                x:val[0],
                y:val[1]
            }
        )
    }
    if points.len() > 0 {
        return Shape::Polyline {points};
    }
    Shape::None
}
#[allow(unused_assignments)]
fn read_circle(line: &mut core::str::Lines)->Shape {
    let mut radius: f64 =0.0;
    let mut x: f64 =0.0;
    let mut y: f64 =0.0;
    if let Some(t) = line.next(){
        if let Ok(val) = t.trim().parse(){
            radius=val;
        }
    }
    if let Some(t) = line.next(){
        if let Ok(val) = t.trim().parse(){
            x=val;
        }
    }
    if let Some(t) = line.next(){
        if let Ok(val) = t.trim().parse(){
            y=val;
            _=line.next();
            return Shape::Circle { radius, center: Point { x, y } }
        }
    }
    Shape::None
}
#[allow(unused_assignments)]
fn read_arc(line: &mut core::str::Lines)->Shape {
    let mut radius=0.0;
    let mut x=0.0;
    let mut y=0.0;
    let mut start=0.0;
    let mut end=0.0;
    if let Some(t) = line.next(){
        if let Ok(val) = t.trim().parse(){
            x=val;
        }
    }
    if let Some(t) = line.next(){
        if let Ok(val) = t.trim().parse(){
            y=val;
        }
    }
    if let Some(t) = line.next(){
        if let Ok(val) = t.trim().parse(){
            radius=val;
        }
    }
    if let Some(t) = line.next(){
        if let Ok(val) = t.trim().parse(){
            start=val;
        }
    }
    if let Some(t) = line.next(){
        if let Ok(val) = t.trim().parse(){
            end=val;
            _=line.next();
            return Shape::Arc { radius, center: Point { x, y }, start, end }
        }
    }
    Shape::None
}
