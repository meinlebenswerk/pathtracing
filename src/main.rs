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
use material::{ DiffuseMaterial, EmissiveMaterial, RTXMaterial };
use scene::{ Scene, RTXContext };
use camera::Camera;
use scenes::triangulate_square;

use objects::sphere::Sphere;
use objects::mesh::Mesh;

extern crate pbr;
use pbr::{ ProgressBar };
use std::{time::Duration, thread::{available_parallelism, self, JoinHandle}, sync::{Arc, Mutex}, io::Stdout};

use ppm::dump_ppm;
use prng::{ mt19937::Mt19937Prng, PRNG };

use rtcore::weekend::ray_color_weekend;

use crate::{output::filmic::tonemap_filmic, ppm::dump_ppm_raw};

#[derive(Copy, Clone)]
struct RenderConfig {
    samples_per_pixel: usize,
    bounce_depth: usize
}


fn render_tile(
    (width, height): (usize, usize),
    (tile_size_x, tile_size_y): (usize, usize),
    (tile_index_x, tile_index_y): (usize, usize),
    context: &RTXContext,
    camera: &Camera,
    config: &RenderConfig,
    rng: &mut dyn PRNG,
    progress: Arc<Mutex<ProgressBar<Stdout>>>
) -> Vec<Vector3f> {
    // Renders one tile of the image
    let mut framebuffer = vec![Vector3f::default(); tile_size_x * tile_size_y];

    for y in 0..tile_size_y {
        for x in 0..tile_size_x {
            let mut color = Vector3f::default();
            for _i in 0..config.samples_per_pixel {
                let u = ((x + tile_size_x * tile_index_x) as f32 + rng.next_f32()) / (width  as f32 - 1.0);
                let v = 1.0 - ((y + tile_size_y * tile_index_y) as f32 + rng.next_f32()) / (height as f32 - 1.0);

                let ray = camera.generate_ray(u, v);
                color += ray_color_weekend(&ray, config.bounce_depth, context, rng);
            }
            color /= config.samples_per_pixel as f32;
            
            let fb_index = x + (y * tile_size_x);
            framebuffer[fb_index] = color;
        }
        {
            progress.lock().unwrap().add((config.samples_per_pixel * tile_size_x) as u64);
        }   
    }

    framebuffer
}

fn main() -> std::io::Result<()> {
    let config = process_cli_args();

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

    let camera = Arc::new(Camera::new(
        Point3f::new(0.0, 0.0, -1.5), 
        Point3f::new(0.0, 0.0, 0.0),  
        Vector3f::new(0.0, 1.0, 0.0), 
        75.0, 
        aspect_ratio
    ));

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
    let box_white_mat: Arc<Box<(dyn RTXMaterial + Send + Sync + 'static)>> = Arc::new(Box::new(
        DiffuseMaterial::new(Vector3f::from_hex("#fff"))
    ));
    let box_green_mat: Arc<Box<(dyn RTXMaterial + Send + Sync + 'static)>> = Arc::new(Box::new(
        DiffuseMaterial::new(Vector3f::from_hex("#0f0"))
    ));
    let box_red_mat: Arc<Box<(dyn RTXMaterial + Send + Sync + 'static)>> = Arc::new(Box::new(
        DiffuseMaterial::new(Vector3f::from_hex("#f00"))
    ));
    let lightsource_mat: Arc<Box<(dyn RTXMaterial + Send + Sync + 'static)>> = Arc::new(
        Box::new(EmissiveMaterial::new(Vector3f::from_hex("#fff"), 4.0)
    ));

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
    scene.add(Arc::new(box_top_mesh));
    scene.add(Arc::new(box_bottom_mesh));
    scene.add(Arc::new(box_back_mesh));
    scene.add(Arc::new(box_left_mesh));
    scene.add(Arc::new(box_right_mesh));

    let sphere1 = Sphere::new(
        Point3f::new(0.0, 0.0, 0.0), 
        0.35, 
    &box_white_mat);
    scene.add_generic(Arc::new(sphere1));

    // Generate and use BVH
    let bvh = generate_bvh(&scene, true);
    scene.use_bvh(Some(bvh));


    let context = Arc::new(RTXContext::new(scene));

    let render_config = Arc::new(RenderConfig {
        bounce_depth,
        samples_per_pixel: spp
    });

    let render_progress = Arc::new(Mutex::new(render_progress));

    let n_threads = available_parallelism().unwrap().get();
    println!("Starting rendering on {} Threads...", n_threads);

    let thread_handles: Vec<JoinHandle<(Vec<Vector3f>, (usize, usize), (usize, usize))>> = (0..n_threads).map(| idx | {
        
        let ctx = Arc::clone(&context);
        let cam = Arc::clone(&camera);
        let cfg = Arc::clone(&render_config);
        let rpb = Arc::clone(&render_progress);
        thread::spawn(move || {
            let mut rng = Mt19937Prng::new(5489);
            return (
                render_tile(
                    (image_width, image_height),
                    (image_width / n_threads, image_height),
                    (idx, 0), 
                    ctx.as_ref(),
                    cam.as_ref(),
                    cfg.as_ref(),
                    &mut rng,
                    rpb
                ), 
                (image_width / n_threads, image_height),
                (idx, 0)
            )
        })
    }).collect();

    for handle in thread_handles {
        let ( tile_fb, (ts_x, ts_y), (tidx_x, tidx_y)) = handle.join().unwrap();

        // Copy over the data
        
        for y in 0..ts_y {
            for x in 0..ts_x {
                let tile_fb_index = x + (y * ts_x);
                let value = tile_fb[tile_fb_index];

                let fb_index = ((tidx_x * ts_x) + x) + (((tidx_y * ts_y) + y) * image_width);
                framebuffer[fb_index] = value;
            }
        }
    }

    {
        render_progress.lock().unwrap().finish_print("rendering done, saving to file...");
    }

    dump_ppm_raw(
        tonemap_filmic(&framebuffer).as_slice(), 
        (image_width, image_height), 
        "output/render_filmic.ppm"
    )?;

    
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

    drop(context);

    Ok(())
}
