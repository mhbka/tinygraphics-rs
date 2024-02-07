use std::fs;

use crate::tgaimage::*;
use crate::triangle_bary::*;
use crate::types::*;
use nom::{
    bytes::complete::tag,
    character::complete::{char, multispace0, space1, digit1},
    number::complete::float,
    combinator::map_res,
    sequence::tuple,
    multi::separated_list0,
    IResult,
};

// draw the object into the tga image
pub fn draw_obj(filepath: &str, image: &mut Image<RGB>) {
    let faces = parse_obj(filepath);
    for face in faces {
        // calculate vector of 2 sides of the face
        let side_1 = Vec3Df {
            x: face.vertices[1].x - face.vertices[0].x,
            y: face.vertices[1].y - face.vertices[0].y,
            z: face.vertices[1].z - face.vertices[0].z,
        };

        let side_2 = Vec3Df {
            x: face.vertices[2].x - face.vertices[0].x,
            y: face.vertices[2].y - face.vertices[0].y,
            z: face.vertices[2].z - face.vertices[0].z,
        };

        // calculate normal of the face using the 2 sides, and normalize
        let mut normal = side_1.cross_product(&side_2);
        normal.normalize();

        // calculate weight of light (scalar product of normal + z-coordinate)
        let light = Vec3Df {x:0.0, y:0.0, z:1.0};
        let intensity = normal.scalar_product(&light);
        if intensity > 0.0 {
            let color = RGB {
                r: (255.0*intensity) as u8,
                g: (255.0*intensity) as u8,
                b: (255.0*intensity) as u8,
            };
    
            // remove z component (for now)
            let coords = face.vertices.map(|v| {
                Vec2Di { x: ((v.x+1.0)*image.width as f32 / 2.0) as i32, 
                        y: ((v.y+1.0)*image.height as f32 / 2.0) as i32
                    }
            });
    
            triangle(image, &coords, color);
        }
        
    }
}

// parse the object from file
pub fn parse_obj(filepath: &str) -> Vec<Face> { 
    let contents = fs::read_to_string(filepath)
        .expect(&format!("No filepath: {filepath}")[..]);

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    for line in contents.lines() {
        if line.starts_with("v ") {
            match parse_Vec3Df(&line) {
                Ok((_, Vec3Df)) => vertices.push(Vec3Df),
                Err(_) => continue
            }
        }

        else if line.starts_with("vt ") {
            //ignore
        }

        else if line.starts_with("vn ") {
            //ignore
        }

        else if line.starts_with("f ") {
            match parse_face(&line, &vertices) {
                Ok((_, mut returned_faces)) => faces.push(returned_faces.remove(0)),
                Err(_) => continue,
            }
        }
    }
    faces   
}

// Vec3Df parsing

fn parse_Vec3Df(input: &str) -> IResult<&str, Vec3Df> {
    let (input, _) = char('v')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, (x, _, y, _, z)) = tuple((float, space1, float, space1, float))(input)?;
    Ok((input, Vec3Df { x, y, z }))
}

// Face parsing

fn parse_face<'a>(input: &'a str, vertices: &Vec<Vec3Df>) -> IResult<&'a str, Vec<Face>> {
    let (input, _) = char('f')(input)?;
    let (input, _) = multispace0(input)?;

    let (input, v1_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v2_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, v3_vec) = separated_list0(tag("/"), map_res(digit1, str::parse::<usize>))(input)?;
    
    let mut faces = Vec::with_capacity(3);
    faces.push(Face {vertices: [vertices[v1_vec[0]-1], vertices[v2_vec[0]-1], vertices[v3_vec[0]-1]]});
    //faces.push(Face {v1: vertices[v1_vec[1]-1], v2: vertices[v2_vec[1]-1], v3: vertices[v3_vec[1]-1]});
    //faces.push(Face {v1: vertices[v1_vec[2]-1], v2: vertices[v2_vec[2]-1], v3: vertices[v3_vec[2]-1]});

    Ok((input, faces))
}