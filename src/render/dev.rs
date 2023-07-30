use graphics::prelude::*;
use notation_rs::prelude::*;
use std::cell::Ref;

use crate::render::fonts::ebgaramond::GLYPH_HEIGHT;

pub fn nrect2graphic(n: NRect, s: Stroke, f: graphics::item::Fill) -> GraphicItem {
    Rect(n.0, n.1, n.2, n.3, s, f)
}

pub fn nrectext2graphic(n: &NRectExt, move_x: f32, move_y: f32) -> Option<GraphicItem> {
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

        NRectType::Flag(beamtype, direction) => {
            match direction {
                DirUD::Up => match beamtype {
                    BeamType::B8 => Some(Path(PathSegments(CADENZA_44.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1), NoStroke, Fillstyle(Black))),
                    BeamType::B16 => Some(Path(PathSegments(CADENZA_139.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1), NoStroke, Fillstyle(Black))), // 139, 141
                    _ => None,
                    // B32 => 32,
                    // B64 => 34,
                },
                DirUD::Down => match beamtype {
                    BeamType::B8 => Some(Path(
                        PathSegments(CADENZA_43.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1 + SPACE * 3.0),
                        NoStroke,
                        Fillstyle(Black),
                    )),
                    BeamType::B16 => Some(Path(
                        PathSegments(CADENZA_142.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1 + SPACE * 3.0),
                        NoStroke,
                        Fillstyle(Black),
                    )), // 139, 141
                    _ => None,
                },
            }
        }

        NRectType::Clef(clef) => match clef {
            Clef::G => Some(Path(PathSegments(CADENZA_8.to_vec()).inv01().move_path(r.0, r.1 + 4.6 * SPACE), NoStroke, Fillstyle(Black))),
            Clef::F => Some(Path(PathSegments(CADENZA_33.to_vec()).inv01().move_path(r.0, r.1 + SPACE), NoStroke, Fillstyle(Black))),
            Clef::C => Some(Path(PathSegments(CADENZA_36.to_vec()).inv01().move_path(r.0, r.1 + 2.0 * SPACE), NoStroke, Fillstyle(Black))),
        },
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
                Accidental::Flat => SPACE * 2.0,
                _ => SPACE * 1.5,
            };
            //
            Some(Path(PathSegments(p).inv01().move_path(r.0, r.1 + y), NoStroke, Fillstyle(Black)))
        }

        NRectType::WIP(msg) => {
            //
            // println!("WIP:{}", msg);
            None //Some(Path(PathSegments(CADENZA_3.to_vec()).inv01(), NoStroke, Fillstyle(Black)))
        }

        NRectType::DevStem(color) => {
            let color = Color::from_str(color);
            Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(color)))
        }

        NRectType::Tie(tie, direction, placement) => {
            // dbg!(tie, direction, placement);
            match tie {
                Tie::LetRing => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(LightGray))),
                Tie::Standard => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Green))),
                Tie::UnresolvedInChunk => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Red))),
            }
        }

        NRectType::TieTo(tie_to) => match tie_to {
            TieTo::ResolveTieFrom => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Lime))),
            TieTo::LetRing => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Gray))),
        },

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

        NRectType::Spacer(s) => None,
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

