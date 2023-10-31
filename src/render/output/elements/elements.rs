use std::{
    cell::{Ref, RefMut},
    collections::BTreeMap,
    sync::Arc,
};

use crate::{
    prelude::*,
    render::{
        fonts::opensans_regular::{OPENSANS_REGULAR_189, OPENSANS_REGULAR_49},
        output::rects2graphic::nrectext2graphic,
    },
};
use graphics::prelude::*;
use itertools::{Itertools, TupleWindows};
use notation_rs::prelude::*;

pub fn output_lines(matrix: &RMatrix) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();

    for row in matrix.rows.iter() {
        let row = row.borrow();
        for item in &row.items {}
        let items: &Vec<Option<Rc<RefCell<RItem>>>> = &row.items;
        use std::slice::Iter;
        let items_iter: Iter<_> = items.iter();
        use notation_rs::core::HEAD_WIDTH_WHITE;
        for (itemidx, (left, right)) in items_iter.tuple_windows().enumerate() {
            if left.is_some() && right.is_some() {
                let left: Ref<RItem> = left.as_ref().unwrap().borrow();
                let mut right: RefMut<RItem> = right.as_ref().unwrap().borrow_mut();

                // let left_coords = left.coords.expect("RItem coords should always be calculated!");
                let left_coord_x = left.coord_x.unwrap();
                let left_coord_y = left.coord_y.unwrap();

                // let right_coords = right.coords.expect("RItem coords should always be calculated!");
                let right_coord_x = right.coord_x.unwrap();
                let right_coord_y = right.coord_y.unwrap();

                let lines_to = &right.lines;
                for (idx, line_to) in lines_to.iter().enumerate() {
                    let rect = NRect::new(
                        left_coord_x,
                        left_coord_y,
                        right_coord_x - left_coord_x,
                        right_coord_y - left_coord_y,
                    );
                    let nrect =
                        NRectExt::new(rect, NRectType::LineTo(line_to.0, line_to.1, line_to.2));

                    let x = left_coord_x + SPACE * 1.8;
                    let x2 = right_coord_x - SPACE * 0.4;
                    let y = left_coord_y + line_to.0 as f32 * SPACE_HALF + 4.0;
                    let y2 = right_coord_y + line_to.1 as f32 * SPACE_HALF + 4.0;

                    match line_to.2 {
                        HeadLineType::Halfstep => {
                            let xmid = x + (x2 - x) / 2.0;
                            let ymid = y + (y2 - y) / 2.0;
                            let p = GraphicItem::Path(
                                PathSegments([M(x, y), L(xmid, ymid + 15.0), L(x2, y2)].to_vec()),
                                Strokestyle(5.0, Tomato),
                                NoFill,
                                PathCacheInfo::NoCache,
                            );
                            graphic_items.push(p);
                            let p1 = GraphicItem::Path(
                                PathSegments(OPENSANS_REGULAR_189.to_vec())
                                    .scale_path(0.05, 0.05)
                                    .move_path(xmid - 10.0, ymid + 56.0),
                                NoStroke,
                                Fillstyle(Black),
                                PathCacheInfo::NoCache,
                            );
                            graphic_items.push(p1);
                        }

                        HeadLineType::Wholestep => {
                            let xmid = x + (x2 - x) / 2.0;
                            let ymid = y + (y2 - y) / 2.0;
                            let p = GraphicItem::Path(
                                //  L(x + 3.0, y + 12.0), L(x2 - 3.0, y2 + 12.0),
                                PathSegments([M(x, y), L(x2, y2)].to_vec()),
                                Strokestyle(5.0, Dodgerblue),
                                NoFill,
                                PathCacheInfo::NoCache,
                            );
                            graphic_items.push(p);

                            let p1 = GraphicItem::Path(
                                PathSegments(OPENSANS_REGULAR_49.to_vec())
                                    .scale_path(0.04, 0.04)
                                    .move_path(xmid - 10.0, ymid - 30.0),
                                NoStroke,
                                Fillstyle(Black),
                                PathCacheInfo::NoCache,
                            );
                            graphic_items.push(p1);
                        }
                        HeadLineType::LineColor(ncolor) => {
                            let graphic_item: GraphicItem = GraphicItem::Line(
                                x,
                                y,
                                x2,
                                y2,
                                Strokestyle(5.0, ncolor2color(ncolor)),
                            );
                            graphic_items.push(graphic_item);
                        }
                        _ => {
                            let graphic_item: GraphicItem =
                                GraphicItem::Line(x, y, x2, y2, Strokestyle(5.0, Black));
                            graphic_items.push(graphic_item);
                        }
                    }
                }
            }
        }
    }
    graphic_items
}

