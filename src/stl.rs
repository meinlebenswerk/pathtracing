use crate::geometry::point::Point3;
use crate::geometry::vector3::Vector3;
use crate::material::RTXMaterial;
use crate::objects::mesh::Mesh;
use crate::objects::triangle::Triangle;
use std::io::{prelude::*, Error};
use std::fs::File;
use std::io::{SeekFrom, Cursor};
use byteorder::{ LittleEndian, ReadBytesExt };

fn read_point(rdr: &mut std::io::Cursor<std::vec::Vec<u8>>) -> std::io::Result<Vector3> {
  let x = rdr.read_f32::<LittleEndian>()?;
  let y = rdr.read_f32::<LittleEndian>()?;
  let z = rdr.read_f32::<LittleEndian>()?;
  Ok(Vector3::new(x, y, z))
}

pub fn load_triangles_from_stl(filename: &str) -> std::io::Result<Vec<Triangle>> {
  let mut file = File::open(filename)?;
  let mut triangles: Vec<Triangle> = Vec::new();

  // skip the first 80 bytes, since it's garbage anyways.
  file.seek(SeekFrom::Start(80))?;

  let triangle_byte_size = 48 + 2;
  let n_triangles;
  let mut n_triangles_read = 0;

  // read the amount of triangles:
  {
    let reference = &mut file;
    let mut buf = Vec::with_capacity(4);
    reference.take(4).read_to_end(&mut buf)?;
    let mut rdr = Cursor::new(buf);
    n_triangles = rdr.read_u32::<LittleEndian>()?;
  }

  loop {
    let reference = &mut file;

    let mut triangle_chunk: Vec<u8> = Vec::with_capacity(triangle_byte_size);
    let n = reference.take(triangle_byte_size as u64).read_to_end(&mut triangle_chunk)?;
    if n == 0 { break }
    if n < triangle_byte_size {
      println!("Could only read {} bytes instead of the expected {}", n, triangle_byte_size);
      break;
    }

    let mut rdr = Cursor::new(triangle_chunk);
    let _normal = read_point(&mut rdr)?;
    let p0 = read_point(&mut rdr)?;
    let p1 = read_point(&mut rdr)?;
    let p2 = read_point(&mut rdr)?;

    let triangle = Triangle::new(p0, p1, p2);
    // println!("Constructed Triangle:\t{}", triangle);
    triangles.push(triangle);
    n_triangles_read += 1;
  }

  if n_triangles_read != n_triangles {
    println!("STL reader could only read {} instead of {} triangles from {}", n_triangles_read, n_triangles, filename);
    return Err(Error::new(std::io::ErrorKind::InvalidInput, "Could not parse STL, broken header/file."));
  }
  Ok(triangles)
}

pub fn create_mesh_from_stl<'material>(filename: &'material str, center: Point3, material: &'material dyn RTXMaterial) -> std::io::Result<Mesh<'material>> {
  let triangles = load_triangles_from_stl(filename)?;
  Ok(Mesh::new(center, triangles, material))
}