use std::fs::File;

#[derive(Debug)]
struct RGB(u8, u8, u8);

impl PartialEq for RGB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

#[allow(dead_code)]
const COLORS: [RGB; 10] = [
    RGB(255, 141, 139),
    RGB(254, 214, 137),
    RGB(136, 255, 137),
    RGB(135, 255, 255),
    RGB(139, 181, 254),
    RGB(215, 140, 255),
    RGB(255, 140, 255),
    RGB(255, 104, 247),
    RGB(254, 108, 183),
    RGB(255, 105, 104),
];

fn main() {
    let decoder = png::Decoder::new(File::open("./input/crab.png").unwrap());

    let (info, mut reader) = decoder.read_info().unwrap();

    let mut buffer = vec![0; info.buffer_size()];

    reader.next_frame(&mut buffer).unwrap();

    let mut encoder = gif::Encoder::new(
        File::create("./output/crab.gif").unwrap(),
        info.width as u16,
        info.height as u16,
        &[],
    )
    .unwrap();

    // TODO
    // for party_color in COLORS.iter() {
    //     for idx in [0..buffer.len()].iter().step_by(3) {
    //         let mut pixel = &mut buffer[idx..idx + 3];

    //         let rgb_pixel = transformPixel(&RGB(pixel[0], pixel[1], pixel[2]), party_color, 60);

    //         pixel[0] = rgb_pixel.0;
    //     }
    // }

    let frame = gif::Frame::from_rgb(info.width as u16, info.height as u16, &buffer);

    encoder.write_frame(&frame).unwrap();
}

fn mix(color: &RGB, overlayed_color: &RGB, opacity: u8) -> RGB {
    RGB(
        (overlayed_color.0 - color.0) * (opacity / 100) + color.0,
        (overlayed_color.1 - color.1) * (opacity / 100) + color.1,
        (overlayed_color.2 - color.2) * (opacity / 100) + color.2,
    )
}

#[cfg(test)]
mod test_grayscale {
    // import classes and functions outside this module
    use super::*;

    #[test]
    fn test_grayscale_black() {
        let color = RGB(255, 255, 255);
        let expected = grayscale(&color);
        assert_eq!(expected, RGB(255, 255, 255));
    }

    #[test]
    fn test_grayscale_white() {
        let color = RGB(0, 0, 0);
        let expected = grayscale(&color);
        assert_eq!(expected, RGB(0, 0, 0));
    }

    #[test]
    fn test_grayscale_red() {
        let color = RGB(255, 0, 0);
        let expected = grayscale(&color);
        assert_eq!(expected, RGB(53, 53, 53));
    }

    #[test]
    fn test_grayscale_green() {
        let color = RGB(0, 255, 0);
        let expected = grayscale(&color);
        assert_eq!(expected, RGB(183, 183, 183));
    }

    #[test]
    fn test_grayscale_blue() {
        let color = RGB(0, 0, 255);
        let expected = grayscale(&color);
        assert_eq!(expected, RGB(17, 17, 17));
    }
}

#[cfg(test)]
mod test_mix {
    // import classes and functions outside this module
    use super::*;

    #[test]
    fn test_mix_with_opaque() {
        let black = RGB(255, 255, 255);
        let white = RGB(0, 0, 0);
        let opacity = 100;
        let expected = mix(&white, &black, opacity);
        assert_eq!(expected, RGB(255, 255, 255));
    }

    #[test]
    fn test_mix_with_translucent() {
        let black = RGB(255, 255, 255);
        let white = RGB(0, 0, 0);
        let opacity = 0;
        let expected = mix(&white, &black, opacity);
        assert_eq!(expected, RGB(0, 0, 0));
    }
}

// Based on luminosity grayscale here: https://docs.gimp.org/2.6/en/gimp-tool-desaturate.html
fn grayscale(color: &RGB) -> RGB {
    let gray_level = (0.21 * color.0 as f32 + 0.72 * color.1 as f32 + 0.07 * color.2 as f32) as u8;
    RGB(gray_level, gray_level, gray_level)
}

fn transformPixel(pixel: &RGB, party_color: &RGB, opacity: u8) -> RGB {
    mix(&grayscale(pixel), party_color, opacity)
}