pub fn ncolor2color(ncolor: NColor) -> Color {
    match ncolor {
        NColor::Black => Black,
        NColor::White => White,
        NColor::Red => Red,
        NColor::Blue => Blue,
        NColor::Dodgerblue => Dodgerblue,
        NColor::Tomato => Tomato,
        NColor::Orange => Orange,
        NColor::Purple => Purple,
        NColor::Lime => Lime,
        NColor::Gray => Gray,
        NColor::LightGray => LightGray,
        NColor::Green => Green,
    }
}

pub fn output_ties(matrix: &RMatrix) -> GraphicItems {
    // drawing of ties

    let mut graphic_items = GraphicItems::new();

    for row in matrix.rows.iter() {
        let row = row.borrow();

        let mut map_rect: BTreeMap<(usize, i8), Rc<RefCell<NRectExt>>> = BTreeMap::new();
        let mut map_ritem: BTreeMap<(usize, i8), Rc<RefCell<RItem>>> = BTreeMap::new();

        let mut itemidx = 0;
        for item in &row.items {
            if let Some(item) = item {
                let item_: Ref<RItem> = item.borrow();

                // Store ties_from in map...
                if let Some(nrects) = &item_.nrects {
                    let ties_from = nrects
                        .iter()
                        .filter(|nrect| nrect.borrow().is_tie_from())
                        .collect::<Vec<_>>();
                    for tie_from in ties_from {
                        let tie: Ref<NRectExt> = tie_from.borrow();
                        match &tie.1 {
                            NRectType::TieFrom(id1, level, ttype, _, _, _, _) => match ttype {
                                TieFromType::Standard => {
                                    map_rect.insert((*id1, *level), tie_from.clone());
                                    map_ritem.insert((*id1, *level), item.clone());
                                }
                                TieFromType::LetRing => {
                                    println!("LetRing {}", itemidx);
                                }
                                TieFromType::UnresolvedInChunk => {
                                    //
                                    println!("UnresolvedInChunk {}", itemidx);
                                    let next_item = &row.items[itemidx + 1];
                                    dbg!(next_item);
                                }
                            },
                            _ => {}
                        }
                    }

                    let ties_to = nrects
                        .iter()
                        .filter(|nrect| nrect.borrow().is_tie_to())
                        .collect::<Vec<_>>();

                    for tie_to in ties_to {
                        let tie: Ref<NRectExt> = tie_to.borrow();
                        match &tie.1 {
                            NRectType::TieTo(ttype) => match ttype {
                                TieToType::ResolveTieFrom(from_note_id, level) => {
                                    let key: (usize, i8) = (*from_note_id, *level);
                                    let from_rect: Ref<NRectExt> =
                                        map_rect.get(&key).unwrap().borrow();
                                    let from_ritem: Ref<RItem> =
                                        map_ritem.get(&key).unwrap().borrow();

                                    let (
                                        from_type,
                                        from_duration,
                                        from_note_direction,
                                        from_tie_direction,
                                        from_placement,
                                    ) = match &from_rect.1 {
                                        NRectType::TieFrom(
                                            note_id,
                                            level,
                                            ttype,
                                            from_duration,
                                            from_note_direction,
                                            from_tie_direction,
                                            placement,
                                        ) => (
                                            ttype,
                                            from_duration,
                                            from_note_direction,
                                            from_tie_direction,
                                            placement,
                                        ),
                                        _ => todo!(),
                                    };

                                    // dbg!(from_rect.0);
                                    // dbg!(from_rNPoint(item.coord_x.unwrap(), item.coord_y.unwrap()));
                                    // dbg!(from_note_direction, from_tie_direction, from_placement);

                                    let mut from_x = from_ritem.coord_x.unwrap() + from_rect.0 .0;
                                    let mut from_y = from_ritem.coord_y.unwrap() + from_rect.0 .1;
                                    // let to_item_coords = item_.coords.unwrap();
                                    let mut to_x = item_.coord_x.unwrap() + tie.0 .0 + tie.0 .2;
                                    let mut to_y = item_.coord_y.unwrap() + tie.0 .1;

                                    // vertical placement
                                    match from_placement {
                                        TiePlacement::Top => {}
                                        TiePlacement::Mid => {
                                            from_y = from_y + TIE_SPACE;
                                            to_y = to_y + TIE_SPACE;
                                        }
                                        TiePlacement::Bottom => {
                                            from_y = from_y + TIE_SPACE;
                                            to_y = to_y + TIE_SPACE;
                                        }
                                    }

                                    // horizontal placement
                                    match from_note_direction {
                                        DirUD::Up => match from_placement {
                                            TiePlacement::Top => {
                                                from_x += TIE_FROM_WIDTH;
                                            }
                                            TiePlacement::Mid => {
                                                from_x += TIE_FROM_WIDTH;
                                                to_x -= TIE_TO_WIDTH;
                                            }
                                            TiePlacement::Bottom => {
                                                from_x -= TIE_ADJUST_X;
                                                to_x += TIE_ADJUST_X;
                                            }
                                        },

                                        DirUD::Down => match from_placement {
                                            TiePlacement::Top => {
                                                from_x -= TIE_ADJUST_X;
                                                to_x += TIE_ADJUST_X;
                                            }
                                            TiePlacement::Mid => {
                                                to_x -= TIE_TO_WIDTH;
                                                from_x += TIE_FROM_WIDTH;
                                            }
                                            TiePlacement::Bottom => {
                                                to_x -= TIE_TO_WIDTH;
                                            }
                                        },
                                    }

                                    let length = to_x - from_x;
                                    let max_seglength = length / 3.0;
                                    let mut from_x2 = from_x + SPACE.min(max_seglength);
                                    let mut from_y2 = from_y;
                                    let mut to_x2 = to_x - SPACE.min(max_seglength);
                                    let mut to_y2 = to_y;
                                    let tie_height = TIE_HEIGHT.min(length / 60.0 * TIE_HEIGHT);

                                    // tie height
                                    match from_tie_direction {
                                        DirUD::Down => {
                                            from_y2 = from_y2 + tie_height;
                                            to_y2 = to_y2 + tie_height;
                                        }
                                        DirUD::Up => {
                                            from_y2 = from_y2 - tie_height;
                                            to_y2 = to_y2 - tie_height;
                                        }
                                    }

                                    // let rect = NRect::new(-5., -5., 10., 10.);
                                    // graphic_items.push(nrectext2graphic(&NRectExt::new(rect, NRectType::Dev(true, "green".to_string())), from_x, from_y).unwrap());
                                    // let rect = NRect::new(-5., -5., 10., 10.);
                                    // graphic_items.push(nrectext2graphic(&NRectExt::new(rect, NRectType::Dev(true, "lime".to_string())), to_x, to_y).unwrap());
                                    // let rect = NRect::new(-5., -5., 10., 10.);
                                    // graphic_items.push(nrectext2graphic(&NRectExt::new(rect, NRectType::Dev(true, "green".to_string())), from_x2, from_y2).unwrap());
                                    // let rect = NRect::new(-5., -5., 10., 10.);
                                    // graphic_items.push(nrectext2graphic(&NRectExt::new(rect, NRectType::Dev(true, "lime".to_string())), to_x2, to_y2).unwrap());

                                    let points = bezieer(
                                        NPoint(from_x, from_y),
                                        NPoint(from_x2, from_y2),
                                        NPoint(to_x2, to_y2),
                                        NPoint(to_x, to_y),
                                        TIE_SEGMENTS,
                                    );
                                    let mut segments =
                                        vec![PathSegment::M(points[0].0, points[0].1)];
                                    segments.extend(
                                        points
                                            .iter()
                                            .skip(1)
                                            .map(|p| PathSegment::L(p.0, p.1))
                                            .collect::<Vec<_>>(),
                                    );

                                    let points = bezieer(
                                        NPoint(to_x, to_y),
                                        NPoint(
                                            to_x2,
                                            to_y2 + from_tie_direction.sign() * TIE_THICKNESS,
                                        ),
                                        NPoint(
                                            from_x2,
                                            from_y2 + from_tie_direction.sign() * TIE_THICKNESS,
                                        ),
                                        NPoint(from_x, from_y),
                                        TIE_SEGMENTS,
                                    );
                                    segments.extend(
                                        points
                                            .iter()
                                            .skip(1)
                                            .map(|p| PathSegment::L(p.0, p.1))
                                            .collect::<Vec<_>>(),
                                    );
                                    segments.push(PathSegment::Z);
                                    graphic_items.push(Path(
                                        PathSegments(segments),
                                        Strokestyle(2.0, Black),
                                        Fillstyle(Black),
                                        PathCacheInfo::NoCache,
                                    ));
                                }
                                TieToType::LetRing => todo!(),
                            },
                            _ => {}
                        }
                    }
                }
            }
            itemidx += 1;
        }
    }

    graphic_items
}

