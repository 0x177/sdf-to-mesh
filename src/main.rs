use macroquad::prelude::*;
mod mtable;
use crate::mtable::{TRI_TABLE,CORNER_INDEX_A,CORNER_INDEX_B};
use core::f32::EPSILON;

const MX: usize = 16;
const MY: usize = 16;
const MZ: usize = 16;
const DENSITY: f32 = 0.25;
const INTERPOLATE: bool = true;

fn sphere(c: Vec3,r: f32) -> f32 {
    c.length()-r
}

fn vmax(x: Vec3) -> f32 {
    return if x.x > x.y {x.x} else {if x.y > x.z {x.y} else {x.z}}
}

fn maxcw(x: Vec3, y: Vec3) -> Vec3 {
    vec3(if x.x>y.x {x.x} else {y.x},if x.y>y.y {x.y} else {y.y},if x.z>y.z {x.z} else {y.z})
}

fn mincw(x: Vec3, y: Vec3) -> Vec3 {
    vec3(if x.x<y.x {x.x} else {y.x},if x.y<y.y {x.y} else {y.y},if x.z<y.z {x.z} else {y.z})
}

fn cubed(p: Vec3,b: Vec3) -> f32 {
    let d = p.abs() - b;

    (maxcw(d,vec3(0.0,0.0,0.0))+vmax(mincw(d,vec3(0.0,0.0,0.0)))).length()
}

fn isovalue(ep: &Vec3) -> f32 {
    let sphere = sphere(vec3(3.0,3.0,3.0)-*ep,0.5);
    let cube = cubed(vec3(3.0,1.0,3.0)-*ep,vec3(0.2,0.2,0.2));
    if sphere < cube {sphere} else {cube}
    //sphere
}

fn interpolate_edges(x: Vec3,y: Vec3) -> Vec3 {
    let mut point = Vec3::new(0.0,0.0,0.0);
    let v1 = isovalue(&x);
    let v2 = isovalue(&y);

    if DENSITY - v1 < EPSILON {
	return x;
    }
    
    if DENSITY - v2 < EPSILON {
	return y;
    }

    if v1 - v2 < EPSILON {
	return x;
    }

    let mu = (DENSITY / v1) - (v2 - v1);

    point.x = x.x + mu * (y.x-x.x);
    point.y = x.y + mu * (y.y-x.y);
    point.z = x.z + mu * (y.z-x.z);

    return point;
}

#[macroquad::main("3D")]
async fn main() {
    let mut vertices: Vec<macroquad::models::Vertex> = Vec::new();
    
    for x in 0..MX {
	for y in 0..MY {
	    for z in 0..MZ {
		let p = vec3(x as f32,y as f32,z as f32);
		let corner_points = [
		    (vec3(0.0,0.0,0.0)+p)*DENSITY,
		    (vec3(1.0,0.0,0.0)+p)*DENSITY,
		    (vec3(1.0,1.0,0.0)+p)*DENSITY,
		    (vec3(0.0,1.0,0.0)+p)*DENSITY,
		    (vec3(0.0,0.0,1.0)+p)*DENSITY,
		    (vec3(1.0,0.0,1.0)+p)*DENSITY,
		    (vec3(1.0,1.0,1.0)+p)*DENSITY,
		    (vec3(0.0,1.0,1.0)+p)*DENSITY,
		];

		let mut value: i32 = 0;
		for (ind,ep) in corner_points.iter().enumerate() {
		    let d = isovalue(&ep);
		    
		    if d < 0.25 {
			value += 1 << ind;
		    }
		}
		
		let edges = TRI_TABLE[value as usize];
		for edge_index in edges {
		    if edge_index >= 0 {
			let index_a = CORNER_INDEX_A[edge_index as usize];
			let index_b = CORNER_INDEX_B[edge_index as usize];

			if INTERPOLATE {
			    let vertex = interpolate_edges(corner_points[index_a as usize],corner_points[index_b as usize]);

			    vertices.push(macroquad::models::Vertex {position: vertex,uv: vec2(1.0,1.0),color: WHITE});
			} else {
			    let vertex = (corner_points[index_a as usize]+corner_points[index_b as usize])/2.0;

			    vertices.push(macroquad::models::Vertex {position: vertex,uv: vec2(1.0,1.0),color: WHITE});
			}
		    }
		}
	    }
	}
    }

    let indices: Vec<u16> = (0..vertices.len() as u16).collect();

    println!("{} {}", vertices.len(),indices.len());
    
    use std::io::Write;
    let mut file = std::fs::File::create("foo.obj").unwrap();


    // maybe it is possible to combine these two loops
    for vertex in &vertices {
     	let st = format!("v {} {} {} \n",vertex.position.x,vertex.position.y,vertex.position.z);
	file.write_all(&st.into_bytes()).unwrap();	
    }
    
    for index in 0..vertices.len()/3 {
	let st = format!("f {} {} {} \n",indices[index*3+0]+1,indices[index*3+1]+1,indices[index*3+2]+1);
	file.write_all(&st.into_bytes()).unwrap();
    }
    
    let mesh = Mesh {
	vertices: vertices.clone(),
	indices,
	texture: None,
    };
	
    loop {
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });
	
	draw_mesh(&mesh);

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
