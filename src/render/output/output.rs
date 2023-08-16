use crate::prelude::*;
use graphics::builder::{BuilderOptions, SizeUnit};
use graphics::prelude::*;
use notation_rs::prelude::*;
use std::cell::Ref;
use std::collections::BTreeMap;

pub fn matrix_to_svg(matrix: &RMatrix, draw_dev_frames: bool, scaling: Option<f32>) -> String {
    let mut graphic_items = GraphicItems::new();
    graphic_items.extend(output_notelines(matrix));
    graphic_items.extend(output_main_elements(matrix, draw_dev_frames));
    graphic_items.extend(output_beamgroups(matrix));
    graphic_items.extend(output_ties(matrix));

    let scaling = scaling.unwrap_or(0.03);

    let svg = SvgBuilder::new()
        .build(
            graphic_items,
            Some(BuilderOptions {
                size_unit: SizeUnit::Rem,
                size_scaling: scaling,
            }),
        )
        .unwrap();
    // std::fs::write(svg_filename, svg).unwrap();
    svg
}

pub fn bezieer(anchor1: NPoint, control1: NPoint, control2: NPoint, anchor2: NPoint, segments: u8) -> Vec<NPoint> {
    let mut coords: Vec<NPoint> = vec![];

    // coords.push(anchor1);
    let mut posx: f32;
    let mut posy: f32;
    for i in 0..segments {
        let u = i as f32 / segments as f32;

        posx =
            u.powf(3.0) * (anchor2.0 + 3.0 * (control1.0 - control2.0) - anchor1.0) + 3.0 * u.powf(2.0) * (anchor1.0 - 2.0 * control1.0 + control2.0) + 3.0 * u * (control1.0 - anchor1.0) + anchor1.0;

        posy =
            u.powf(3.0) * (anchor2.1 + 3.0 * (control1.1 - control2.1) - anchor1.1) + 3.0 * u.powf(2.0) * (anchor1.1 - 2.0 * control1.1 + control2.1) + 3.0 * u * (control1.1 - anchor1.1) + anchor1.1;

        coords.push(NPoint(posx, posy));
    }
    coords.push(anchor2);
    coords
}
