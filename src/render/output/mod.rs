use crate::prelude::*;
use crate::render::fonts::ebgaramond::GLYPH_HEIGHT;
use graphics::prelude::*;
use notation_rs::prelude::*;
use std::cell::Ref;
use std::collections::BTreeMap;

pub mod elements;

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

        NRectType::TieFrom(_, _, ttype, _, _, _, _) => match ttype {
            TieFromType::LetRing => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(LightGray))),
            TieFromType::Standard => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Green))),
            TieFromType::UnresolvedInChunk => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Red))),
        },

        NRectType::TieTo(ttype) => match ttype {
            TieToType::ResolveTieFrom(_, _) => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Lime))),
            TieToType::LetRing => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Gray))),
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

// pub fn five_lines(w: f32) -> GraphicItems {
//     let mut items = GraphicItems::new();
//     for i in 0..5 {
//         let y = (i - 2) as f32 * SPACE;
//         let line = Line(0., y, w, y, Strokestyle(NOTELINES_WIDTH, Orange));
//         items.push(line);
//     }
//     items
// }

pub fn matrix_to_svg(matrix: &RMatrix, draw_dev_frames: bool) -> String {
    let mut graphic_items = GraphicItems::new();

    graphic_items.extend(output_notelines(matrix));
    graphic_items.extend(output_main_elements(matrix, draw_dev_frames));
    graphic_items.extend(output_beamgroups(matrix));
    graphic_items.extend(output_ties(matrix));

    let svg = SvgBuilder::new().build(graphic_items).unwrap();
    // std::fs::write(svg_filename, svg).unwrap();
    svg
}

