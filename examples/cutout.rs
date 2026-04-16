use anyhow::anyhow;
use clap::Parser;
use gdstk_parse::{BoundingBoxCache, Cell, Library, Point, Polygon, PolygonRef, Rect};
use std::path::Path;

fn extract_polygons(
    cell: &Cell,
    area: Vec<f64>,
    cache: &BoundingBoxCache,
) -> anyhow::Result<Vec<Polygon>> {
    let mut polygons = Vec::new();
    let area = Rect::new(Point::new(area[0], area[1]), Point::new(area[2], area[3]));
    cell.traverse_polygons_with_overlap_strictly(
        area,
        cache,
        |points: Vec<Point>, _bbox: Rect, poly: &PolygonRef, _cell: &Cell| {
            polygons.push(Polygon::from_points(&points, poly.layer(), poly.datatype()));
            Ok(())
        },
    )?;
    Ok(polygons)
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: String,

    #[arg(short = 'o', long, default_value = "cutout.oas")]
    output: String,

    #[arg(short = 't', long, default_value_t = 0)]
    top_cell: u32,

    #[arg(short = 'A', long)]
    answer: Option<String>,

    #[arg(short = 'c', long, default_value = "false")]
    clip: bool,

    #[arg(long, value_delimiter = ',', num_args = 1)]
    area: Vec<f64>,
}

fn main() -> anyhow::Result<()> {
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
    let top_cell_index = args.top_cell as usize;
    if cells.len() <= top_cell_index {
        return Err(anyhow!("no top cell"));
    }
    let cache = lib.create_bounding_box_cache();
    let polygons = extract_polygons(&cells[top_cell_index], args.area.clone(), &cache)?;

    let mut wlib = Library::new("cutout", 1e-6, 1e-9)?;

    {
        let mut poly_cell = wlib.append_cell("polygon");
        let area4: [f64; 4] = args.area.clone().try_into().unwrap();
        let clip_area = Rect::from_array(area4);
        for poly in polygons {
            if args.clip {
                for mut clipped in poly.clip(clip_area)? {
                    clipped.set_layer(poly.layer());
                    clipped.set_datatype(poly.datatype());
                    poly_cell.append_polygon(&clipped);
                }
            } else {
                poly_cell.append_polygon(&poly);
            }
        }
    }

    wlib.write_oas(&args.output, 6)?;
    Ok(())
}
