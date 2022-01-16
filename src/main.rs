extern crate exr;

mod ppm;
mod rtx_traits;
mod scene;
mod camera;
mod stl;
mod utils;
mod scenes;
mod prng;
mod bvh;
mod objects;
mod material;
mod math;

mod output;
mod rtcore;
mod geometry;

mod config;

mod cli;

use bvh::generate_bvh;
use cli::process_cli_args;
use geometry::{vector::Vector3f, point::Point3f};
use material::{ DiffuseMaterial, EmissiveMaterial };
use scene::{ Scene, RTXContext };
use camera::Camera;
use scenes::triangulate_square;

use objects::sphere::Sphere;
use objects::mesh::Mesh;

extern crate pbr;
use pbr::{ ProgressBar };
use std::time::Duration;

use ppm::dump_ppm;
use prng::{ mt19937::Mt19937Prng };

use rtcore::weekend::ray_color_weekend;

fn main() -> std::io::Result<()> {
    let config = process_cli_args();
    let mut prng = Mt19937Prng::new(5489);

    let spp = config.spp;
    let bounce_depth = config.bounce_depth;

    let image_width = config.image_width;
    let image_height = config.image_height;

    // Progress:
    let mut render_progress = ProgressBar::new((image_height * image_width * spp) as u64);
    render_progress.message("Rendering... ");
    render_progress.set_max_refresh_rate(Some(Duration::from_millis(500)));
    render_progress.show_tick = false;

    let aspect_ratio = (image_width as f32) / (image_height as f32);

    // Framebuffer
    let mut framebuffer = vec![Vector3f::default(); image_height * image_width];

    let camera = Camera::new(
        Point3f::new(0.0, 0.0, -1.5), 
        Point3f::new(0.0, 0.0, 0.0),  
        Vector3f::new(0.0, 1.0, 0.0), 
        75.0, 
        aspect_ratio
    );
    let mut scene = Scene::new();

    // Setup the world-sphere:
    // let world_mat = WorldMaterial::new(Vector3f::from_hex("#87CEEB"), Vector3f::from_hex("#005d7d"));
    // let world_mat = WorldMaterial::new(Vector3f::from_hex("#000"), Vector3f::from_hex("#000"));
    // let world_sphere = Sphere::new(Vector3f::new(0.0, 0.0, 0.0), 200.0, &world_mat);
    // scene.add(&world_sphere);

    // Setup cornell-box-scene
    // Sizing
    let box_size = 2.0;

    // Box
    let box_white_mat = DiffuseMaterial::new(Vector3f::from_hex("#fff"));
    let box_green_mat = DiffuseMaterial::new(Vector3f::from_hex("#0f0"));
    let box_red_mat = DiffuseMaterial::new(Vector3f::from_hex("#f00"));
    let lightsource_mat = EmissiveMaterial::new(Vector3f::from_hex("#fff"), 4.0);

    let xz_plane_points = vec![
        Vector3f::new(-box_size/2.0, 0.0,  box_size/2.0),
        Vector3f::new(-box_size/2.0, 0.0, -box_size/2.0),
        Vector3f::new( box_size/2.0, 0.0, -box_size/2.0),
        Vector3f::new( box_size/2.0, 0.0,  box_size/2.0),
    ];
    let yz_plane_points = vec![
        Vector3f::new(0.0,  box_size/2.0, -box_size/2.0),
        Vector3f::new(0.0,  box_size/2.0,  box_size/2.0),
        Vector3f::new(0.0, -box_size/2.0,  box_size/2.0),
        Vector3f::new(0.0, -box_size/2.0, -box_size/2.0),
    ];
    let xy_plane_points = vec![
        Vector3f::new(-box_size/2.0,  box_size/2.0,  0.0),
        Vector3f::new( box_size/2.0,  box_size/2.0,  0.0),
        Vector3f::new( box_size/2.0, -box_size/2.0,  0.0),
        Vector3f::new(-box_size/2.0, -box_size/2.0,  0.0),
    ];

    let box_top_mesh = Mesh::new(
        Point3f::new(0.0, box_size/2.0, 0.0), 
        triangulate_square(&xz_plane_points), 
        &lightsource_mat
    );
    let box_bottom_mesh = Mesh::new(
        Point3f::new(0.0, -box_size/2.0, 0.0), 
        triangulate_square(&xz_plane_points), 
        &box_white_mat
    );
    let box_back_mesh = Mesh::new(
        Point3f::new(0.0, 0.0, box_size/2.0), 
        triangulate_square(&xy_plane_points), 
        &box_white_mat
    );
    let box_left_mesh = Mesh::new(
        Point3f::new(-box_size/2.0, 0.0, 0.0), 
        triangulate_square(&yz_plane_points),
        &box_green_mat
    );
    let box_right_mesh = Mesh::new(
        Point3f::new(box_size/2.0, 0.0, 0.0), 
        triangulate_square(&yz_plane_points),
        &box_red_mat
    );

    // scene.add(&lighsource_mesh);
    scene.add(&box_top_mesh);
    scene.add(&box_bottom_mesh);
    scene.add(&box_back_mesh);
    scene.add(&box_left_mesh);
    scene.add(&box_right_mesh);

    let sphere1 = Sphere::new(
        Point3f::new(0.0, 0.0, 0.0), 
        0.35, 
        &box_white_mat);
    scene.add_generic(&sphere1);

    // Generate and use BVH
    let bvh = generate_bvh(&scene, true);
    scene.use_bvh(Some(bvh));


    let mut context = RTXContext::new(&mut prng, &scene);

    for y in 0..image_height {
        for x in 0..image_width {
            let mut color = Vector3f::default();
            for _i in 0..spp {
                let u = (x as f32 + context.rng.next_f32()) / (image_width  as f32 - 1.0);
                let v = 1.0 - (y as f32 + context.rng.next_f32()) / (image_height as f32 - 1.0);

                let ray = camera.generate_ray(u, v);
                // println!("Generated ray @ uv=({},{}): {}", u, v, ray);
                color += ray_color_weekend(&ray, bounce_depth, &mut context);
                // trace(&ray, 0, &mut context, &mut color);
                // coz::progress!();
            }
            color /= spp as f32;
            
            let fb_index = x + (y * image_width);
            framebuffer[fb_index] = color;

            render_progress.add(spp as u64);
        }
    }
    render_progress.finish_print("rendering done, saving to file...");
    dump_ppm(
        &framebuffer, 
        (image_width, image_height), 
        "output/render.ppm"
    )?;

    exr::prelude::write_rgb_file(
        "output/render.exr", 
        image_width, image_height, 
        |x, y| {
            let index = x + (y * image_width);
            let pixel = framebuffer[index];
            (pixel.x, pixel.y, pixel.z)
        }
    ).unwrap();


    Ok(())
}
