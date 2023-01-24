use std::io;

struct Point {
    x: f64,
    y: f64,
}

struct Segment {
    start: Point,
    end: Point,
}

fn point_on_segment(point: &Point, segment: &Segment) -> bool {
    let x = point.x;
    let y = point.y;
    let x1 = segment.start.x;
    let y1 = segment.start.y;
    let x2 = segment.end.x;
    let y2 = segment.end.y;

    (x <= x1 && x >= x2 || x >= x1 && x <= x2) && (y <= y1 && y >= y2 || y >= y1 && y <= y2)
}

fn point_on_left_right(point: &Point, segment: &Segment) -> i8 {
    let x = point.x;
    let y = point.y;
    let x1 = segment.start.x;
    let y1 = segment.start.y;
    let x2 = segment.end.x;
    let y2 = segment.end.y;

    let slope = (y2 - y1) / (x2 - x1);
    let y_intercept = y1 - (slope * x1);
    let y_on_line = slope * x + y_intercept;
    if y > y_on_line {
        1
    } else if y < y_on_line {
        -1
    } else {
        0
    }
}

fn half_line_segment_intersect(
    half_line_start: &Point,
    half_line_end: &Point,
    segment: &Segment,
) -> bool {
    if point_on_segment(half_line_start, segment) || point_on_segment(half_line_end, segment) {
        return true;
    }
    let half_line_left_right = point_on_left_right(half_line_start, segment);
    let segment_left_right = point_on_left_right(half_line_end, segment);
    if half_line_left_right == 0 || segment_left_right == 0 {
        return true;
    }
    half_line_left_right == segment_left_right
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_vec: Vec<&str> = input.trim().split(" ").collect();
    let half_line_start = Point {
        x: input_vec[0].split(",").collect::<Vec<&str>>()[0]
            .parse::<f64>()
            .unwrap(),
        y: input_vec[0].split(",").collect::<Vec<&str>>()[1]
            .parse::<f64>()
            .unwrap(),
    };
    let half_line_end = Point {
        x: input_vec[1].split(",").collect::<Vec<&str>>()[0]
            .parse::<f64>()
            .unwrap(),
        y: input_vec[1].split(",").collect::<Vec<&str>>()[1]
            .parse::<f64>()
            .unwrap(),
    };

    let mut segments = Vec::new();
    let mut input = String::new();
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let input_vec: Vec<&str> = input.trim().split(" ").collect();
        let start = Point {
            x: input_vec[0].split(",").collect::<Vec<&str>>()[0]
                .parse::<f64>()
                .unwrap(),
            y: input_vec[0].split(",").collect::<Vec<&str>>()[1]
                .parse::<f64>()
                .unwrap(),
        };
        let end = Point {
            x: input_vec[1].split(",").collect::<Vec<&str>>()[0]
                .parse::<f64>()
                .unwrap(),
            y: input_vec[1].split(",").collect::<Vec<&str>>()[1]
                .parse::<f64>()
                .unwrap(),
        };
        segments.push(Segment { start, end });
        input.clear();
    }

    let mut closest_segment = None;
    let mut closest_distance = std::f64::INFINITY;
    for segment in segments {
        if half_line_segment_intersect(&half_line_start, &half_line_end, &segment) {
            let distance = ((segment.start.x - half_line_start.x).powi(2)
                + (segment.start.y - half_line_start.y).powi(2))
            .sqrt();
            if distance < closest_distance {
                closest_distance = distance;
                closest_segment = Some(segment);
            }
        }
    }

    if let Some(segment) = closest_segment {
        println!(
            "({}, {}) ({}, {})",
            segment.start.x, segment.start.y, segment.end.x, segment.end.y
        );
    } else {
        println!();
    }
}
