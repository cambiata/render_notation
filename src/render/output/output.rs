use crate::prelude::*;
use graphics::builder::{BuilderOptions, SizeUnit};
use graphics::prelude::*;
use notation_rs::prelude::*;
use std::cell::Ref;
use std::collections::BTreeMap;

pub fn matrix_to_svg(
    matrix: &RMatrix,
    draw_dev_frames: bool,
    options: Option<BuilderOptions>,
) -> String {
    let mut graphic_items = GraphicItems::new();

    graphic_items.extend(output_ackolades(matrix));
    graphic_items.extend(output_notelines(matrix));
    graphic_items.extend(output_main_elements(matrix, draw_dev_frames));
    graphic_items.extend(output_beamgroups(matrix));
    graphic_items.extend(output_ties(matrix));
    graphic_items.extend(output_lines(matrix));

    graphic_items.extend(output_row_nrects(matrix));

    let svg = SvgBuilder::new().build(graphic_items, options).unwrap();
    // std::fs::write(svg_filename, svg).unwrap();
    svg
}

pub fn matrix_to_fuse(
    matrix: &RMatrix,
    draw_dev_frames: bool,
    options: Option<BuilderOptions>,
    fuse_name: &str,
    fuse_category: &str,
) -> String {
    let mut graphic_items = GraphicItems::new();
    graphic_items.extend(output_notelines(matrix));
    graphic_items.extend(output_main_elements(matrix, draw_dev_frames));
    graphic_items.extend(output_beamgroups(matrix));
    graphic_items.extend(output_ties(matrix));
    graphic_items.extend(output_lines(matrix));

    let scale = match options {
        Some(options) => options.size_scaling,
        None => 1.0,
    };

    graphic_items = graphic_items.scale_items(scale, -scale, scale);

    let items_bbox = &graphic_items.bbox();
    let fuse_width = items_bbox.2 + (-items_bbox.0);
    let fuse_height = items_bbox.3 + (-items_bbox.1);
    dbg!(fuse_width, fuse_height);

    let mut fuse = FuseBuilder::new().build(graphic_items, options).unwrap();
    let mut fuse = fuse.replace("@FUSE_NAME@", fuse_name);
    let mut fuse = fuse.replace("@FUSE_CATEGORY@", fuse_category);

    fuse
}

pub fn bezieer(
    anchor1: NPoint,
    control1: NPoint,
    control2: NPoint,
    anchor2: NPoint,
    segments: u8,
) -> Vec<NPoint> {
    let mut coords: Vec<NPoint> = vec![];

    // coords.push(anchor1);
    let mut posx: f32;
    let mut posy: f32;
    for i in 0..segments {
        let u = i as f32 / segments as f32;

        posx = u.powf(3.0) * (anchor2.0 + 3.0 * (control1.0 - control2.0) - anchor1.0)
            + 3.0 * u.powf(2.0) * (anchor1.0 - 2.0 * control1.0 + control2.0)
            + 3.0 * u * (control1.0 - anchor1.0)
            + anchor1.0;

        posy = u.powf(3.0) * (anchor2.1 + 3.0 * (control1.1 - control2.1) - anchor1.1)
            + 3.0 * u.powf(2.0) * (anchor1.1 - 2.0 * control1.1 + control2.1)
            + 3.0 * u * (control1.1 - anchor1.1)
            + anchor1.1;

        coords.push(NPoint(posx, posy));
    }
    coords.push(anchor2);
    coords
}
