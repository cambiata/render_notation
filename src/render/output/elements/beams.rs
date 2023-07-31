use graphics::prelude::*;
use notation_rs::prelude::*;
use std::cell::Ref;

pub fn do_single(first_data: &RItemBeamData, coords: NPoint) -> GraphicItems {
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

pub fn do_beam(items: &Vec<(RItemBeamData, NPoint)>) -> GraphicItems {
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

pub fn do_sub_beams(beam_width: f32, beam_height: f32, tip_coords: &Vec<(f32, f32, f32)>, direction: DirUD, durations: &Vec<Duration>) -> GraphicItems {
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
        PathCacheInfo::NoCache,
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
                PathCacheInfo::NoCache,
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

pub fn do_sub_sixteen_rightside(left: (f32, f32), right: (f32, f32), direction: DirUD) -> GraphicItems {
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
        PathCacheInfo::NoCache,
    ));

    graphic_items
}

pub fn do_sub_sixteen(left: (f32, f32), right: (f32, f32), direction: DirUD) -> GraphicItems {
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
        PathCacheInfo::NoCache,
    ));

    graphic_items
}

pub fn do_sub_sixteen_leftside(left: (f32, f32), right: (f32, f32), direction: DirUD) -> GraphicItems {
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
        PathCacheInfo::NoCache,
    ));

    graphic_items.push(Line(x, y, x2, y2, Strokestyle(DEV_LINE_THICKNESS, Red)));
    graphic_items
}
