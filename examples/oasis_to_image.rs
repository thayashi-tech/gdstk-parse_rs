use anyhow::{anyhow, Result};
use clap::Parser;
use gdstk_parse::{
    ApplyTransform, BoundingBoxCache, Cell, GetBoundingBox, Library, Matrix3, Point, PolygonRef,
    Rect,
};
use image::{Rgba, RgbaImage};
use imageproc::drawing::Canvas;
use imageproc::drawing::{draw_line_segment_mut, draw_polygon_mut};
use imageproc::point::Point as ImgPoint;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;

use image::{codecs::png::PngDecoder, ImageDecoder};
use std::io::Cursor;
use std::process;

fn get_image_data(path: &str) -> anyhow::Result<(u32, u32, Vec<u8>)> {
    let img = std::fs::read(path)?;
    let img_bytes = img.as_slice();
    let cursor = Cursor::new(img_bytes);
    let decoder = PngDecoder::new(cursor)?;

    let (width, height) = decoder.dimensions();

    let mut buf = vec![0; decoder.total_bytes() as usize];
    decoder.read_image(&mut buf)?;

    Ok((width, height, buf))
}

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
fn draw_polygons(
    cell: &Cell,
    width: u32,
    height: u32,
    cell_bounds: bool,
    polygon_bounds: bool,
    area: Option<Vec<f64>>,
    cache: Option<BoundingBoxCache>,
) -> RgbaImage {
    let (min, max) = cell.bounding_box().min_max();
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
    let mut colors: HashMap<(u32, u32), Rgba<u8>> = HashMap::new();

    let point_to_canvas = |p: Point| {
        let px = ((p.x - min.x) * scale_x) as f32;
        let py = (height as f32) - ((p.y - min.y) * scale_y) as f32;
        (px, py)
    };
    let canvas = RefCell::new(BlendCanvas(&mut image));
    let rng = RefCell::new(SimpleRng::new(42));

    let draw_area = |area: Rect| {
        let (lb, rt) = area.min_max();
        let points = [lb, Point::new(lb.x, rt.y), rt, Point::new(rt.x, lb.y)];
        let color = rng.borrow_mut().next_rgba();
        for i in 0..4 {
            let s = points[i];
            let e = points[(i + 1) % 4];
            draw_line_segment_mut(
                &mut *canvas.borrow_mut(),
                point_to_canvas(s),
                point_to_canvas(e),
                color,
            )
        }
    };
    let mut count = 0;
    if let Some(area) = area {
        let area = Rect::new(Point::new(area[0], area[1]), Point::new(area[2], area[3]));
        draw_area(area);
        cell.traverse_polygons_with_overlap_strictly(
            area,
            &cache.unwrap(),
            |points: Vec<Point>, bbox: Rect, poly: &PolygonRef, _cell: &Cell| {
                let color = *colors
                    .entry((poly.layer(), poly.datatype()))
                    .or_insert(rng.borrow_mut().next_rgba());
                let ipoints: Vec<ImgPoint<i32>> = points
                    .into_iter()
                    .map(|p| {
                        let (px, py) = point_to_canvas(p);
                        ImgPoint::new(px as i32, py as i32)
                    })
                    .collect();
                count += 1;
                if ipoints.len() >= 3 {
                    draw_polygon_mut(&mut *canvas.borrow_mut(), &ipoints, color);
                }
                if polygon_bounds {
                    draw_area(bbox);
                }
                true
            },
        );
    } else {
        cell.traverse_polygons(|poly: &PolygonRef, _cell: &Cell, trans: &Vec<Matrix3>| {
            let color = *colors
                .entry((poly.layer(), poly.datatype()))
                .or_insert(rng.borrow_mut().next_rgba());
            let offs = poly.repetition_offsets();
            for t in trans {
                for off in &offs {
                    let transform = t * off;
                    let points: Vec<ImgPoint<i32>> = poly
                        .to_points()
                        .iter()
                        .map(|p| {
                            let p = p.apply_transform(&transform);
                            let (px, py) = point_to_canvas(p);
                            ImgPoint::new(px as i32, py as i32)
                        })
                        .collect();
                    count += 1;
                    if points.len() >= 3 {
                        draw_polygon_mut(&mut *canvas.borrow_mut(), &points, color);
                    }
                }
                if polygon_bounds {
                    let bbox = poly.bounding_box();
                    let bbox = bbox.apply_transform(t);
                    draw_area(bbox);
                }
            }
            true
        });
    }
    if cell_bounds {
        draw_area(cell.bounding_box());
    }
    println!("number of polygon is {}", count);
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

    #[arg(short = 't', long, default_value_t = 0)]
    top_cell: u32,

    #[arg(short = 'A', long)]
    answer: Option<String>,

    #[arg(long, default_value_t = false)]
    cell_bounds: bool,

    #[arg(long, default_value_t = false)]
    polygon_bounds: bool,

    #[arg(long, value_delimiter = ',', num_args = 1)]
    area: Option<Vec<f64>>,
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
        println!("top cell name {}", cell.name());
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
    let top_cell_index = args.top_cell as usize;
    if cells.len() <= top_cell_index {
        return Err(anyhow!("no top cell"));
    }
    let cache = if args.area.is_some() {
        Some(lib.create_bounding_box_cache())
    } else {
        None
    };
    draw_polygons(
        &cells[top_cell_index],
        args.width,
        args.height,
        args.cell_bounds,
        args.polygon_bounds,
        args.area,
        cache,
    )
    .save(args.output.clone())?;

    if let Some(answer) = args.answer {
        let (w1, h1, data1) = get_image_data(&args.output)?;
        let (w2, h2, data2) = get_image_data(&answer)?;

        if w1 != w2 || h1 != h2 {
            anyhow::bail!("Image dimensions do not match");
        }

        let mut diff = vec![0; (w1 * h1 * 4) as usize];
        let options = pixelmatch::Options {
            threshold: 0.1,
            ..Default::default()
        };

        let num_diff_pixels = pixelmatch::pixelmatch(
            data1.as_slice(),
            data2.as_slice(),
            Some(&mut diff),
            None,
            None,
            Some(options),
        );
        std::fs::write("diff.png", diff)?;
        if num_diff_pixels.unwrap() > 0 {
            process::exit(66);
        }
    }
    Ok(())
}
