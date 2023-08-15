use crate::prelude::*;
use graphics::prelude::*;
use notation_rs::prelude::*;
use std::cell::Ref;
use std::collections::BTreeMap;

pub fn nrect2graphic(n: NRect, s: Stroke, f: graphics::item::Fill) -> GraphicItem {
    Rect(n.0, n.1, n.2, n.3, s, f)
}

pub fn nrectext2graphic(n: &NRectExt, move_x: f32, move_y: f32) -> Option<GraphicItem> {
    let r = n.0.move_rect(move_x, move_y);
    match &n.1 {
        NRectType::Head(head_type, head_shape) => {
            //
            let p = match head_shape {
                HeadShape::BlackHead => CADENZA_HEAD_BLACK.to_vec(),
                HeadShape::WhiteHead => CADENZA_HEAD_WHITE.to_vec(),
                HeadShape::WholeHead => CADENZA_HEAD_WHOLE.to_vec(),
            };
            Some(Path(PathSegments(p).inv01().move_path(r.0, SPACE_HALF + r.1), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
        }

        NRectType::Dotted(dots_nr) => {
            let p = CADENZA_DOT.to_vec();
            Some(Path(
                PathSegments(p).inv01().move_path(r.0 + SPACE_QUARTER, r.1 + SPACE_QUARTER),
                NoStroke,
                Fillstyle(Black),
                PathCacheInfo::NoCache,
            ))
        }

        NRectType::Pause(pause_type) => {
            //
            let p = match pause_type {
                PauseShape::Whole => CADENZA_PAUSE_WHOLE.to_vec(),
                PauseShape::Half => CADENZA_PAUSE_HALF.to_vec(),
                PauseShape::Quarter => CADENZA_PAUSE_QUARTER.to_vec(),
                PauseShape::Eighth => CADENZA_PAUSE_EIGHTH.to_vec(),
                PauseShape::Sixteenth => CADENZA_PAUSE_SIXTEENTH.to_vec(),
                PauseShape::ThirtySecond => CADENZA_PAUSE_THIRTYSECOND.to_vec(),
            };
            let y: f32 = match pause_type {
                PauseShape::Whole => SPACE_HALF,
                PauseShape::Half => SPACE,
                PauseShape::Quarter => 3. * SPACE_HALF,
                PauseShape::Eighth => SPACE,
                PauseShape::Sixteenth => SPACE,
                PauseShape::ThirtySecond => 0.,
            };
            Some(Path(PathSegments(p).inv01().move_path(r.0, r.1 + y), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
        }

        NRectType::Flag(beamtype, direction) => {
            match direction {
                DirUD::Up => match beamtype {
                    BeamType::B8 => Some(Path(
                        PathSegments(CADENZA_FLAG_EIGTH_UP.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1),
                        NoStroke,
                        Fillstyle(Black),
                        PathCacheInfo::NoCache,
                    )),
                    BeamType::B16 => Some(Path(
                        PathSegments(CADENZA_FLAG_SIXTEENTH_UP.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1),
                        NoStroke,
                        Fillstyle(Black),
                        PathCacheInfo::NoCache,
                    )), // 139, 141
                    _ => None,
                    // B32 => 32,
                    // B64 => 34,
                },
                DirUD::Down => match beamtype {
                    BeamType::B8 => Some(Path(
                        PathSegments(CADENZA_FLAG_EIGHT_DOWN.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1 + SPACE * 3.0),
                        NoStroke,
                        Fillstyle(Black),
                        PathCacheInfo::NoCache,
                    )),
                    BeamType::B16 => Some(Path(
                        PathSegments(CADENZA_FLAG_SIXTEENTH_DOWN.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1 + SPACE * 3.0),
                        NoStroke,
                        Fillstyle(Black),
                        PathCacheInfo::NoCache,
                    )), // 139, 141
                    _ => None,
                },
            }
        }

        NRectType::TplSymbol(figure, octave, accidental) => {
            //
            Some(Ellipse(r.0, r.1, r.2, r.3, Strokestyle(7.0, Black), NoFill))
        }

        NRectType::Clef(clef) => match clef {
            Clef::G => Some(Path(
                PathSegments(CADENZA_CLEF_G.to_vec()).inv01().move_path(r.0, r.1 + 4.6 * SPACE),
                NoStroke,
                Fillstyle(Black),
                PathCacheInfo::NoCache,
            )),
            Clef::F => Some(Path(
                PathSegments(CADENZA_CLEF_F.to_vec()).inv01().move_path(r.0, r.1 + SPACE),
                NoStroke,
                Fillstyle(Black),
                PathCacheInfo::NoCache,
            )),
            Clef::C => Some(Path(
                PathSegments(CADENZA_CLEF_C.to_vec()).inv01().move_path(r.0, r.1 + 2.0 * SPACE),
                NoStroke,
                Fillstyle(Black),
                PathCacheInfo::NoCache,
            )),
        },

        NRectType::KeySignature(key, opt_clef) => {
            //
            match key {
                Key::Sharps(n) => {
                    let mut a = PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(0.0, -SPACE * 2.0);
                    if n >= &2 {
                        a.extend(&PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_SHARP, -SPACE * 0.5));
                    }
                    if n >= &3 {
                        a.extend(&PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_SHARP * 2.0, -SPACE * 2.5));
                    }
                    if n >= &4 {
                        a.extend(&PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_SHARP * 3.0, -SPACE * 1.0));
                    }
                    if n >= &5 {
                        a.extend(&PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_SHARP * 4.0, -SPACE * 3.0));
                    }
                    if n >= &6 {
                        a.extend(&PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_SHARP * 5.0, -SPACE * 1.5));
                    }
                    Some(Path(a.move_path(r.0, r.1 + SPACE * 3.5), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
                }
                Key::Flats(n) => {
                    let mut a = PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01();
                    if n >= &2 {
                        a.extend(&PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_FLAT, -SPACE * 1.5));
                    }
                    if n >= &3 {
                        a.extend(&PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_FLAT * 2.0, SPACE * 0.5));
                    }
                    if n >= &4 {
                        a.extend(&PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_FLAT * 3.0, -SPACE * 1.0));
                    }
                    if n >= &5 {
                        a.extend(&PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_FLAT * 4.0, SPACE * 1.0));
                    }
                    if n >= &6 {
                        a.extend(&PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_FLAT * 5.0, -SPACE * 0.5));
                    }
                    Some(Path(a.move_path(r.0, r.1 + SPACE * 3.5), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
                }
                Key::Open => None,
                Key::Naturals(n) => todo!("Key::Naturals not defined yet!"),
            }
        }

        NRectType::TimeSignature(time) => {
            let a = match time {
                Time::Common => PathSegments(CADENZA_TIME_COMMON.to_vec()).inv01(),
                Time::Cut => PathSegments(CADENZA_TIME_CUT.to_vec()).inv01(),
                Time::Standard(nom, denom) => {
                    let x = match nom {
                        TimeNominator::Three => 5.0,
                        _ => 0.0,
                    };
                    // PathSegments(CADENZA_NUMBER_THREE.to_vec()).inv01().move_path(x, -SPACE);
                    let mut a = match denom {
                        TimeDenominator::Wholes => PathSegments(CADENZA_NUMBER_ONE.to_vec()).inv01().move_path(x, SPACE),
                        TimeDenominator::Halves => PathSegments(CADENZA_NUMBER_TWO.to_vec()).inv01().move_path(x, SPACE),
                        TimeDenominator::Quarters => PathSegments(CADENZA_NUMBER_FOUR.to_vec()).inv01().move_path(x, SPACE),
                        TimeDenominator::Egigths => PathSegments(CADENZA_NUMBER_EIGHT.to_vec()).inv01().move_path(x, SPACE),

                        _ => PathSegments(CADENZA_NUMBER_FOUR.to_vec()).inv01().move_path(x, SPACE),
                    };

                    let x = match denom {
                        TimeDenominator::Halves => 5.0,
                        _ => 0.0,
                    };
                    match nom {
                        TimeNominator::One => a.extend(&PathSegments(CADENZA_NUMBER_ONE.to_vec()).inv01().move_path(x, -SPACE)),
                        TimeNominator::Two => a.extend(&PathSegments(CADENZA_NUMBER_TWO.to_vec()).inv01().move_path(x, -SPACE)),
                        TimeNominator::Three => a.extend(&PathSegments(CADENZA_NUMBER_THREE.to_vec()).inv01().move_path(x, -SPACE)),
                        // TimeNominator::Five => a.extend(&PathSegments(CADENZA_NUMBER_FIVE.to_vec()).inv01().move_path(x, -SPACE)),
                        TimeNominator::Six => a.extend(&PathSegments(CADENZA_NUMBER_SIX.to_vec()).inv01().move_path(x, -SPACE)),
                        // TimeNominator::Seven => a.extend(&PathSegments(CADENZA_NUMBER_SEVEN.to_vec()).inv01().move_path(x, -SPACE)),
                        // TimeNominator::Eight => a.extend(&PathSegments(CADENZA_NUMBER_EIGHT.to_vec()).inv01().move_path(x, -SPACE)),
                        TimeNominator::Nine => a.extend(&PathSegments(CADENZA_NUMBER_NINE.to_vec()).inv01().move_path(x, -SPACE)),
                        TimeNominator::Twelve => {
                            a.extend(&PathSegments(CADENZA_NUMBER_ONE.to_vec()).inv01().move_path(x, -SPACE));
                            a.extend(&PathSegments(CADENZA_NUMBER_TWO.to_vec()).inv01().move_path(SPACE, -SPACE));
                        }
                        _ => a.extend(&PathSegments(CADENZA_NUMBER_FOUR.to_vec()).inv01().move_path(x, -SPACE)),
                    }
                    a
                }
            };
            Some(Path(a.move_path(r.0, r.1 + SPACE * 3.0), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
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
                Accidental::Flat => SPACE * 2.0,
                _ => SPACE * 1.5,
            };
            //
            Some(Path(PathSegments(p).inv01().move_path(r.0, r.1 + y), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
        }

        NRectType::WIP(msg) => {
            //
            // println!("WIP:{}", msg);
            None //Some(Path(PathSegments(CADENZA_3.to_vec()).inv01(), NoStroke, Fillstyle(Black)))
        }

        NRectType::ColorRect(color) => {
            let color = Color::from_str(color);
            Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(color)))
        }
        NRectType::StrokeRect(color) => {
            let color = Color::from_str(color);
            Some(Rect(r.0, r.1, r.2, r.3, Strokestyle(1.0, color), NoFill))
        }

        NRectType::TieFrom(_, _, ttype, _, _, _, _) => match ttype {
            TieFromType::Standard => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Green))),
            // TieFromType::Standard => None,
            TieFromType::LetRing => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(LightGray))),
            TieFromType::UnresolvedInChunk => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Red))),
        },

        NRectType::TieTo(ttype) => match ttype {
            TieToType::ResolveTieFrom(_, _) => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Lime))),
            // TieToType::ResolveTieFrom(_, _) => None,
            TieToType::LetRing => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Gray))),
        },

        NRectType::HelpLine => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Black))),

        NRectType::LyricChar(c) => {
            let path = crate::render::fonts::merriweather_regular::get_path(*c).to_vec();
            Some(Path(
                PathSegments(path)
                    .scale_path(LYRICS_FONT_SCALE, LYRICS_FONT_SCALE)
                    .move_path(r.0, r.1 + GLYPH_HEIGHT * LYRICS_FONT_SCALE),
                NoStroke,
                Fillstyle(Black),
                PathCacheInfo::NoCache,
            ))
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

pub fn matrix_to_svg(matrix: &RMatrix, draw_dev_frames: bool) -> String {
    let mut graphic_items = GraphicItems::new();

    graphic_items.extend(output_notelines(matrix));
    graphic_items.extend(output_main_elements(matrix, draw_dev_frames));
    graphic_items.extend(output_beamgroups(matrix));
    // graphic_items.extend(output_ties(matrix));

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

// class Bezieer {
// 	static public function coordinates(anchor1:Pnt, control1:Pnt, control2:Pnt, anchor2:Pnt, ?segments:Int = 15):Pnts {
// 		var coord:Pnts = [];
// 		coord.push(anchor1);
// 		var posx:Float;
// 		var posy:Float;
// 		for (i in 0...segments) {
// 			var u = i / segments;

// 			posx = Math.pow(u, 3) * (anchor2.x + 3 * (control1.x - control2.x) - anchor1.x)
// 				+ 3 * Math.pow(u, 2) * (anchor1.x - 2 * control1.x + control2.x)
// 				+ 3 * u * (control1.x - anchor1.x)
// 				+ anchor1.x;

// 			posy = Math.pow(u, 3) * (anchor2.y + 3 * (control1.y - control2.y) - anchor1.y)
// 				+ 3 * Math.pow(u, 2) * (anchor1.y - 2 * control1.y + control2.y)
// 				+ 3 * u * (control1.y - anchor1.y)
// 				+ anchor1.y;

// 			coord.push({x: posx, y: posy});
// 		}
// 		coord.push(anchor2);
// 		return coord;
// 	}
// }

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
