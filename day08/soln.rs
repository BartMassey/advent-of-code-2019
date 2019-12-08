// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 8.  
//! Bart Massey 2019

/// Width and height from puzzle description.
const DIM: (usize, usize) = (25, 6);

/// A histogram containing the count of '0', '1' and '2'
/// pixels.
type Hist = [u64; 3];

/// Find the histogram of the layer with the minimum number
/// of zeros.
fn hist_min_zeros(
    (width, height): (usize, usize),
    image: &str,
) -> Hist {
    // Unfortunately, Rust does not include an iterator
    // method that returns Vecs of cloned chunks. So we have
    // to build a slice of chars to use its `chunks()`
    // method.
    let image: Vec<char> = image.chars().collect();
    image
        .chunks(width * height)
        .map(|pixels| {
            let mut hist: Hist = [0; 3];
            for p in pixels {
                // The test data given in the problem
                // contains out-of-range pixels. So we
                // ignore them rather than panicing here.
                if let '0'..='2' = *p {
                    hist[(*p as u8 - b'0') as usize] += 1;
                }
            }
            hist
        })
        .min_by_key(|hist| hist[0])
        .expect("no hists")
}

// Test from problem description.
#[test]
fn test_hist_min_zeros() {
    let image = "123456789012";
    assert_eq!([0, 1, 1], hist_min_zeros((3, 2), image));
}

/// Produce a string rendering of the given image as
/// described in the problem. The string will contain
/// newlines as needed, including a trailing newline.
fn render_image(
    (width, height): (usize, usize),
    image: &str,
) -> String {
    // Set up the "top" plane as transparent.  Will panic
    // later if this makes it all the way through to the
    // bottom.
    let nplane = width * height;
    let mut render: Vec<char> = Vec::with_capacity(nplane);
    render.resize(nplane, '2');

    // Process the planes from top-to-bottom ("Z
    // buffering").  This is more efficient that going
    // bottom-to-top (although not interestingly so) since
    // we don't have to reverse the planes.
    let image: Vec<char> = image.chars().collect();
    for c in image.chunks(nplane) {
        for (i, p) in c.iter().enumerate() {
            match render[i] {
                '0' | '1' => (),
                '2' => render[i] = *p,
                _ => panic!("illegal pixel in render"),
            }
        }
    }

    // Transform the render into a string.
    let mut result = String::with_capacity(width + nplane);
    for (i, p) in render.iter().enumerate() {
        match *p {
            '0' => result.push(' '),
            '1' => result.push('*'),
            '2' => panic!("transparent pixel in output"),
            _ => panic!("illegal pixel in output"),
        }
        if i % width == width - 1 {
            result.push('\n');
        }
    }
    result
}

// Test from problem description.
#[test]
fn test_render_image() {
    let image = "0222112222120000";
    let render = render_image((2, 2), image);
    assert_eq!(" *\n* \n", render);
}

pub fn main() {
    let image = aoc::input_line();
    let image = image.trim_end();
    let part = aoc::get_part();
    match part {
        aoc::Part1 => {
            let h = hist_min_zeros(DIM, image);
            println!("{:?}", h[1] * h[2]);
        }
        aoc::Part2 => {
            // Not `println!()`, because the render
            // already contains a trailing newline.
            print!("{}", render_image(DIM, image));
        }
    }
}
