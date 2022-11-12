use std::env;

// Package to parse input from the CLI
// TODO this doesn't work.

pub struct RTArguments {
  pub spp: usize,
  pub bounce_depth: usize,
  pub image_width: usize,
  pub image_height: usize
}

impl Default for RTArguments {
  fn default() -> Self {
      Self {
        spp: 64,
        bounce_depth: 32,
        image_height: 512,
        image_width: 528
      }
  }
}

// Help

#[allow(dead_code)]
fn print_help(program_name: &str) {
  let default_args = RTArguments::default();
  println!("Usage: {} [options]", program_name);
  println!("\t spp=xx \t\t trace xx rays per pixel (default={})", default_args.spp);
  println!("\t bd=xx  \t\t sets the ray bounce-depth to xx (default={})", default_args.bounce_depth);
  println!("\t res=<width>x<height>  \t\t set the render-resolution (default={}x{})", default_args.image_width, default_args.image_height);
}

// Processing

pub fn process_cli_args() -> RTArguments {
  let mut program_name: Option<String> = None;

  for arg in env::args() {
    if program_name.is_none() {
      program_name = Some(arg);
      continue;
    }
  }

  RTArguments::default()
}