pub fn matrix_to_svg(matrix: &RMatrix, draw_dev_frames: bool) -> String {
    let mut graphic_items = GraphicItems::new();

    if let Some(bartemplate) = &matrix.bartemplate {
        for (rowidx, row) in matrix.rows.iter().enumerate() {
            let template = bartemplate.0[rowidx];
            match template {
                PartTemplate::Music => {
                    let row = row.borrow();
                    for i in -2..3 {
                        let y = row.y + (i as f32) * SPACE;
                        graphic_items.push(Line(0., y, matrix.width, y, Strokestyle(NOTELINES_WIDTH, Black)));
                    }
                }
                _ => {}
            }
        }
    }

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

                    let color = "orange";
                    let frame_rect = nrect.0.clone();
                    let color = if col.duration == 0 { "orange" } else { "red" };

                    if col.duration == 0 || draw_dev_frames {
                        let frame_nrect = NRectExt::new(frame_rect, NRectType::Dev(false, color.to_string()));
                        let frame_item = nrectext2graphic(&frame_nrect, coords.0, coords.1).unwrap();
                        graphic_items.push(frame_item);
                    }

                    // glyph rect
                    if let Some(graphic_item) = nrectext2graphic(&nrect, coords.0, coords.1) {
                        graphic_items.push(graphic_item);
                    }
                    // let graphic_item = next2graphic(&nrect, coords.0, coords.1).unwrap();
                }
            } else {
                let y = matrix.get_row(rowidx).unwrap().borrow().y;
                let x = col.x;
                let rect = NRect::new(0., 0., 10.0, 10.0);
                let nrect = NRectExt::new(rect, NRectType::Dev(true, "gray".to_string()));
                let graphic_item = nrectext2graphic(&nrect, x, y).unwrap();
                graphic_items.push(graphic_item);
            }
            rowidx += 1;
        }
    }

    for row in matrix.rows.iter() {
        let row = row.borrow();

        let mut note_beam_start: (f32, f32, f32) = (0., 0., 0.);
        let mut note_beam_end: (f32, f32, f32) = (0., 0., 0.);
        let mut note_middles_data: Vec<(f32, RItemBeamData)> = vec![];

        let mut note2_beam_start: (f32, f32, f32) = (0., 0., 0.);
        let mut note2_beam_end: (f32, f32, f32) = (0., 0., 0.);
        let mut note2_middles_data: Vec<(f32, RItemBeamData)> = vec![];

        let mut notedata: Vec<(RItemBeamData, NPoint)> = vec![];
        let mut note2data: Vec<(RItemBeamData, NPoint)> = vec![];

        for item in &row.items {
            if let Some(item) = item {
                let item: Ref<RItem> = item.borrow();
                // upper beams
                let NPoint(item_x, item_y) = item.coords.expect("RItem coords should always be calculated!");
                //------------------------------------------------------------------
                match &item.note_beam {
                    RItemBeam::Single(ref data) => {
                        graphic_items.extend(do_single(data, item.coords.unwrap()));
                    }
                    RItemBeam::Start(data) => {
                        notedata = vec![];
                        notedata.push((data.clone(), item.coords.unwrap()));
                    }
                    RItemBeam::Middle(data) => {
                        notedata.push((data.clone(), item.coords.unwrap()));
                    }
                    RItemBeam::End(data) => {
                        notedata.push((data.clone(), item.coords.unwrap()));
                        graphic_items.extend(do_beam(&notedata));
                    }
                    _ => {}
                }
                match &item.note2_beam {
                    RItemBeam::Single(data) => {
                        graphic_items.extend(do_single(data, item.coords.unwrap()));
                    }
                    RItemBeam::Start(data) => {
                        note2data = vec![];
                        note2data.push((data.clone(), item.coords.unwrap()));
                    }
                    RItemBeam::Middle(data) => {
                        note2data.push((data.clone(), item.coords.unwrap()));
                    }
                    RItemBeam::End(data) => {
                        note2data.push((data.clone(), item.coords.unwrap()));
                        graphic_items.extend(do_beam(&note2data));
                    }
                    _ => {}
                }
            }
        }
    }

    dbg!(matrix.width, matrix.height);
    let svg = SvgBuilder::new().build(graphic_items).unwrap();
    // std::fs::write(svg_filename, svg).unwrap();
    svg
}

// fn get_head_x_adjustment(data: &RItemBeamData) -> f32 {
//     let adjustment_x: f32 = if let Some(adjustment_x) = data.adjustment_x {
//         match adjustment_x {
//             ComplexXAdjustment::UpperRight(upper_right) => upper_right,
//             _ => 0.0,
//         }
//     } else {
//         0.0
//     };
//     let head_x = match data.direction {
//         DirUD::Down => 0.0 + STEM_WIDTH / 2.0,
//         DirUD::Up => data.head_width - STEM_WIDTH / 2.0,
//     };

//     adjustment_x + head_x
// }

