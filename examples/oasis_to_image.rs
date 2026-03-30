use anyhow::{anyhow, Result};
use clap::Parser;
use gdstk_parse::{ApplyTransform, Cell, GetBoundingBox, Library, Point};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_polygon_mut;
use imageproc::drawing::Canvas;
use imageproc::point::Point as ImgPoint;
use std::collections::HashMap;
use std::path::Path;

struct BlendCanvas<'a>(&'a mut RgbaImage);
impl<'a> Canvas for BlendCanvas<'a> {
    type Pixel = Rgba<u8>;

    fn dimensions(&self) -> (u32, u32) {
        self.0.dimensions()
    }

    fn get_pixel(&self, x: u32, y: u32) -> Self::Pixel {
        *self.0.get_pixel(x, y)
    }

    fn draw_pixel(&mut self, x: u32, y: u32, color: Self::Pixel) {
        if x < self.0.width() && y < self.0.height() {
            let dst = self.0.get_pixel_mut(x, y);
            let alpha = color[3] as f32 / 255.0;
            let inv_alpha = 1.0 - alpha;
            for i in 0..3 {
                dst[i] = (color[i] as f32 * alpha + dst[i] as f32 * inv_alpha) as u8;
            }
            dst[3] = 255;
        }
    }
}

struct SimpleRng {
    state: u32,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }
    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state &= 0x7FFFFFFF;
        self.state
    }
    fn next_rgba(&mut self) -> Rgba<u8> {
        let r = (self.next() % 256) as u8;
        let g = (self.next() % 256) as u8;
        let b = (self.next() % 256) as u8;
        Rgba([r, g, b, 64])
    }
}
fn draw_polygons(cell: &Cell, width: u32, height: u32) -> RgbaImage {
    let mut rng = SimpleRng::new(42);

    let (min, max) = cell.bounding_box();
    let cell_w = max.x - min.x;
    let cell_h = max.y - min.y;
    let cell_size = cell_w.max(cell_h);

    let image_size = width.min(height);
    let center = min + (max - min) * 0.5;
    let min = center - Point::new(cell_size * 0.5, cell_size * 0.5);

    let scale_x = image_size as f64 / cell_size;
    let scale_y = image_size as f64 / cell_size;

    let mut image = RgbaImage::new(width, height);
    image.fill(255);
    let mut canvas = BlendCanvas(&mut image);
    let mut colors: HashMap<(u32, u32), Rgba<u8>> = HashMap::new();

    cell.traverse_polygons(|poly, _cell, trans| {
        let color = *colors
            .entry((poly.layer(), poly.datatype()))
            .or_insert(rng.next_rgba());
        let offs = poly.repetition_offsets();
        for t in trans {
            for off in &offs {
                let transform = t * off;
                let points: Vec<ImgPoint<i32>> = poly
                    .to_points()
                    .iter()
                    .map(|p| {
                        let p = p.apply_transform(transform);
                        let px = ((p.x - min.x) * scale_x) as i32;
                        let py = (height as i32) - ((p.y - min.y) * scale_y) as i32;
                        ImgPoint::new(px, py)
                    })
                    .collect();
                if points.len() >= 3 {
                    draw_polygon_mut(&mut canvas, &points, color);
                }
            }
        }
        true
    });
    image
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: String,

    #[arg(short = 'o', long, default_value = "output.png")]
    output: String,

    #[arg(short = 'W', long, default_value_t = 512)]
    width: u32,

    #[arg(short = 'H', long, default_value_t = 512)]
    height: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.input);
    let lib = if let Some(ext) = path.extension() {
        if ext == "oas" {
            Library::from_oas(&args.input)
        } else if ext == "gds" {
            Library::from_gds(&args.input, 0.0, 0.0)
        } else {
            return Err(anyhow!("unsupported extension {:?}", ext));
        }
    } else {
        return Err(anyhow!("no extension found"));
    }?;

    let (cells, _) = lib.top_level();
    println!("number of cells {}", cells.len());
    for cell in cells.iter() {
        println!("cell name {}", cell.name());
    }
    println!("layers");
    for i in 0..lib.count_layernames() {
        let layername = lib.layername(i);
        println!(
            "  {} -- {}:{}",
            layername.name(),
            layername.layer(),
            layername.datatype()
        );
    }
    // draw top level
    for cell in cells.iter() {
        draw_polygons(cell, args.width, args.height)
            .save(args.output)
            .expect("fail to save image");
        break;
    }
    Ok(())
}