pub fn output_ackolades(matrix: &RMatrix) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();

    let mut first_y: Option<f32> = None;
    let mut last_y: Option<f32> = None;

    if let Some(bartemplate) = &matrix.bartemplate {
        for (rowidx, row) in matrix.rows.iter().enumerate() {
            let template = bartemplate.0[rowidx];
            match template {
                PartTemplate::Music => {
                    let row = row.borrow();
                    if first_y.is_none() {
                        first_y = Some(row.y);
                    } else {
                        last_y = Some(row.y);
                    }
                }
                _ => {}
            }
        }
    }

    if first_y.is_some() && last_y.is_some() {
        let y1 = first_y.unwrap() - SPACE * 2.0;
        let y2 = last_y.unwrap() + SPACE * 2.0;
        graphic_items.push(Line(0., y1, 0., y2, Strokestyle(NOTELINES_WIDTH, Black)));

        let y1 = y1 - 5.0;
        let y2 = y2 + 5.0;
        graphic_items.push(Rect(
            -SPACE,
            y1,
            SPACE * 0.5,
            y2 - y1,
            NoStroke,
            Fillstyle(Black),
        ));

        graphic_items.push(Path(
            PathSegments(vec![
                M(-SPACE, y1),
                L(0., y1 - 10.0),
                L(0., y1 - 8.0),
                L(-SPACE_HALF, y1),
            ]),
            NoStroke,
            Fillstyle(Black),
            PathCacheInfo::NoCache,
        ));

        graphic_items.push(Path(
            PathSegments(vec![
                M(-SPACE, y2),
                L(0., y2 + 10.0),
                L(0., y2 + 8.0),
                L(-SPACE_HALF, y2),
            ]),
            NoStroke,
            Fillstyle(Black),
            PathCacheInfo::NoCache,
        ));
    }

    graphic_items
}