fn do_single(first_data: &RItemBeamData, coords: NPoint) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();

    let direction_sign = first_data.direction.sign();
    let first_coords = (*&coords.0 + get_head_x_adjustment(first_data), *&coords.1);
    let first_tip_y = first_coords.1 + (first_data.tip_level * SPACE_HALF) + (STEM_LENGTH * SPACE_HALF) * direction_sign;
    let first_bop_y = first_coords.1
        + match first_data.direction {
            DirUD::Up => first_data.bottom_level as f32 * SPACE_HALF - STEM_HEAD_CORRECTION,
            DirUD::Down => first_data.top_level as f32 * SPACE_HALF + STEM_HEAD_CORRECTION,
        };
    let (rect_y, rect_y2, rect_h) = match first_data.direction {
        DirUD::Down => (first_bop_y, first_tip_y, first_tip_y - first_bop_y),
        DirUD::Up => (first_tip_y, first_bop_y, first_bop_y - first_tip_y),
    };

    graphic_items.push(Line(first_coords.0, rect_y, first_coords.0, rect_y2, Strokestyle(STEM_WIDTH, Black)));

    graphic_items
}

fn do_beam(items: &Vec<(RItemBeamData, NPoint)>) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();

    match items.len() {
        0 => {
            panic!("do_beams: items.len() == 0");
        }
        1 => {
            println!("Single beam item");
        }
        _ => {
            // println!("Multiple beam item - items.len():{}", items.len());
            let last_idx = items.len() - 1;

            // first
            let first_data: &RItemBeamData = &items[0].0;
            let direction_sign = first_data.direction.sign();
            let first_coords = (*&items[0].1 .0 + get_head_x_adjustment(first_data), *&items[0].1 .1);
            let first_tip_y = first_coords.1 + (first_data.tip_level * SPACE_HALF) + (STEM_LENGTH * SPACE_HALF) * direction_sign;
            let first_bop_y = first_coords.1
                + match first_data.direction {
                    DirUD::Up => first_data.bottom_level as f32 * SPACE_HALF - STEM_HEAD_CORRECTION,
                    DirUD::Down => first_data.top_level as f32 * SPACE_HALF + STEM_HEAD_CORRECTION,
                };
            let (rect_y, rect_y2, rect_h) = match first_data.direction {
                DirUD::Up => (first_tip_y, first_bop_y, first_bop_y - first_tip_y),
                DirUD::Down => (first_bop_y, first_tip_y, first_tip_y - first_bop_y),
            };
            graphic_items.push(Line(first_coords.0, rect_y, first_coords.0, rect_y2, Strokestyle(STEM_WIDTH, Black)));

            // last
            let last_data = &items[last_idx].0;
            let last_coords = (*&items[last_idx].1 .0 + get_head_x_adjustment(last_data), *&items[last_idx].1 .1);
            let last_tip_y = last_coords.1 + (last_data.tip_level * SPACE_HALF) + (STEM_LENGTH * SPACE_HALF) * direction_sign;
            let last_bop_y = last_coords.1
                + match last_data.direction {
                    DirUD::Up => last_data.bottom_level as f32 * SPACE_HALF - STEM_HEAD_CORRECTION,
                    DirUD::Down => last_data.top_level as f32 * SPACE_HALF + STEM_HEAD_CORRECTION,
                };
            let (rect_y, rect_y2, rect_h) = match last_data.direction {
                DirUD::Up => (last_tip_y, last_bop_y, last_bop_y - last_tip_y),
                DirUD::Down => (last_bop_y, last_tip_y, last_tip_y - last_bop_y),
            };

            graphic_items.push(Line(last_coords.0, rect_y, last_coords.0, rect_y2, Strokestyle(STEM_WIDTH, Black)));

            //================================================================
            let beam_width = last_coords.0 - first_coords.0;
            let beam_height = last_tip_y - first_tip_y;

            // middles
            let mut tip_coords: Vec<(f32, f32, f32)> = vec![(first_coords.0, first_tip_y, 0.)];
            if items.len() > 2 {
                let middle_items = &items[1..last_idx];
                for middle_item in middle_items {
                    let middle_data = &middle_item.0;
                    let middle_coords = (middle_item.1 .0 + get_head_x_adjustment(last_data), middle_item.1 .1);

                    // let nrect = NRectExt::new(NRect::new(-5., -5., 10., 10.), NRectType::DevStem("orange".to_string()));
                    // graphic_items.push(next2graphic(&nrect, middle_coords.0, middle_coords.1).unwrap());

                    let fraction = (middle_coords.0 - first_coords.0) / beam_width;
                    let middle_tip_y = first_tip_y + (beam_height * fraction);

                    let middle_coords = (middle_item.1 .0 + get_head_x_adjustment(last_data), middle_item.1 .1);

                    // let nrect = NRectExt::new(NRect::new(-5., -5., 10., 10.), NRectType::DevStem("purple".to_string()));
                    // graphic_items.push(next2graphic(&nrect, middle_coords.0, middle_tip_y).unwrap());

                    let middle_bop_y = middle_coords.1
                        + match middle_data.direction {
                            DirUD::Up => middle_data.bottom_level as f32 * SPACE_HALF - STEM_HEAD_CORRECTION,
                            DirUD::Down => middle_data.top_level as f32 * SPACE_HALF + STEM_HEAD_CORRECTION,
                        };

                    let (rect_y, rect_y2, rect_h) = match middle_data.direction {
                        DirUD::Down => (middle_bop_y, middle_tip_y, middle_tip_y - middle_bop_y),
                        DirUD::Up => (middle_tip_y, middle_bop_y, middle_bop_y - middle_tip_y),
                    };

                    graphic_items.push(Line(middle_coords.0, rect_y, middle_coords.0, rect_y2, Strokestyle(STEM_WIDTH, Black)));
                    tip_coords.push((middle_coords.0, middle_tip_y, fraction));
                }
            }
            tip_coords.push((last_coords.0, last_tip_y, 1.0));
            let sub_beam_graphic_items = do_sub_beams(beam_width, beam_height, &tip_coords, last_data.direction, &last_data.note_durations.as_ref().unwrap());
            graphic_items.extend(sub_beam_graphic_items);
        }
    }

    graphic_items
}

