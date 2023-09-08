use crate::prelude::*;
use crate::render::fonts::opensans_regular::{
    OPENSANS_REGULAR_171, OPENSANS_REGULAR_189, OPENSANS_REGULAR_48, OPENSANS_REGULAR_49, OPENSANS_REGULAR_50, OPENSANS_REGULAR_51, OPENSANS_REGULAR_52, OPENSANS_REGULAR_53, OPENSANS_REGULAR_54,
    OPENSANS_REGULAR_55, OPENSANS_REGULAR_56, OPENSANS_REGULAR_57,
};
use crate::render::fonts::svg_symbols::SVG_SYMBOL_TPL_CIRCLE;
use graphics::prelude::*;
use notation_rs::prelude::*;
use std::cell::Ref;
use std::collections::BTreeMap;

pub fn nrect2graphic(n: NRect, s: Stroke, f: graphics::item::Fill) -> GraphicItem {
    Rect(n.0, n.1, n.2, n.3, s, f)
}

pub fn nrectext2graphic(n: &NRectExt, move_x: f32, move_y: f32) -> Vec<GraphicItem> {
    let r = n.0.move_rect(move_x, move_y);
    match &n.1 {
        NRectType::Head(head_type, head_shape) => {
            //
            let p = match head_shape {
                HeadShape::BlackHead => CADENZA_HEAD_BLACK.to_vec(),
                HeadShape::WhiteHead => CADENZA_HEAD_WHITE.to_vec(),
                HeadShape::WholeHead => CADENZA_HEAD_WHOLE.to_vec(),
            };
            vec![Path(PathSegments(p).inv01().move_path(r.0, SPACE_HALF + r.1), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache)]
        }

        NRectType::Dotted(dots_nr) => {
            let p = CADENZA_DOT.to_vec();
            vec![Path(
                PathSegments(p).inv01().move_path(r.0 + SPACE_QUARTER, r.1 + SPACE_QUARTER),
                NoStroke,
                Fillstyle(Black),
                PathCacheInfo::NoCache,
            )]
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
            vec![Path(PathSegments(p).inv01().move_path(r.0, r.1 + y), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache)]
        }

        NRectType::Flag(beamtype, direction) => {
            match direction {
                DirUD::Up => match beamtype {
                    BeamType::B8 => vec![Path(
                        PathSegments(CADENZA_FLAG_EIGTH_UP.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1),
                        NoStroke,
                        Fillstyle(Black),
                        PathCacheInfo::NoCache,
                    )],
                    BeamType::B16 => vec![Path(
                        PathSegments(CADENZA_FLAG_SIXTEENTH_UP.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1),
                        NoStroke,
                        Fillstyle(Black),
                        PathCacheInfo::NoCache,
                    )], // 139, 141
                    _ => vec![],
                    // B32 => 32,
                    // B64 => 34,
                },
                DirUD::Down => match beamtype {
                    BeamType::B8 => vec![Path(
                        PathSegments(CADENZA_FLAG_EIGHT_DOWN.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1 + SPACE * 3.0),
                        NoStroke,
                        Fillstyle(Black),
                        PathCacheInfo::NoCache,
                    )],
                    BeamType::B16 => vec![Path(
                        PathSegments(CADENZA_FLAG_SIXTEENTH_DOWN.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1 + SPACE * 3.0),
                        NoStroke,
                        Fillstyle(Black),
                        PathCacheInfo::NoCache,
                    )], // 139, 141
                    _ => vec![],
                },
            }
        }

        NRectType::TplSymbol(figure_nr, octave, accidental) => {
            let mut circle = PathSegments(SVG_SYMBOL_TPL_CIRCLE.to_vec()).scale_path(0.14, 0.14).move_path(r.0 - 5.9 * SPACE, r.1 - 0.0 * SPACE);

            let figure = match figure_nr {
                '0' => PathSegments(OPENSANS_REGULAR_48.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
                '1' => PathSegments(OPENSANS_REGULAR_49.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
                '2' => PathSegments(OPENSANS_REGULAR_50.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
                '3' => PathSegments(OPENSANS_REGULAR_51.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
                '4' => PathSegments(OPENSANS_REGULAR_52.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
                '5' => PathSegments(OPENSANS_REGULAR_53.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
                '6' => PathSegments(OPENSANS_REGULAR_54.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
                '7' => PathSegments(OPENSANS_REGULAR_55.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
                '8' => PathSegments(OPENSANS_REGULAR_56.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
                '9' => PathSegments(OPENSANS_REGULAR_57.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
                _ => PathSegments(OPENSANS_REGULAR_48.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
            };

            circle.extend(&figure);

            vec![Path(circle, NoStroke, Fillstyle(Black), PathCacheInfo::NoCache)]
        }

        NRectType::Clef(clef) => match clef {
            Clef::G => vec![Path(
                PathSegments(CADENZA_CLEF_G.to_vec()).inv01().move_path(r.0, r.1 + 4.6 * SPACE),
                NoStroke,
                Fillstyle(Black),
                PathCacheInfo::NoCache,
            )],
            Clef::F => vec![Path(
                PathSegments(CADENZA_CLEF_F.to_vec()).inv01().move_path(r.0, r.1 + SPACE),
                NoStroke,
                Fillstyle(Black),
                PathCacheInfo::NoCache,
            )],
            Clef::C => vec![Path(
                PathSegments(CADENZA_CLEF_C.to_vec()).inv01().move_path(r.0, r.1 + 2.0 * SPACE),
                NoStroke,
                Fillstyle(Black),
                PathCacheInfo::NoCache,
            )],
        },

        NRectType::KeySignature(key, opt_clef) => {
            //
            match key {
                Key::Sharps(n, _) => {
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
                    vec![Path(a.move_path(r.0, r.1 + SPACE * 3.5), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache)]
                }
                Key::Flats(n, _) => {
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
                    vec![Path(a.move_path(r.0, r.1 + SPACE * 3.5), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache)]
                }
                Key::Open => vec![],
                Key::Naturals(n, _) => todo!("Key::Naturals not defined yet!"),
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
                        TimeNominator::Three => a.extend(&PathSegments(CADENZA_NUMBER_THREE.to_vec()).inv01().move_path(x + 6.0, -SPACE)),
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
            vec![Path(a.move_path(r.0, r.1 + SPACE * 3.0), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache)]
        }

        NRectType::Barline(btype) => match btype {
            BarlineType::Single => vec![Rect(r.0 + (r.2 - BARLINE_WIDTH_SINGLE), r.1, r.2, r.3, NoStroke, Fillstyle(Black))],
            BarlineType::Double => {
                let mut line = PathSegments([M(r.0, r.1), L(r.0 + BARLINE_WIDTH_SINGLE, r.1), L(r.0 + BARLINE_WIDTH_SINGLE, r.1 + r.3), L(r.0, r.1 + r.3)].to_vec());
                let line2 = PathSegments([M(r.0, r.1), L(r.0 + BARLINE_WIDTH_SINGLE, r.1), L(r.0 + BARLINE_WIDTH_SINGLE, r.1 + r.3), L(r.0, r.1 + r.3)].to_vec())
                    .move_path(BARLINE_DOUBLE_SPACE - BARLINE_WIDTH_SINGLE, 0.);
                line.extend(&line2);
                let path = Path(line, NoStroke, Fillstyle(Black), PathCacheInfo::NoCache);
                vec![path]
            }

            BarlineType::Final => todo!(),
            BarlineType::RepeatTo => todo!(),
            BarlineType::RepeatFrom => todo!(),
            BarlineType::RepeatToAndFrom => todo!(),
            BarlineType::FraseTick => {
                let path = Line(r.0, r.1 + SPACE_HALF, r.0 + SPACE_HALF, r.1 - SPACE, Strokestyle(4.0, Black));
                vec![path]
            }
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
            vec![Path(PathSegments(p).inv01().move_path(r.0, r.1 + y), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache)]
        }

        NRectType::WIP(msg) => {
            //
            // println!("WIP:{}", msg);
            vec![] //vec![Path(PathSegments(CADENZA_3.to_vec()).inv01(), NoStroke, Fillstyle(Black)))
        }

        NRectType::ColorRect(color) => {
            let color = Color::from_str(color);
            vec![Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(color))]
        }
        NRectType::StrokeRect(color) => {
            let color = Color::from_str(color);
            vec![Rect(r.0, r.1, r.2, r.3, Strokestyle(1.0, color), NoFill)]
        }

        NRectType::TieFrom(_, _, ttype, _, _, _, _) => match ttype {
            // TieFromType::Standard => vec![Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Green))),
            TieFromType::Standard => vec![],
            TieFromType::LetRing => vec![Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(LightGray))],
            TieFromType::UnresolvedInChunk => vec![Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Red))],
        },

        NRectType::TieTo(ttype) => match ttype {
            // TieToType::ResolveTieFrom(_, _) => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Lime))),
            TieToType::ResolveTieFrom(_, _) => vec![],
            TieToType::LetRing => vec![Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Gray))],
        },

        NRectType::HelpLine => vec![Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Black))],

        NRectType::LyricChar(c) => {
            let path = crate::render::fonts::merriweather_regular::get_path(*c).to_vec();
            vec![Path(
                PathSegments(path)
                    .scale_path(LYRICS_FONT_SCALE, LYRICS_FONT_SCALE)
                    .move_path(r.0, r.1 + GLYPH_HEIGHT * LYRICS_FONT_SCALE),
                NoStroke,
                Fillstyle(Black),
                PathCacheInfo::NoCache,
            )]
        }
        NRectType::Dev(ellipse, color) => {
            let color = Color::from_str(color);
            if *ellipse {
                vec![Ellipse(r.0, r.1, r.2, r.3, Strokestyle(1.0, color), NoFill)]
            } else {
                vec![Rect(r.0, r.1, r.2, r.3, Strokestyle(1.0, color), NoFill)]
            }
        }

        NRectType::DUMMY => vec![],
        NRectType::Spacer(s) => vec![],

        NRectType::LineFrom(level_from, line_to, line_type) => vec![Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Orange))],
        NRectType::LineTo(level_from, level_to, line_type) => {
            //
            dbg!(r);
            // vec![Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Lime)))
            // vec![Rect(from_rect.0, from_rect.1, 10.0, 10.0, NoStroke, Fillstyle(Lime)))

            // match line_type {
            //     // HeadLineType::Line => {
            //     //     vec![Line(from_rect.0, from_rect.1, r.0, r.1, Strokestyle(4.0, Black)))
            //     // }
            //     // HeadLineType::Glissando => {}
            //     // HeadLineType::GlissandoWave => {}
            //     HeadLineType::Halfstep => {
            //         let width = r.0 - from_rect.0;
            //         let midx = width / 2.0;
            //         let height = r.1 - from_rect.1;
            //         let midy = (height / 2.0) + 15.0;
            //         let mut p = PathSegments([M(from_rect.0, from_rect.1), L(from_rect.0 + midx, from_rect.1 + midy), L(r.0, r.1)].to_vec());
            //         let mut p1 = PathSegments(OPENSANS_REGULAR_189.to_vec()).scale_path(0.05, 0.05).move_path(from_rect.0 + midx - 15.0, r.1);
            //         vec![
            //             Path(p, Strokestyle(4.0, Red), NoFill, PathCacheInfo::NoCache),
            //             Path(p1, NoStroke, Fillstyle(Black), PathCacheInfo::NoCache),
            //         ]
            //     }
            //     HeadLineType::Wholestep => {
            //         let width = r.0 - from_rect.0;
            //         let midx = width / 2.0;
            //         let height = 15.0;
            //         let mut p = PathSegments([M(from_rect.0, from_rect.1), L(from_rect.0, from_rect.1 + height), L(r.0, r.1 + height), L(r.0, r.1)].to_vec());
            //         let mut p1 = PathSegments(OPENSANS_REGULAR_49.to_vec()).scale_path(0.04, 0.04).move_path(from_rect.0 + midx - 10.0, r.1 - 2.0);
            //         vec![
            //             Path(p, Strokestyle(4.0, Blue), NoFill, PathCacheInfo::NoCache),
            //             Path(p1, NoStroke, Fillstyle(Black), PathCacheInfo::NoCache),
            //         ]
            //     }
            //     _ => vec![Line(from_rect.0, from_rect.1, r.0, r.1, Strokestyle(4.0, Lime))],
            // }
            //
            vec![]
        }
    }
}

// pub fn nrectext2graphic(n: &NRectExt, move_x: f32, move_y: f32) -> Option<GraphicItem> {
//     let r = n.0.move_rect(move_x, move_y);
//     match &n.1 {
//         NRectType::Head(head_type, head_shape) => {
//             //
//             let p = match head_shape {
//                 HeadShape::BlackHead => CADENZA_HEAD_BLACK.to_vec(),
//                 HeadShape::WhiteHead => CADENZA_HEAD_WHITE.to_vec(),
//                 HeadShape::WholeHead => CADENZA_HEAD_WHOLE.to_vec(),
//             };
//             vec![Path(PathSegments(p).inv01().move_path(r.0, SPACE_HALF + r.1), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
//         }

//         NRectType::Dotted(dots_nr) => {
//             let p = CADENZA_DOT.to_vec();
//             Some(Path(
//                 PathSegments(p).inv01().move_path(r.0 + SPACE_QUARTER, r.1 + SPACE_QUARTER),
//                 NoStroke,
//                 Fillstyle(Black),
//                 PathCacheInfo::NoCache,
//             ))
//         }

//         NRectType::Pause(pause_type) => {
//             //
//             let p = match pause_type {
//                 PauseShape::Whole => CADENZA_PAUSE_WHOLE.to_vec(),
//                 PauseShape::Half => CADENZA_PAUSE_HALF.to_vec(),
//                 PauseShape::Quarter => CADENZA_PAUSE_QUARTER.to_vec(),
//                 PauseShape::Eighth => CADENZA_PAUSE_EIGHTH.to_vec(),
//                 PauseShape::Sixteenth => CADENZA_PAUSE_SIXTEENTH.to_vec(),
//                 PauseShape::ThirtySecond => CADENZA_PAUSE_THIRTYSECOND.to_vec(),
//             };
//             let y: f32 = match pause_type {
//                 PauseShape::Whole => SPACE_HALF,
//                 PauseShape::Half => SPACE,
//                 PauseShape::Quarter => 3. * SPACE_HALF,
//                 PauseShape::Eighth => SPACE,
//                 PauseShape::Sixteenth => SPACE,
//                 PauseShape::ThirtySecond => 0.,
//             };
//             Some(Path(PathSegments(p).inv01().move_path(r.0, r.1 + y), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
//         }

//         NRectType::Flag(beamtype, direction) => {
//             match direction {
//                 DirUD::Up => match beamtype {
//                     BeamType::B8 => Some(Path(
//                         PathSegments(CADENZA_FLAG_EIGTH_UP.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1),
//                         NoStroke,
//                         Fillstyle(Black),
//                         PathCacheInfo::NoCache,
//                     )),
//                     BeamType::B16 => Some(Path(
//                         PathSegments(CADENZA_FLAG_SIXTEENTH_UP.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1),
//                         NoStroke,
//                         Fillstyle(Black),
//                         PathCacheInfo::NoCache,
//                     )), // 139, 141
//                     _ => None,
//                     // B32 => 32,
//                     // B64 => 34,
//                 },
//                 DirUD::Down => match beamtype {
//                     BeamType::B8 => Some(Path(
//                         PathSegments(CADENZA_FLAG_EIGHT_DOWN.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1 + SPACE * 3.0),
//                         NoStroke,
//                         Fillstyle(Black),
//                         PathCacheInfo::NoCache,
//                     )),
//                     BeamType::B16 => Some(Path(
//                         PathSegments(CADENZA_FLAG_SIXTEENTH_DOWN.to_vec()).inv01().move_path(r.0 - FLAG_X_ADJUST, r.1 + SPACE * 3.0),
//                         NoStroke,
//                         Fillstyle(Black),
//                         PathCacheInfo::NoCache,
//                     )), // 139, 141
//                     _ => None,
//                 },
//             }
//         }

//         NRectType::TplSymbol(figure_nr, octave, accidental) => {
//             let mut circle = PathSegments(SVG_SYMBOL_TPL_CIRCLE.to_vec()).scale_path(0.14, 0.14).move_path(r.0 - 5.9 * SPACE, r.1 - 0.0 * SPACE);

//             let figure = match figure_nr {
//                 '0' => PathSegments(OPENSANS_REGULAR_48.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
//                 '1' => PathSegments(OPENSANS_REGULAR_49.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
//                 '2' => PathSegments(OPENSANS_REGULAR_50.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
//                 '3' => PathSegments(OPENSANS_REGULAR_51.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
//                 '4' => PathSegments(OPENSANS_REGULAR_52.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
//                 '5' => PathSegments(OPENSANS_REGULAR_53.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
//                 '6' => PathSegments(OPENSANS_REGULAR_54.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
//                 '7' => PathSegments(OPENSANS_REGULAR_55.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
//                 '8' => PathSegments(OPENSANS_REGULAR_56.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
//                 '9' => PathSegments(OPENSANS_REGULAR_57.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
//                 _ => PathSegments(OPENSANS_REGULAR_48.to_vec()).scale_path(0.07, 0.07).move_path(r.0 + SPACE * 0.85, r.1 + SPACE * 2.2),
//             };

//             circle.extend(&figure);

//             Some(Path(circle, NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
//         }

//         NRectType::Clef(clef) => match clef {
//             Clef::G => Some(Path(
//                 PathSegments(CADENZA_CLEF_G.to_vec()).inv01().move_path(r.0, r.1 + 4.6 * SPACE),
//                 NoStroke,
//                 Fillstyle(Black),
//                 PathCacheInfo::NoCache,
//             )),
//             Clef::F => Some(Path(
//                 PathSegments(CADENZA_CLEF_F.to_vec()).inv01().move_path(r.0, r.1 + SPACE),
//                 NoStroke,
//                 Fillstyle(Black),
//                 PathCacheInfo::NoCache,
//             )),
//             Clef::C => Some(Path(
//                 PathSegments(CADENZA_CLEF_C.to_vec()).inv01().move_path(r.0, r.1 + 2.0 * SPACE),
//                 NoStroke,
//                 Fillstyle(Black),
//                 PathCacheInfo::NoCache,
//             )),
//         },

//         NRectType::KeySignature(key, opt_clef) => {
//             //
//             match key {
//                 Key::Sharps(n, _) => {
//                     let mut a = PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(0.0, -SPACE * 2.0);
//                     if n >= &2 {
//                         a.extend(&PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_SHARP, -SPACE * 0.5));
//                     }
//                     if n >= &3 {
//                         a.extend(&PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_SHARP * 2.0, -SPACE * 2.5));
//                     }
//                     if n >= &4 {
//                         a.extend(&PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_SHARP * 3.0, -SPACE * 1.0));
//                     }
//                     if n >= &5 {
//                         a.extend(&PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_SHARP * 4.0, -SPACE * 3.0));
//                     }
//                     if n >= &6 {
//                         a.extend(&PathSegments(CADENZA_ACCIDENTAL_SHARP.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_SHARP * 5.0, -SPACE * 1.5));
//                     }
//                     Some(Path(a.move_path(r.0, r.1 + SPACE * 3.5), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
//                 }
//                 Key::Flats(n, _) => {
//                     let mut a = PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01();
//                     if n >= &2 {
//                         a.extend(&PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_FLAT, -SPACE * 1.5));
//                     }
//                     if n >= &3 {
//                         a.extend(&PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_FLAT * 2.0, SPACE * 0.5));
//                     }
//                     if n >= &4 {
//                         a.extend(&PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_FLAT * 3.0, -SPACE * 1.0));
//                     }
//                     if n >= &5 {
//                         a.extend(&PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_FLAT * 4.0, SPACE * 1.0));
//                     }
//                     if n >= &6 {
//                         a.extend(&PathSegments(CADENZA_ACCIDENTAL_FLAT.to_vec()).inv01().move_path(ACCIDENTAL_WIDTH_FLAT * 5.0, -SPACE * 0.5));
//                     }
//                     Some(Path(a.move_path(r.0, r.1 + SPACE * 3.5), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
//                 }
//                 Key::Open => None,
//                 Key::Naturals(n, _) => todo!("Key::Naturals not defined yet!"),
//             }
//         }

//         NRectType::TimeSignature(time) => {
//             let a = match time {
//                 Time::Common => PathSegments(CADENZA_TIME_COMMON.to_vec()).inv01(),
//                 Time::Cut => PathSegments(CADENZA_TIME_CUT.to_vec()).inv01(),
//                 Time::Standard(nom, denom) => {
//                     let x = match nom {
//                         TimeNominator::Three => 5.0,
//                         _ => 0.0,
//                     };
//                     // PathSegments(CADENZA_NUMBER_THREE.to_vec()).inv01().move_path(x, -SPACE);
//                     let mut a = match denom {
//                         TimeDenominator::Wholes => PathSegments(CADENZA_NUMBER_ONE.to_vec()).inv01().move_path(x, SPACE),
//                         TimeDenominator::Halves => PathSegments(CADENZA_NUMBER_TWO.to_vec()).inv01().move_path(x, SPACE),
//                         TimeDenominator::Quarters => PathSegments(CADENZA_NUMBER_FOUR.to_vec()).inv01().move_path(x, SPACE),
//                         TimeDenominator::Egigths => PathSegments(CADENZA_NUMBER_EIGHT.to_vec()).inv01().move_path(x, SPACE),
//                         _ => PathSegments(CADENZA_NUMBER_FOUR.to_vec()).inv01().move_path(x, SPACE),
//                     };

//                     let x = match denom {
//                         TimeDenominator::Halves => 5.0,
//                         _ => 0.0,
//                     };
//                     match nom {
//                         TimeNominator::One => a.extend(&PathSegments(CADENZA_NUMBER_ONE.to_vec()).inv01().move_path(x, -SPACE)),
//                         TimeNominator::Two => a.extend(&PathSegments(CADENZA_NUMBER_TWO.to_vec()).inv01().move_path(x, -SPACE)),
//                         TimeNominator::Three => a.extend(&PathSegments(CADENZA_NUMBER_THREE.to_vec()).inv01().move_path(x + 6.0, -SPACE)),
//                         // TimeNominator::Five => a.extend(&PathSegments(CADENZA_NUMBER_FIVE.to_vec()).inv01().move_path(x, -SPACE)),
//                         TimeNominator::Six => a.extend(&PathSegments(CADENZA_NUMBER_SIX.to_vec()).inv01().move_path(x, -SPACE)),
//                         // TimeNominator::Seven => a.extend(&PathSegments(CADENZA_NUMBER_SEVEN.to_vec()).inv01().move_path(x, -SPACE)),
//                         // TimeNominator::Eight => a.extend(&PathSegments(CADENZA_NUMBER_EIGHT.to_vec()).inv01().move_path(x, -SPACE)),
//                         TimeNominator::Nine => a.extend(&PathSegments(CADENZA_NUMBER_NINE.to_vec()).inv01().move_path(x, -SPACE)),
//                         TimeNominator::Twelve => {
//                             a.extend(&PathSegments(CADENZA_NUMBER_ONE.to_vec()).inv01().move_path(x, -SPACE));
//                             a.extend(&PathSegments(CADENZA_NUMBER_TWO.to_vec()).inv01().move_path(SPACE, -SPACE));
//                         }
//                         _ => a.extend(&PathSegments(CADENZA_NUMBER_FOUR.to_vec()).inv01().move_path(x, -SPACE)),
//                     }
//                     a
//                 }
//             };
//             Some(Path(a.move_path(r.0, r.1 + SPACE * 3.0), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
//         }

//         NRectType::Barline(btype) => match btype {
//             BarlineType::Single => Some(Rect(r.0 + (r.2 - BARLINE_WIDTH_SINGLE), r.1, r.2, r.3, NoStroke, Fillstyle(Black))),
//             BarlineType::Double => {
//                 let mut line = PathSegments([M(r.0, r.1), L(r.0 + BARLINE_WIDTH_SINGLE, r.1), L(r.0 + BARLINE_WIDTH_SINGLE, r.1 + r.3), L(r.0, r.1 + r.3)].to_vec());
//                 let line2 = PathSegments([M(r.0, r.1), L(r.0 + BARLINE_WIDTH_SINGLE, r.1), L(r.0 + BARLINE_WIDTH_SINGLE, r.1 + r.3), L(r.0, r.1 + r.3)].to_vec())
//                     .move_path(BARLINE_DOUBLE_SPACE - BARLINE_WIDTH_SINGLE, 0.);
//                 line.extend(&line2);
//                 let path = Path(line, NoStroke, Fillstyle(Black), PathCacheInfo::NoCache);
//                 Some(path)
//             }

//             BarlineType::Final => todo!(),
//             BarlineType::RepeatTo => todo!(),
//             BarlineType::RepeatFrom => todo!(),
//             BarlineType::RepeatToAndFrom => todo!(),
//             BarlineType::FraseTick => {
//                 let path = Line(r.0, r.1 + SPACE_HALF, r.0 + SPACE_HALF, r.1 - SPACE, Strokestyle(4.0, Black));
//                 Some(path)
//             }
//         },

//         NRectType::Accidental(accidental) => {
//             let p = match accidental {
//                 Accidental::Sharp => CADENZA_ACCIDENTAL_SHARP.to_vec(),
//                 Accidental::Flat => CADENZA_ACCIDENTAL_FLAT.to_vec(),
//                 Accidental::Natural => CADENZA_ACCIDENTAL_NATURAL.to_vec(),
//                 Accidental::DblSharp => CADENZA_ACCIDENTAL_DOUBLESHARP.to_vec(),
//                 Accidental::DblFlat => CADENZA_ACCIDENTAL_DOUBLEFLAT.to_vec(),
//                 // _ => CADENZA_ACCIDENTAL_FLAT.to_vec(),
//             };
//             let y = match accidental {
//                 Accidental::Flat => SPACE * 2.0,
//                 _ => SPACE * 1.5,
//             };
//             //
//             Some(Path(PathSegments(p).inv01().move_path(r.0, r.1 + y), NoStroke, Fillstyle(Black), PathCacheInfo::NoCache))
//         }

//         NRectType::WIP(msg) => {
//             //
//             // println!("WIP:{}", msg);
//             None //Some(Path(PathSegments(CADENZA_3.to_vec()).inv01(), NoStroke, Fillstyle(Black)))
//         }

//         NRectType::ColorRect(color) => {
//             let color = Color::from_str(color);
//             Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(color)))
//         }
//         NRectType::StrokeRect(color) => {
//             let color = Color::from_str(color);
//             Some(Rect(r.0, r.1, r.2, r.3, Strokestyle(1.0, color), NoFill))
//         }

//         NRectType::TieFrom(_, _, ttype, _, _, _, _) => match ttype {
//             // TieFromType::Standard => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Green))),
//             TieFromType::Standard => None,
//             TieFromType::LetRing => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(LightGray))),
//             TieFromType::UnresolvedInChunk => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Red))),
//         },

//         NRectType::TieTo(ttype) => match ttype {
//             // TieToType::ResolveTieFrom(_, _) => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Lime))),
//             TieToType::ResolveTieFrom(_, _) => None,
//             TieToType::LetRing => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Gray))),
//         },

//         NRectType::HelpLine => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Black))),

//         NRectType::LyricChar(c) => {
//             let path = crate::render::fonts::merriweather_regular::get_path(*c).to_vec();
//             Some(Path(
//                 PathSegments(path)
//                     .scale_path(LYRICS_FONT_SCALE, LYRICS_FONT_SCALE)
//                     .move_path(r.0, r.1 + GLYPH_HEIGHT * LYRICS_FONT_SCALE),
//                 NoStroke,
//                 Fillstyle(Black),
//                 PathCacheInfo::NoCache,
//             ))
//         }
//         NRectType::Dev(ellipse, color) => {
//             let color = Color::from_str(color);
//             if *ellipse {
//                 Some(Ellipse(r.0, r.1, r.2, r.3, Strokestyle(1.0, color), NoFill))
//             } else {
//                 Some(Rect(r.0, r.1, r.2, r.3, Strokestyle(1.0, color), NoFill))
//             }
//         }

//         NRectType::DUMMY => None,
//         NRectType::Spacer(s) => None,

//         NRectType::LineFrom(level_from, line_type) => Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Orange))),
//         NRectType::LineTo(level_from, level_to, line_type, from_rect) => {
//             //
//             // dbg!(r, from_rect);
//             // Some(Rect(r.0, r.1, r.2, r.3, NoStroke, Fillstyle(Lime)))
//             // Some(Rect(from_rect.0, from_rect.1, 10.0, 10.0, NoStroke, Fillstyle(Lime)))

//             match line_type {
//                 // HeadLineType::Line => {
//                 //     Some(Line(from_rect.0, from_rect.1, r.0, r.1, Strokestyle(4.0, Black)))
//                 // }
//                 // HeadLineType::Glissando => {}
//                 // HeadLineType::GlissandoWave => {}
//                 HeadLineType::Halfstep => {
//                     let width = r.0 - from_rect.0;
//                     let midx = width / 2.0;
//                     let height = r.1 - from_rect.1;
//                     let midy = (height / 2.0) + 15.0;
//                     let mut p = PathSegments([M(from_rect.0, from_rect.1), L(from_rect.0 + midx, from_rect.1 + midy), L(r.0, r.1)].to_vec());

//                     let mut p2 = PathSegments(OPENSANS_REGULAR_49.to_vec()).scale_path(0.03, 0.03).move_path(from_rect.0 + midx, r.1);

//                     p.extend(&p2);

//                     Some(Path(p, Strokestyle(4.0, Red), NoFill, PathCacheInfo::NoCache))
//                 }
//                 HeadLineType::Wholestep => {
//                     let height = 15.0;
//                     let mut p = PathSegments([M(from_rect.0, from_rect.1), L(from_rect.0, from_rect.1 + height), L(r.0, r.1 + height), L(r.0, r.1)].to_vec());
//                     Some(Path(p, Strokestyle(4.0, Gray), NoFill, PathCacheInfo::NoCache))
//                 }
//                 _ => Some(Line(from_rect.0, from_rect.1, r.0, r.1, Strokestyle(4.0, Lime))),
//             }

//             //
//         }
//     }
// }
