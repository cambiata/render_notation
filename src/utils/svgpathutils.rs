#[cfg(test)]
mod tests2 {
    use crate::utils::svgpathutils::{get_segments, parse_segments};

    const SVG_PATH: &str = r#"M1316.42,-22.815C1463.73,-22.815 1583.19,104.929 1583.19,262.105C1583.19,419.282 1463.73,547.026 1316.42,547.026C1169.11,547.026 1049.64,419.282 1049.64,262.105C1049.64,104.929 1169.11,-22.815 1316.42,-22.815ZM1316.42,22.815C1192.57,22.815 1091.9,129.962 1091.9,262.105C1091.9,394.248 1192.57,501.395 1316.42,501.395C1440.26,501.395 1540.93,394.248 1540.93,262.105C1540.93,129.962 1440.26,22.815 1316.42,22.815Z"#;

    #[test]
    fn example() {
        dbg!(SVG_PATH);
        let segments: Vec<String> = get_segments(SVG_PATH.to_string());
        parse_segments(segments);
    }
}
fn get_segments(path_string: String) -> Vec<String> {
    let mut segments: Vec<String> = vec![];
    let mut segment: String = "".to_string();
    for char in path_string.chars() {
        print!("{:?}", char);

        match char {
            'M' | 'C' | 'L' | 'Z' => {
                segments.push(segment.clone());
                segment.clear();
                segment.push(char);
            }
            _ => {}
        }

        match char {
            'M' | 'C' | 'L' | 'Z' => {}
            _ => segment.push(char),
        }
    }
    if segment.len() > 0 {
        segments.push(segment);
    }
    for segment in segments.iter() {
        println!("{}", segment);
    }
    segments
}

fn parse_segments(segments: Vec<String>) {
    let mut path_segments: Vec<PathSegment> = vec![];
    for segment in segments.iter() {
        println!("{}", segment);
        if segment.len() == 0 {
            continue;
        }
        match segment.chars().next().unwrap() {
            'M' => {
                let values_segments = &segment[1..].split(',').collect::<Vec<&str>>();
                let values = values_segments.iter().map(|s| s.parse::<f32>().unwrap()).collect::<Vec<f32>>();
                let path_segment: PathSegment = PathSegment::M(values[0], values[1]);
                path_segments.push(path_segment);
            }
            'L' => {
                let values_segments = &segment[1..].split(',').collect::<Vec<&str>>();
                let values = values_segments.iter().map(|s| s.parse::<f32>().unwrap()).collect::<Vec<f32>>();
                let path_segment: PathSegment = PathSegment::L(values[0], values[1]);
                path_segments.push(path_segment);
            }
            'C' => {
                let values_segments: &Vec<&str> = &segment[1..].split(' ').collect::<Vec<&str>>();
                let mut vs: Vec<f32> = vec![];
                for value_segment in values_segments.iter() {
                    let values = value_segment.split(',').collect::<Vec<&str>>();
                    for value in values.iter() {
                        let v = value.parse::<f32>().unwrap();
                        vs.push(v);
                    }
                }

                // let values = values_segments.iter().map(|s| s.parse::<f32>().unwrap()).collect::<Vec<f32>>();
                let path_segment: PathSegment = PathSegment::C(vs[0], vs[1], vs[2], vs[3], vs[4], vs[5]);
                path_segments.push(path_segment);
            }
            'Z' => {
                let path_segment: PathSegment = PathSegment::Z;
                path_segments.push(path_segment);
            }
            _ => {}
        }
    }

    dbg!(path_segments);
}

#[derive(Debug, Clone)]
pub enum PathSegment {
    M(f32, f32),
    L(f32, f32),
    Q(f32, f32, f32, f32),
    C(f32, f32, f32, f32, f32, f32),
    Z,
}