fn do_sub_beams(beam_width: f32, beam_height: f32, tip_coords: &Vec<(f32, f32, f32)>, direction: DirUD, durations: &Vec<Duration>) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();
    let lastidx = tip_coords.len() - 1;
    let beamtypes: Vec<BeamType> = durations.iter().map(|d| duration_to_beamtype(d)).collect::<Vec<BeamType>>();

    let sixteenths = tip_coords.iter().map(|(x, y, _)| (*x, *y + BEAM_SUB_DISTANCE * -direction.sign())).collect::<Vec<(f32, f32)>>();
    let sixteenths_y = tip_coords.iter().map(|(_, y, _)| *y + BEAM_SUB_DISTANCE * -direction.sign()).collect::<Vec<f32>>();
    let thirtytwos = tip_coords.iter().map(|(x, y, _)| (*x, *y + BEAM_SUB_DISTANCE * 2.0 * -direction.sign())).collect::<Vec<(f32, f32)>>();
    let thirtytwos_y = tip_coords.iter().map(|(_, y, _)| *y + BEAM_SUB_DISTANCE * 2.0 * -direction.sign()).collect::<Vec<f32>>();

    let (x, y, x2, y2) = (tip_coords[0].0 - STEM_WIDTH_HALF, tip_coords[0].1, tip_coords[lastidx].0 + STEM_WIDTH_HALF, tip_coords[lastidx].1);
    // graphic_items.push(Line(x, y, x2, y2, Strokestyle(DEV_LINE_THICKNESS, Blue)));

    let beamheight = match direction {
        DirUD::Down => -BEAM_HEIGHT,
        DirUD::Up => BEAM_HEIGHT,
    };

    graphic_items.push(Path(
        PathSegments(vec![M(x, y), L(x2, y2), L(x2, y2 + beamheight), L(x, y + beamheight)]),
        Stroke::NoStroke,
        Fillstyle(Black),
    ));

    use BeamType::*;
    match beamtypes.as_slice() {
        [B16, B16] | [B16, B16, B16] | [B16, B16, B16, B16] => {
            let (x, y, x2, y2) = (sixteenths[0].0, sixteenths[0].1, sixteenths[lastidx].0, sixteenths[lastidx].1);

            // graphic_items.push(Line(x, y, x2, y2, Strokestyle(DEV_LINE_THICKNESS, Blue)));

            let beamheight = match direction {
                DirUD::Down => -BEAM_HEIGHT,
                DirUD::Up => BEAM_HEIGHT,
            };

            graphic_items.push(Path(
                PathSegments(vec![M(x, y), L(x2, y2), L(x2, y2 + beamheight), L(x, y + beamheight)]),
                Stroke::NoStroke,
                Fillstyle(Black),
            ));
        }
        [B8, B16] | [B8, B16, B8] => graphic_items.extend(do_sub_sixteen_rightside(sixteenths[0], sixteenths[1], direction)),
        [B16, B8] => graphic_items.extend(do_sub_sixteen_leftside(sixteenths[0], sixteenths[1], direction)),
        [B16, B16, B8] | [B16, B16, B8, B8] => graphic_items.extend(do_sub_sixteen(sixteenths[0], sixteenths[1], direction)),
        [B16, B8, B16] => {
            graphic_items.extend(do_sub_sixteen_leftside(sixteenths[0], sixteenths[1], direction));
            graphic_items.extend(do_sub_sixteen_rightside(sixteenths[1], sixteenths[2], direction))
        }
        [B8, B16, B16] | [B8, B16, B16, B8] => {
            graphic_items.extend(do_sub_sixteen(sixteenths[1], sixteenths[2], direction));
        }

        _ => println!("Unhandled durastions for sub_beaming"),
    }

    graphic_items
}

