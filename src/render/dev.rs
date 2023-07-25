use std::cell::Ref;

use graphics::prelude::*;
use notation_rs::prelude::*;

use crate::render::fonts::ebgaramond::GLYPH_HEIGHT;

pub fn nrect2rect(n: NRect, s: Stroke, f: graphics::item::Fill) -> GraphicItem {
    Rect(n.0, n.1, n.2, n.3, s, f)
}

pub fn next2graphic(n: &NRectExt, move_x: f32, move_y: f32) -> Option<GraphicItem> {
    let r = n.0.move_rect(move_x, move_y);
    match &n.1 {
        NRectType::Head(head_type, head_shape) => {
            //
            let p = match head_shape {
                HeadShape::BlackHead => CADENZA_148.to_vec(),
                HeadShape::WhiteHead => CADENZA_153.to_vec(),
                HeadShape::WholeHead => CADENZA_83.to_vec(),
            };
            Some(Path(PathSegments(p).inv01().move_path(r.0, SPACE_HALF + r.1), NoStroke, Fillstyle(Black)))
        }
        NRectType::Dotted(dots_nr) => {
            let p = CADENZA_DOT.to_vec();
            Some(Path(PathSegments(p).inv01().move_path(r.0 + SPACE_QUARTER, r.1 + SPACE_QUARTER), NoStroke, Fillstyle(Black)))
        }

        NRectType::Pause(pause_type) => {
            //
            let p = match pause_type {
                PauseShape::Whole => CADENZA_122.to_vec(),
                PauseShape::Half => CADENZA_172.to_vec(),
                PauseShape::Quarter => CADENZA_147.to_vec(),
                PauseShape::Eighth => CADENZA_165.to_vec(),
                PauseShape::Sixteenth => CADENZA_176.to_vec(),
                PauseShape::ThirtySecond => CADENZA_3.to_vec(),
            };
            let y: f32 = match pause_type {
                PauseShape::Whole => SPACE_HALF,
                PauseShape::Half => SPACE,
                PauseShape::Quarter => 3. * SPACE_HALF,
                PauseShape::Eighth => SPACE,
                PauseShape::Sixteenth => SPACE,
                PauseShape::ThirtySecond => 0.,
            };
            Some(Path(PathSegments(p).inv01().move_path(r.0, r.1 + y), NoStroke, Fillstyle(Black)))
        }
        NRectType::Clef(_) => {
            //
            Some(Path(PathSegments(CADENZA_8.to_vec()).inv01(), NoStroke, Fillstyle(Black)))
        }
        NRectType::Accidental(accidental) => {
            let p = match accidental {
                Accidental::Sharp => CADENZA_ACCIDENTAL_SHARP.to_vec(),
                Accidental::Flat => CADENZA_ACCIDENTAL_FLAT.to_vec(),
                Accidental::Natural => CADENZA_ACCIDENTAL_NATURAL.to_vec(),
                Accidental::DblSharp => CADENZA_ACCIDENTAL_DOUBLESHARP.to_vec(),
                Accidental::DblFlat => CADENZA_ACCIDENTAL_DOUBLEFLAT.to_vec(),
                // _ => CADENZA_ACCIDENTAL_FLAT.to_vec(),
            };
            let y = match accidental {
                // Accidental::Sharp => SPACE * 1.5,
                Accidental::Flat => SPACE * 2.0,
                // Accidental::Natural => 0.,
                // Accidental::DblSharp => 0.,
                // Accidental::DblFlat => 0.,
                _ => SPACE * 1.5,
            };
            //
            Some(Path(PathSegments(p).inv01().move_path(r.0, r.1 + y), NoStroke, Fillstyle(Black)))
        }
        NRectType::WIP(msg) => {
            //
            println!("WIP:{}", msg);
            Some(Path(PathSegments(CADENZA_3.to_vec()).inv01(), NoStroke, Fillstyle(Black)))
        }
        NRectType::DevStem => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Black))),
        NRectType::Tie(tie) => None, // Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Black)),
        NRectType::LyricChar(c) => {
            //
            fn get_glyph_path(p: Vec<PathSegment>, rect: &NRect) -> Option<GraphicItem> {
                Some(Path(
                    PathSegments(p)
                        .scale_path(FONT_SCALE_LYRICS, -FONT_SCALE_LYRICS)
                        .move_path(rect.0, rect.1 + GLYPH_HEIGHT * FONT_SCALE_LYRICS),
                    NoStroke,
                    Fillstyle(Black),
                ))
            }

            match c {
                'a' => get_glyph_path(EBGARAMOND_LOWER_A.to_vec(), &r),
                'b' => get_glyph_path(EBGARAMOND_LOWER_B.to_vec(), &r),
                'c' => get_glyph_path(EBGARAMOND_LOWER_C.to_vec(), &r),
                'A' => get_glyph_path(EBGARAMOND_UPPER_A.to_vec(), &r),
                'B' => get_glyph_path(EBGARAMOND_UPPER_B.to_vec(), &r),
                'C' => get_glyph_path(EBGARAMOND_UPPER_C.to_vec(), &r),
                _ => None,
            }
        }
        NRectType::Dev(ellipse, color) => {
            let color = Color::from_str(color);
            if *ellipse {
                Some(Ellipse(r.0, r.1, r.2, r.3, Strokestyle(1.0, color), NoFill))
            } else {
                Some(Rect(r.0, r.1, r.2, r.3, Strokestyle(1.0, color), NoFill))
            }
        }

        NRectType::DUMMY => None,
    }
}

pub fn five_lines(w: f32) -> GraphicItems {
    let mut items = GraphicItems::new();
    for i in 0..5 {
        let y = (i - 2) as f32 * SPACE;
        let line = Line(0., y, w, y, Strokestyle(NOTELINES_WIDTH, Orange));
        items.push(line);
    }
    items
}

pub fn matrix_to_svg(matrix: &RMatrix, svg_filename: &str) {
    let mut items = GraphicItems::new();
    for col in matrix.cols.iter() {
        let col = col.borrow();
        let mut rowidx = 0;
        for item in &col.items {
            if let Some(item) = item {
                let item: Ref<RItem> = item.borrow();
                let coords = item.coords.expect("RItem coords should always be calculated!");

                let nrects = item.nrects.as_ref().unwrap();
                for nrect in nrects.iter() {
                    let nrect = nrect.borrow();

                    let frame_rect = nrect.0.clone();
                    let color = if col.duration == 0 { "orange" } else { "blue" };
                    let frame_nrect = NRectExt::new(frame_rect, NRectType::Dev(false, color.to_string()));
                    let frame_item = next2graphic(&frame_nrect, coords.0, coords.1).unwrap();
                    items.push(frame_item);

                    let graphic_item = next2graphic(&nrect, coords.0, coords.1).unwrap();
                    items.push(graphic_item);
                }
            } else {
                let y = matrix.get_row(rowidx).unwrap().borrow().y;
                let x = col.x;
                let rect = NRect::new(0., 0., 10.0, 10.0);
                let nrect = NRectExt::new(rect, NRectType::Dev(true, "gray".to_string()));
                let graphic_item = next2graphic(&nrect, x, y).unwrap();
                items.push(graphic_item);
            }
            rowidx += 1;
        }
    }
    dbg!(matrix.width, matrix.height);
    let svg = SvgBuilder::new().build(items).unwrap();
    std::fs::write(svg_filename, svg).unwrap();
}