fn output_ties(matrix: &RMatrix) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();

    for row in matrix.rows.iter() {
        let row = row.borrow();

        let mut map_rect: BTreeMap<(usize, i8), Rc<RefCell<NRectExt>>> = BTreeMap::new();
        let mut map_ritem: BTreeMap<(usize, i8), Rc<RefCell<RItem>>> = BTreeMap::new();

        for item in &row.items {
            if let Some(item) = item {
                let item_: Ref<RItem> = item.borrow();

                // Store ties_from in map...
                if let Some(nrects) = &item_.nrects {
                    let ties_from = nrects.iter().filter(|nrect| nrect.borrow().is_tie_from()).collect::<Vec<_>>();
                    for tie_from in ties_from {
                        let tie: Ref<NRectExt> = tie_from.borrow();
                        match &tie.1 {
                            NRectType::TieFrom(note_id, level, ttype, _, _, _, _) => match ttype {
                                TieFromType::Standard => {
                                    map_rect.insert((*note_id, *level), tie_from.clone());
                                    map_ritem.insert((*note_id, *level), item.clone());
                                }
                                TieFromType::LetRing => todo!(),
                                TieFromType::UnresolvedInChunk => todo!(),
                            },
                            _ => {}
                        }
                    }

                    let ties_to = nrects.iter().filter(|nrect| nrect.borrow().is_tie_to()).collect::<Vec<_>>();
                    dbg!(&ties_to);

                    for tie_to in ties_to {
                        let tie: Ref<NRectExt> = tie_to.borrow();
                        match &tie.1 {
                            NRectType::TieTo(ttype) => match ttype {
                                TieToType::ResolveTieFrom(from_note_id, level) => {
                                    let key: (usize, i8) = (*from_note_id, *level);

                                    let from_rect: Ref<NRectExt> = map_rect.get(&key).unwrap().borrow();
                                    let from_ritem: Ref<RItem> = map_ritem.get(&key).unwrap().borrow();

                                    let (from_type, from_duration, from_note_direction, from_tie_direction, from_placement) = match &from_rect.1 {
                                        NRectType::TieFrom(note_id, level, ttype, from_duration, from_note_direction, from_tie_direction, placement) => {
                                            (ttype, from_duration, from_note_direction, from_tie_direction, placement)
                                        }
                                        _ => todo!(),
                                    };
                                    dbg!(from_rect.0);
                                    dbg!(from_ritem.coords.unwrap());
                                    dbg!(from_note_direction, from_tie_direction, from_placement);

                                    let from_item_coords = from_ritem.coords.unwrap();
                                    let mut from_x = from_item_coords.0 + from_rect.0 .0;
                                    let mut from_y = from_item_coords.1 + from_rect.0 .1;
                                    let to_item_coords = item_.coords.unwrap();
                                    let mut to_x = to_item_coords.0 + tie.0 .0 + tie.0 .2;
                                    let mut to_y = to_item_coords.1 + tie.0 .1;

                                    // vertical placement
                                    match from_placement {
                                        TiePlacement::Top => {}
                                        TiePlacement::Mid => {
                                            from_y = from_y + SPACE_HALF;
                                            to_y = to_y + SPACE_HALF;
                                        }
                                        TiePlacement::Bottom => {
                                            from_y = from_y + SPACE;
                                            to_y = to_y + SPACE;
                                        }
                                    }

                                    // horizontal placement
                                    match from_note_direction {
                                        DirUD::Up => match from_placement {
                                            TiePlacement::Top => {
                                                // if duration_has_stem(from_duration) {
                                                from_x += SPACE_HALF;
                                                // }
                                            }
                                            TiePlacement::Mid => {
                                                from_x += SPACE_HALF;
                                                to_x -= SPACE_HALF;
                                            }
                                            TiePlacement::Bottom => {}
                                        },
                                        DirUD::Down => match from_placement {
                                            TiePlacement::Top => {}
                                            TiePlacement::Mid => {
                                                to_x -= SPACE_HALF;
                                                from_x += SPACE_HALF;
                                            }
                                            TiePlacement::Bottom => {
                                                to_x -= SPACE_HALF;
                                            }
                                        },
                                    }

                                    let rect = NRect::new(-5., -5., 10., 10.);
                                    graphic_items.push(nrectext2graphic(&NRectExt::new(rect, NRectType::Dev(true, "green".to_string())), from_x, from_y).unwrap());

                                    let rect = NRect::new(-5., -5., 10., 10.);
                                    graphic_items.push(nrectext2graphic(&NRectExt::new(rect, NRectType::Dev(true, "lime".to_string())), to_x, to_y).unwrap());

                                    let length = to_x - from_x;
                                    let max_seglength = length / 3.0;
                                    let mut from_x2 = from_x + SPACE.min(max_seglength);
                                    let mut from_y2 = from_y;
                                    let mut to_x2 = to_x - SPACE.min(max_seglength);
                                    let mut to_y2 = to_y;

                                    match from_tie_direction {
                                        DirUD::Down => {
                                            from_y2 = from_y2 + SPACE_HALF;
                                            to_y2 = to_y2 + SPACE_HALF;
                                        }
                                        DirUD::Up => {
                                            from_y2 = from_y2 - SPACE_HALF;
                                            to_y2 = to_y2 - SPACE_HALF;
                                        }
                                    }

                                    let rect = NRect::new(-5., -5., 10., 10.);
                                    graphic_items.push(nrectext2graphic(&NRectExt::new(rect, NRectType::Dev(true, "green".to_string())), from_x2, from_y2).unwrap());
                                    let rect = NRect::new(-5., -5., 10., 10.);
                                    graphic_items.push(nrectext2graphic(&NRectExt::new(rect, NRectType::Dev(true, "lime".to_string())), to_x2, to_y2).unwrap());
                                }
                                TieToType::LetRing => todo!(),
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    graphic_items
}

fn output_notelines(matrix: &RMatrix) -> GraphicItems {
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

    graphic_items
}

fn output_beamgroups(matrix: &RMatrix) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();
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
                        if duration_has_stem(&data.duration) {
                            graphic_items.extend(do_single(data, item.coords.unwrap()));
                        }
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
                        if duration_has_stem(&data.duration) {
                            graphic_items.extend(do_single(data, item.coords.unwrap()));
                        }
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

    graphic_items
}

fn output_main_elements(matrix: &RMatrix, draw_dev_frames: bool) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();
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
    graphic_items
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
