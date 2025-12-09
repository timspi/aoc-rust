use std::{i64, time::Instant};

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    println!("{}   ({} us)", run(input), now.elapsed().as_micros());
}

fn run(input: &str) -> String {
    let corners: Vec<(i64, i64)> = input
        .lines()
        .map(|l| {
            let mut split = l.split(',');
            (
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();
    
    // Visually inspect dataset
    // _render_image(&corners, 1000);

    // The data traces a circle with a horizontal cutout in the center,
    // so the biggest area is either a rect in the upper or lower half
    // Cutout:
    // 1537,50158
    // 94553,50158
    // 94553,48602
    // 1655,48602

    let (upper_corners, lower_corners): (Vec<_>, Vec<_>) = corners.into_iter().partition(|(_x, y)| *y < 50_000);
    let mut max = 0;

    let (upper_left, upper_right): (Vec<_>, Vec<_>) = upper_corners.into_iter().partition(|(x, _y)| *x < 50_000);
    let (lower_left, lower_right): (Vec<_>, Vec<_>) = lower_corners.into_iter().partition(|(x, _y)| *x < 50_000);

    // println!("upper");
    // From dataset
    let right = 94553;
    let bottom = 48602;
    for (left, top) in &upper_left {
        // Check if any corner in upper right quadrant is inside rect
        if upper_right.iter().any(|(corner_right, corner_top)| *corner_right < right && *corner_top > *top) {
            continue;
        }
        // Check if any corner in upper left quadrant is inside rect
        if upper_left.iter().any(|(corner_left, corner_top)| *corner_left > *left && *corner_top < *top) {
            continue;
        }
        let area = (bottom - top + 1) * (right - left + 1);
        // println!("found rect {left},{top} to {right},{bottom} with area {area}");
        if area > max {
            max = area;
        }
    }

    // println!("lower");
    // From dataset
    let right = 94553;
    let top = 50158;
    for (left, bottom) in &lower_left {
        // Check if any corner in lower right quadrant is inside rect
        if lower_right.iter().any(|(corner_right, corner_bottom)| *corner_right < right && *corner_bottom < *bottom) {
            continue;
        }
        // Check if any corner in lower left quadrant is inside rect
        if lower_left.iter().any(|(corner_left, corner_bottom)| *corner_left > *left && *corner_bottom < *bottom) {
            continue;
        }
        let area = (bottom - top + 1) * (right - left + 1);
        // println!("found rect {left},{bottom} to {right},{top} with area {area}");
        if area > max {
            max = area;
        }
    }

    max.to_string()
}

fn _render_image(corners: &Vec<(i64, i64)>, size: i64) {
    let mut imgbuf = image::ImageBuffer::new(size as u32, size as u32);
    let divider = 100_000 / size;
    let mut iter = corners.iter().map(|(x, y)| (x / divider, y / divider));
    let mut last = iter.next().unwrap();

    for (x, y) in iter {
        if last.0 != x {
            for ix in x.min(last.0)..=x.max(last.0) {
                let pixel = imgbuf.get_pixel_mut(ix as u32, y as u32);
                *pixel = image::Rgb([255_u8, 255, 255]);
            }
        } else {
            for iy in y.min(last.1)..=y.max(last.1) {
                let pixel = imgbuf.get_pixel_mut(x as u32, iy as u32);
                *pixel = image::Rgb([255_u8, 255, 255]);
            }
        }
        last = (x, y);
    }

    imgbuf.save("floor.png").unwrap();
}