pub fn output_notelines(matrix: &RMatrix) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();
    if let Some(bartemplate) = &matrix.bartemplate {
        for (rowidx, row) in matrix.rows.iter().enumerate() {
            let template = bartemplate.0[rowidx];
            match template {
                PartTemplate::Music => {
                    let row = row.borrow();
                    for i in -2..3 {
                        let y = row.y + (i as f32) * SPACE;
                        graphic_items.push(Line(
                            0.,
                            y,
                            matrix.width,
                            y,
                            Strokestyle(NOTELINES_WIDTH, Black),
                        ));
                    }
                }
                _ => {}
            }
        }
    }
    graphic_items
}

pub fn output_beamgroups(matrix: &RMatrix) -> GraphicItems {
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
                let item_x = item.coord_x.unwrap();
                let item_y = item.coord_y.unwrap();
                //------------------------------------------------------------------
                match &item.notedata.beamdata1 {
                    RItemBeam::Single(ref data) => {
                        if duration_has_stem(&data.duration) {
                            graphic_items.extend(do_single(
                                data,
                                NPoint(item.coord_x.unwrap(), item.coord_y.unwrap()),
                            ));
                        }
                    }
                    RItemBeam::Start(data) => {
                        notedata = vec![];
                        notedata.push((
                            data.clone(),
                            NPoint(item.coord_x.unwrap(), item.coord_y.unwrap()),
                        ));
                    }
                    RItemBeam::Middle(data) => {
                        notedata.push((
                            data.clone(),
                            NPoint(item.coord_x.unwrap(), item.coord_y.unwrap()),
                        ));
                    }
                    RItemBeam::End(data) => {
                        notedata.push((
                            data.clone(),
                            NPoint(item.coord_x.unwrap(), item.coord_y.unwrap()),
                        ));
                        graphic_items.extend(do_beam(&notedata));
                    }
                    _ => {}
                }
                match &item.notedata.beamdata2 {
                    RItemBeam::Single(data) => {
                        if duration_has_stem(&data.duration) {
                            graphic_items.extend(do_single(
                                data,
                                NPoint(item.coord_x.unwrap(), item.coord_y.unwrap()),
                            ));
                        }
                    }
                    RItemBeam::Start(data) => {
                        note2data = vec![];
                        note2data.push((
                            data.clone(),
                            NPoint(item.coord_x.unwrap(), item.coord_y.unwrap()),
                        ));
                    }
                    RItemBeam::Middle(data) => {
                        note2data.push((
                            data.clone(),
                            NPoint(item.coord_x.unwrap(), item.coord_y.unwrap()),
                        ));
                    }
                    RItemBeam::End(data) => {
                        note2data.push((
                            data.clone(),
                            NPoint(item.coord_x.unwrap(), item.coord_y.unwrap()),
                        ));
                        graphic_items.extend(do_beam(&note2data));
                    }
                    _ => {}
                }
            }
        }
    }

    graphic_items
}

