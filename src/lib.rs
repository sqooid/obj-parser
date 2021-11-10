use nalgebra_glm as glm;
use std::{fs, path};

#[derive(Debug)]
pub struct Vertex {
    pos: glm::Vec3,
    color: glm::U8Vec4,
    normal: glm::Vec3,
}

#[derive(Debug)]
pub enum Face {
    Triangle(Vec<Vertex>),
    Quad(Vec<Vertex>),
    Ngon(Vec<Vertex>),
}

pub fn read_from_file(filepath: impl AsRef<path::Path> + Copy) -> Option<Vec<Face>> {
    let contents = match fs::read_to_string(filepath) {
        Ok(x) => x,
        Err(_) => return None,
    };
    let mut img: image::RgbaImage = image::RgbaImage::new(0, 0);
    let mut colors: Vec<glm::U8Vec4> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();
    let mut normals: Vec<glm::Vec3> = Vec::new();
    let mut positions: Vec<glm::Vec3> = Vec::new();

    for line in contents.lines() {
        let mut token_it = line.split_ascii_whitespace();
        match token_it.next() {
            None => continue,
            Some(header) => match header {
                "mtllib" => {
                    let mut parent_path = path::PathBuf::new();
                    parent_path.push(filepath);
                    parent_path.pop();
                    parent_path.push(token_it.next()?);
                    let mtl_contents = fs::read_to_string(&parent_path).unwrap();
                    parent_path.pop();
                    for mtl_line in mtl_contents.lines() {
                        let mut mtl_tokens = mtl_line.split_ascii_whitespace();
                        match mtl_tokens.next() {
                            None => continue,
                            Some(mtl_header) => match mtl_header {
                                "map_Kd" => {
                                    parent_path.push(mtl_tokens.next()?);
                                    img = image::open(&parent_path).unwrap().into_rgba8()
                                }
                                _ => (),
                            },
                        }
                    }
                }
                "vn" => normals.push(glm::vec3(
                    token_it.next()?.parse().unwrap(),
                    token_it.next()?.parse().unwrap(),
                    token_it.next()?.parse().unwrap(),
                )),
                "vt" => {
                    let (x, y): (f32, f32) = (
                        token_it.next()?.parse().unwrap(),
                        token_it.next()?.parse().unwrap(),
                    );
                    let (width, height) = img.dimensions();
                    let pixel =
                        img.get_pixel((x * width as f32) as u32, (y * height as f32) as u32);
                    colors.push(glm::U8Vec4::new(pixel[0], pixel[1], pixel[2], pixel[3]));
                }
                "v" => positions.push(glm::vec3(
                    token_it.next()?.parse().unwrap(),
                    token_it.next()?.parse().unwrap(),
                    token_it.next()?.parse().unwrap(),
                )),
                "f" => {
                    let mut vertices: Vec<Vertex> = Vec::new();
                    while let Some(vert) = token_it.next() {
                        let indices: Vec<usize> =
                            vert.split("/").map(|u| u.parse().unwrap()).collect();
                        vertices.push(Vertex {
                            pos: positions[indices[0] - 1].clone(),
                            color: colors[indices[1] - 1].clone(),
                            normal: normals[indices[2] - 1].clone(),
                        });
                    }
                    match vertices.len() {
                        3 => faces.push(Face::Triangle(vertices)),
                        4 => faces.push(Face::Quad(vertices)),
                        _ => faces.push(Face::Ngon(vertices)),
                    }
                }
                _ => (),
            },
        }
    }
    Some(faces)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mesh = crate::read_from_file("assets/magica-export.obj");
        println!("{:?}", mesh);
    }
}
