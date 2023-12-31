use graphics::path::PathSegment;
use graphics::path::PathSegment::*;

pub const SVG_SYMBOL_TPL_CIRCLE: &'static [PathSegment] = &[
    M(1316.42, -22.815),
    C(1463.73, -22.815, 1583.19, 104.929, 1583.19, 262.105),
    C(1583.19, 419.282, 1463.73, 547.026, 1316.42, 547.026),
    C(1169.11, 547.026, 1049.64, 419.282, 1049.64, 262.105),
    C(1049.64, 104.929, 1169.11, -22.815, 1316.42, -22.815),
    Z,
    M(1316.42, 22.815),
    C(1192.57, 22.815, 1091.9, 129.962, 1091.9, 262.105),
    C(1091.9, 394.248, 1192.57, 501.395, 1316.42, 501.395),
    C(1440.26, 501.395, 1540.93, 394.248, 1540.93, 262.105),
    C(1540.93, 129.962, 1440.26, 22.815, 1316.42, 22.815),
    Z,
];