fn do_sub_sixteen_rightside(left: (f32, f32), right: (f32, f32), direction: DirUD) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();
    let width = right.0 - left.0;
    let height = right.1 - left.1;
    let fraction = (width - HEAD_WIDTH_BLACK) / width;
    let tip_left_x = right.0 - HEAD_WIDTH_BLACK;
    let tip_left_y = left.1 + (fraction * height);

    let (x, y, x2, y2) = (tip_left_x - STEM_WIDTH_HALF, tip_left_y, right.0 + STEM_WIDTH_HALF, right.1);
    graphic_items.push(Line(x, y, x2, y2, Strokestyle(DEV_LINE_THICKNESS, Red)));

    let beamheight = match direction {
        DirUD::Down => -BEAM_HEIGHT,
        DirUD::Up => BEAM_HEIGHT,
    };

    graphic_items.push(Path(
        PathSegments(vec![M(x, y), L(x2, y2), L(x2, y2 + beamheight), L(x, y + beamheight)]),
        Stroke::NoStroke,
        Fillstyle(Black),
    ));

    graphic_items
}

fn do_sub_sixteen(left: (f32, f32), right: (f32, f32), direction: DirUD) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();
    let (x, y, x2, y2) = (left.0 - STEM_WIDTH_HALF, left.1, right.0 + STEM_WIDTH_HALF, right.1);
    graphic_items.push(Line(x, y, x2, y2, Strokestyle(DEV_LINE_THICKNESS, Red)));

    let beamheight = match direction {
        DirUD::Down => -BEAM_HEIGHT,
        DirUD::Up => BEAM_HEIGHT,
    };

    graphic_items.push(Path(
        PathSegments(vec![M(x, y), L(x2, y2), L(x2, y2 + beamheight), L(x, y + beamheight)]),
        Stroke::NoStroke,
        Fillstyle(Black),
    ));

    graphic_items
}

fn do_sub_sixteen_leftside(left: (f32, f32), right: (f32, f32), direction: DirUD) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();
    let width = right.0 - left.0;
    let height = right.1 - left.1;
    let fraction = (width - HEAD_WIDTH_BLACK) / width;
    dbg!(fraction);
    let tip_left_x = left.0 + HEAD_WIDTH_BLACK;
    let tip_left_y = left.1 + (fraction * height);
    let (x, y, x2, y2) = (left.0 - STEM_WIDTH_HALF, left.1, tip_left_x + STEM_WIDTH_HALF, tip_left_y);

    let beamheight = match direction {
        DirUD::Down => -BEAM_HEIGHT,
        DirUD::Up => BEAM_HEIGHT,
    };

    graphic_items.push(Path(
        PathSegments(vec![M(x, y), L(x2, y2), L(x2, y2 + beamheight), L(x, y + beamheight)]),
        Stroke::NoStroke,
        Fillstyle(Black),
    ));

    graphic_items.push(Line(x, y, x2, y2, Strokestyle(DEV_LINE_THICKNESS, Red)));
    graphic_items
}