pub fn output_main_elements(matrix: &RMatrix, draw_dev_frames: bool) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();
    for col in matrix.cols.iter() {
        let col = col.borrow();
        let mut rowidx = 0;
        for item in &col.items {
            if let Some(item) = item {
                let item: Ref<RItem> = item.borrow();
                let coords = NPoint(item.coord_x.unwrap(), item.coord_y.unwrap());

                let nrects = item.nrects.as_ref().unwrap();
                for nrect in nrects.iter() {
                    let nrect = nrect.borrow();

                    let color = "orange";
                    let frame_rect = nrect.0;
                    let color = if col.duration == 0 { "orange" } else { "red" };

                    if col.duration == 0 || draw_dev_frames {
                        let frame_nrect =
                            NRectExt::new(frame_rect, NRectType::Dev(false, color.to_string()));
                        let frame_items = nrectext2graphic(&frame_nrect, coords.0, coords.1);

                        graphic_items.extend(GraphicItems(frame_items));
                    }

                    // glyph rect
                    let glyph_items = nrectext2graphic(&nrect, coords.0, coords.1);
                    graphic_items.extend(GraphicItems(glyph_items));

                    // let graphic_item = next2graphic(&nrect, coords.0, coords.1).unwrap();
                }
            } else {
                let y = matrix.get_row(rowidx).unwrap().borrow().y;
                let x = col.x;
                let rect = NRect::new(0., 0., 10.0, 10.0);
                let nrect = NRectExt::new(rect, NRectType::Dev(true, "gray".to_string()));
                let items = nrectext2graphic(&nrect, x, y);
                graphic_items.extend(GraphicItems(items));
            }
            rowidx += 1;
        }
    }
    graphic_items
}

pub fn output_row_nrects(matrix: &RMatrix) -> GraphicItems {
    let mut graphic_items = GraphicItems::new();
    for (rowidx, row) in matrix.rows.iter().enumerate() {
        let row = row.borrow();
        for nrect in row.nrects.iter() {
            let nrect = nrect.borrow();
            let items = nrectext2graphic(&nrect, 0.0, row.y);
            graphic_items.extend(GraphicItems(items));
        }
    }
    graphic_items
}
