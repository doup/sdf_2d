use std::{collections::HashMap, fs::File};

#[derive(Clone, Debug)]
pub struct Char {
    pub code: u8,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub x_advance: f32,
    pub x_offset: f32,
    pub y: f32,
    pub y_offset: f32,
}

#[derive(Debug)]
pub struct Font {
    pub size: f32,
    pub line_height: f32,
    pub base: f32,
    pub scale_width: f32,
    pub scale_height: f32,
    pub chars: HashMap<u8, Char>,
    pub image: Vec<u8>,
}

impl Font {
    pub fn get_char(&self, letter: char) -> &Char {
        let placeholder = self.chars.get(&63); // '?' character
        let char = self.chars.get(&(letter as u8));

        match char {
            Some(char) => char,
            None => placeholder.unwrap()
        }
    }
}

fn load_png(path: String) -> (Vec<u8>, u32, u32) {
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::EXPAND
    // | Transformations::STRIP_ALPHA`.
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    reader.next_frame(&mut buf).unwrap();

    // Only use alpha channel, discard the others
    let image = buf.chunks(4).map(|chunk| chunk[3]).collect::<Vec<u8>>();

    (image, info.width, info.height)
}

pub fn get_comic_sans() -> Font {
    let (image, width, height) = load_png(String::from("comic-sans.png"));
    let mut comic_sans = Font {
        size: 32.0,
        line_height: 46.0,
        base: 36.0,
        scale_width: width as f32,
        scale_height: height as f32,
        chars: HashMap::new(),
        image: image,
    };

    // Space
    comic_sans.chars.insert(32, Char {
        code: 32,
        width: 0.0,
        height: 0.0,
        x: 0.0,
        x_advance: 12.0,
        x_offset: 0.0,
        y: 0.0,
        y_offset: 0.0,
    });

    comic_sans.chars.insert(10, Char {
        code: 10,
        width: 0.0,
        height: 0.0,
        x: 0.0,
        x_advance: 10.0,
        x_offset: -8.0,
        y: 0.0,
        y_offset: 0.0,
    });

    comic_sans.chars.insert(33, Char {
        code: 33,
        width: 21.0,
        height: 44.0,
        x: 419.0,
        x_advance: 8.0,
        x_offset: -7.0,
        y: 0.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(34, Char {
        code: 34,
        width: 26.0,
        height: 27.0,
        x: 230.0,
        x_advance: 14.0,
        x_offset: -7.0,
        y: 218.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(35, Char {
        code: 35,
        width: 43.0,
        height: 42.0,
        x: 135.0,
        x_advance: 27.0,
        x_offset: -8.0,
        y: 135.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(36, Char {
        code: 36,
        width: 35.0,
        height: 50.0,
        x: 28.0,
        x_advance: 22.0,
        x_offset: -7.0,
        y: 0.0,
        y_offset: 1.0,
    });

    comic_sans.chars.insert(37, Char {
        code: 37,
        width: 40.0,
        height: 43.0,
        x: 359.0,
        x_advance: 26.0,
        x_offset: -6.0,
        y: 50.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(38, Char {
        code: 38,
        width: 35.0,
        height: 43.0,
        x: 399.0,
        x_advance: 21.0,
        x_offset: -7.0,
        y: 50.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(39, Char {
        code: 39,
        width: 20.0,
        height: 25.0,
        x: 256.0,
        x_advance: 12.0,
        x_offset: -4.0,
        y: 218.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(40, Char {
        code: 40,
        width: 26.0,
        height: 49.0,
        x: 63.0,
        x_advance: 12.0,
        x_offset: -7.0,
        y: 0.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(41, Char {
        code: 41,
        width: 26.0,
        height: 49.0,
        x: 89.0,
        x_advance: 12.0,
        x_offset: -7.0,
        y: 0.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(42, Char {
        code: 42,
        width: 32.0,
        height: 29.0,
        x: 169.0,
        x_advance: 17.0,
        x_offset: -8.0,
        y: 218.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(43, Char {
        code: 43,
        width: 31.0,
        height: 30.0,
        x: 138.0,
        x_advance: 15.0,
        x_offset: -8.0,
        y: 218.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(44, Char {
        code: 44,
        width: 21.0,
        height: 25.0,
        x: 276.0,
        x_advance: 9.0,
        x_offset: -5.0,
        y: 218.0,
        y_offset: 25.0,
    });

    comic_sans.chars.insert(45, Char {
        code: 45,
        width: 27.0,
        height: 19.0,
        x: 442.0,
        x_advance: 13.0,
        x_offset: -7.0,
        y: 218.0,
        y_offset: 18.0,
    });

    comic_sans.chars.insert(46, Char {
        code: 46,
        width: 21.0,
        height: 21.0,
        x: 383.0,
        x_advance: 8.0,
        x_offset: -6.0,
        y: 218.0,
        y_offset: 25.0,
    });

    comic_sans.chars.insert(47, Char {
        code: 47,
        width: 31.0,
        height: 44.0,
        x: 440.0,
        x_advance: 16.0,
        x_offset: -7.0,
        y: 0.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(48, Char {
        code: 48,
        width: 35.0,
        height: 42.0,
        x: 68.0,
        x_advance: 20.0,
        x_offset: -8.0,
        y: 135.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(49, Char {
        code: 49,
        width: 27.0,
        height: 42.0,
        x: 393.0,
        x_advance: 14.0,
        x_offset: -6.0,
        y: 93.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(50, Char {
        code: 50,
        width: 32.0,
        height: 41.0,
        x: 463.0,
        x_advance: 20.0,
        x_offset: -6.0,
        y: 135.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(51, Char {
        code: 51,
        width: 31.0,
        height: 41.0,
        x: 0.0,
        x_advance: 20.0,
        x_offset: -6.0,
        y: 177.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(52, Char {
        code: 52,
        width: 35.0,
        height: 42.0,
        x: 420.0,
        x_advance: 20.0,
        x_offset: -8.0,
        y: 93.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(53, Char {
        code: 53,
        width: 34.0,
        height: 42.0,
        x: 455.0,
        x_advance: 20.0,
        x_offset: -7.0,
        y: 93.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(54, Char {
        code: 54,
        width: 33.0,
        height: 43.0,
        x: 296.0,
        x_advance: 20.0,
        x_offset: -7.0,
        y: 50.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(55, Char {
        code: 55,
        width: 34.0,
        height: 42.0,
        x: 0.0,
        x_advance: 20.0,
        x_offset: -7.0,
        y: 135.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(56, Char {
        code: 56,
        width: 33.0,
        height: 41.0,
        x: 31.0,
        x_advance: 20.0,
        x_offset: -7.0,
        y: 177.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(57, Char {
        code: 57,
        width: 34.0,
        height: 42.0,
        x: 34.0,
        x_advance: 20.0,
        x_offset: -7.0,
        y: 135.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(58, Char {
        code: 58,
        width: 21.0,
        height: 33.0,
        x: 63.0,
        x_advance: 10.0,
        x_offset: -6.0,
        y: 218.0,
        y_offset: 10.0,
    });

    comic_sans.chars.insert(59, Char {
        code: 59,
        width: 22.0,
        height: 38.0,
        x: 129.0,
        x_advance: 10.0,
        x_offset: -7.0,
        y: 177.0,
        y_offset: 10.0,
    });

    comic_sans.chars.insert(60, Char {
        code: 60,
        width: 26.0,
        height: 30.0,
        x: 112.0,
        x_advance: 12.0,
        x_offset: -8.0,
        y: 218.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(61, Char {
        code: 61,
        width: 29.0,
        height: 28.0,
        x: 201.0,
        x_advance: 16.0,
        x_offset: -7.0,
        y: 218.0,
        y_offset: 12.0,
    });

    comic_sans.chars.insert(62, Char {
        code: 62,
        width: 28.0,
        height: 32.0,
        x: 84.0,
        x_advance: 12.0,
        x_offset: -8.0,
        y: 218.0,
        y_offset: 10.0,
    });

    comic_sans.chars.insert(63, Char {
        code: 63,
        width: 32.0,
        height: 42.0,
        x: 103.0,
        x_advance: 17.0,
        x_offset: -8.0,
        y: 135.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(64, Char {
        code: 64,
        width: 43.0,
        height: 45.0,
        x: 282.0,
        x_advance: 30.0,
        x_offset: -7.0,
        y: 0.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(65, Char {
        code: 65,
        width: 36.0,
        height: 41.0,
        x: 178.0,
        x_advance: 23.0,
        x_offset: -6.0,
        y: 135.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(66, Char {
        code: 66,
        width: 33.0,
        height: 42.0,
        x: 434.0,
        x_advance: 20.0,
        x_offset: -6.0,
        y: 50.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(67, Char {
        code: 67,
        width: 34.0,
        height: 41.0,
        x: 214.0,
        x_advance: 19.0,
        x_offset: -7.0,
        y: 135.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(68, Char {
        code: 68,
        width: 36.0,
        height: 43.0,
        x: 471.0,
        x_advance: 23.0,
        x_offset: -6.0,
        y: 0.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(69, Char {
        code: 69,
        width: 33.0,
        height: 44.0,
        x: 325.0,
        x_advance: 20.0,
        x_offset: -6.0,
        y: 0.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(70, Char {
        code: 70,
        width: 33.0,
        height: 43.0,
        x: 0.0,
        x_advance: 19.0,
        x_offset: -6.0,
        y: 50.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(71, Char {
        code: 71,
        width: 37.0,
        height: 43.0,
        x: 33.0,
        x_advance: 22.0,
        x_offset: -7.0,
        y: 50.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(72, Char {
        code: 72,
        width: 37.0,
        height: 43.0,
        x: 70.0,
        x_advance: 25.0,
        x_offset: -6.0,
        y: 50.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(73, Char {
        code: 73,
        width: 32.0,
        height: 41.0,
        x: 248.0,
        x_advance: 17.0,
        x_offset: -7.0,
        y: 135.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(74, Char {
        code: 74,
        width: 36.0,
        height: 42.0,
        x: 467.0,
        x_advance: 21.0,
        x_offset: -7.0,
        y: 50.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(75, Char {
        code: 75,
        width: 33.0,
        height: 42.0,
        x: 0.0,
        x_advance: 20.0,
        x_offset: -5.0,
        y: 93.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(76, Char {
        code: 76,
        width: 33.0,
        height: 43.0,
        x: 107.0,
        x_advance: 18.0,
        x_offset: -7.0,
        y: 50.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(77, Char {
        code: 77,
        width: 43.0,
        height: 42.0,
        x: 33.0,
        x_advance: 28.0,
        x_offset: -7.0,
        y: 93.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(78, Char {
        code: 78,
        width: 40.0,
        height: 43.0,
        x: 140.0,
        x_advance: 26.0,
        x_offset: -7.0,
        y: 50.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(79, Char {
        code: 79,
        width: 40.0,
        height: 41.0,
        x: 280.0,
        x_advance: 26.0,
        x_offset: -7.0,
        y: 135.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(80, Char {
        code: 80,
        width: 31.0,
        height: 42.0,
        x: 76.0,
        x_advance: 17.0,
        x_offset: -7.0,
        y: 93.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(81, Char {
        code: 81,
        width: 43.0,
        height: 47.0,
        x: 189.0,
        x_advance: 28.0,
        x_offset: -7.0,
        y: 0.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(82, Char {
        code: 82,
        width: 35.0,
        height: 42.0,
        x: 107.0,
        x_advance: 20.0,
        x_offset: -7.0,
        y: 93.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(83, Char {
        code: 83,
        width: 35.0,
        height: 40.0,
        x: 64.0,
        x_advance: 22.0,
        x_offset: -6.0,
        y: 177.0,
        y_offset: 5.0,
    });

    comic_sans.chars.insert(84, Char {
        code: 84,
        width: 38.0,
        height: 41.0,
        x: 320.0,
        x_advance: 22.0,
        x_offset: -7.0,
        y: 135.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(85, Char {
        code: 85,
        width: 36.0,
        height: 41.0,
        x: 358.0,
        x_advance: 24.0,
        x_offset: -6.0,
        y: 135.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(86, Char {
        code: 86,
        width: 35.0,
        height: 42.0,
        x: 142.0,
        x_advance: 21.0,
        x_offset: -6.0,
        y: 93.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(87, Char {
        code: 87,
        width: 47.0,
        height: 42.0,
        x: 177.0,
        x_advance: 33.0,
        x_offset: -6.0,
        y: 93.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(88, Char {
        code: 88,
        width: 37.0,
        height: 42.0,
        x: 224.0,
        x_advance: 23.0,
        x_offset: -7.0,
        y: 93.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(89, Char {
        code: 89,
        width: 36.0,
        height: 42.0,
        x: 261.0,
        x_advance: 20.0,
        x_offset: -8.0,
        y: 93.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(90, Char {
        code: 90,
        width: 37.0,
        height: 41.0,
        x: 394.0,
        x_advance: 22.0,
        x_offset: -7.0,
        y: 135.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(91, Char {
        code: 91,
        width: 25.0,
        height: 47.0,
        x: 232.0,
        x_advance: 12.0,
        x_offset: -6.0,
        y: 0.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(92, Char {
        code: 92,
        width: 30.0,
        height: 43.0,
        x: 329.0,
        x_advance: 18.0,
        x_offset: -6.0,
        y: 50.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(93, Char {
        code: 93,
        width: 25.0,
        height: 47.0,
        x: 257.0,
        x_advance: 12.0,
        x_offset: -6.0,
        y: 0.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(94, Char {
        code: 94,
        width: 29.0,
        height: 25.0,
        x: 297.0,
        x_advance: 19.0,
        x_offset: -5.0,
        y: 218.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(95, Char {
        code: 95,
        width: 38.0,
        height: 20.0,
        x: 404.0,
        x_advance: 20.0,
        x_offset: -9.0,
        y: 218.0,
        y_offset: 30.0,
    });

    comic_sans.chars.insert(96, Char {
        code: 96,
        width: 24.0,
        height: 24.0,
        x: 326.0,
        x_advance: 18.0,
        x_offset: -6.0,
        y: 218.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(97, Char {
        code: 97,
        width: 32.0,
        height: 35.0,
        x: 221.0,
        x_advance: 16.0,
        x_offset: -8.0,
        y: 177.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(98, Char {
        code: 98,
        width: 32.0,
        height: 42.0,
        x: 297.0,
        x_advance: 19.0,
        x_offset: -6.0,
        y: 93.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(99, Char {
        code: 99,
        width: 31.0,
        height: 34.0,
        x: 447.0,
        x_advance: 16.0,
        x_offset: -7.0,
        y: 177.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(100, Char {
        code: 100,
        width: 33.0,
        height: 42.0,
        x: 329.0,
        x_advance: 19.0,
        x_offset: -7.0,
        y: 93.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(101, Char {
        code: 101,
        width: 32.0,
        height: 34.0,
        x: 478.0,
        x_advance: 18.0,
        x_offset: -7.0,
        y: 177.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(102, Char {
        code: 102,
        width: 30.0,
        height: 44.0,
        x: 358.0,
        x_advance: 16.0,
        x_offset: -7.0,
        y: 0.0,
        y_offset: 3.0,
    });

    comic_sans.chars.insert(103, Char {
        code: 103,
        width: 32.0,
        height: 41.0,
        x: 431.0,
        x_advance: 17.0,
        x_offset: -8.0,
        y: 135.0,
        y_offset: 12.0,
    });

    comic_sans.chars.insert(104, Char {
        code: 104,
        width: 31.0,
        height: 43.0,
        x: 180.0,
        x_advance: 18.0,
        x_offset: -6.0,
        y: 50.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(105, Char {
        code: 105,
        width: 21.0,
        height: 41.0,
        x: 489.0,
        x_advance: 9.0,
        x_offset: -6.0,
        y: 93.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(106, Char {
        code: 106,
        width: 28.0,
        height: 50.0,
        x: 0.0,
        x_advance: 13.0,
        x_offset: -9.0,
        y: 0.0,
        y_offset: 4.0,
    });

    comic_sans.chars.insert(107, Char {
        code: 107,
        width: 31.0,
        height: 43.0,
        x: 211.0,
        x_advance: 17.0,
        x_offset: -6.0,
        y: 50.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(108, Char {
        code: 108,
        width: 21.0,
        height: 43.0,
        x: 242.0,
        x_advance: 9.0,
        x_offset: -6.0,
        y: 50.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(109, Char {
        code: 109,
        width: 39.0,
        height: 36.0,
        x: 151.0,
        x_advance: 25.0,
        x_offset: -7.0,
        y: 177.0,
        y_offset: 10.0,
    });

    comic_sans.chars.insert(110, Char {
        code: 110,
        width: 31.0,
        height: 36.0,
        x: 190.0,
        x_advance: 17.0,
        x_offset: -7.0,
        y: 177.0,
        y_offset: 10.0,
    });

    comic_sans.chars.insert(111, Char {
        code: 111,
        width: 31.0,
        height: 34.0,
        x: 0.0,
        x_advance: 17.0,
        x_offset: -7.0,
        y: 218.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(112, Char {
        code: 112,
        width: 31.0,
        height: 44.0,
        x: 388.0,
        x_advance: 17.0,
        x_offset: -7.0,
        y: 0.0,
        y_offset: 10.0,
    });

    comic_sans.chars.insert(113, Char {
        code: 113,
        width: 31.0,
        height: 42.0,
        x: 362.0,
        x_advance: 17.0,
        x_offset: -8.0,
        y: 93.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(114, Char {
        code: 114,
        width: 29.0,
        height: 35.0,
        x: 253.0,
        x_advance: 15.0,
        x_offset: -6.0,
        y: 177.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(115, Char {
        code: 115,
        width: 31.0,
        height: 35.0,
        x: 282.0,
        x_advance: 16.0,
        x_offset: -8.0,
        y: 177.0,
        y_offset: 10.0,
    });

    comic_sans.chars.insert(116, Char {
        code: 116,
        width: 30.0,
        height: 39.0,
        x: 99.0,
        x_advance: 15.0,
        x_offset: -7.0,
        y: 177.0,
        y_offset: 6.0,
    });

    comic_sans.chars.insert(117, Char {
        code: 117,
        width: 31.0,
        height: 35.0,
        x: 313.0,
        x_advance: 17.0,
        x_offset: -7.0,
        y: 177.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(118, Char {
        code: 118,
        width: 32.0,
        height: 34.0,
        x: 31.0,
        x_advance: 16.0,
        x_offset: -8.0,
        y: 218.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(119, Char {
        code: 119,
        width: 37.0,
        height: 35.0,
        x: 344.0,
        x_advance: 22.0,
        x_offset: -7.0,
        y: 177.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(120, Char {
        code: 120,
        width: 34.0,
        height: 35.0,
        x: 381.0,
        x_advance: 19.0,
        x_offset: -8.0,
        y: 177.0,
        y_offset: 10.0,
    });

    comic_sans.chars.insert(121, Char {
        code: 121,
        width: 33.0,
        height: 43.0,
        x: 263.0,
        x_advance: 17.0,
        x_offset: -9.0,
        y: 50.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(122, Char {
        code: 122,
        width: 32.0,
        height: 35.0,
        x: 415.0,
        x_advance: 17.0,
        x_offset: -7.0,
        y: 177.0,
        y_offset: 11.0,
    });

    comic_sans.chars.insert(123, Char {
        code: 123,
        width: 27.0,
        height: 48.0,
        x: 135.0,
        x_advance: 12.0,
        x_offset: -8.0,
        y: 0.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(124, Char {
        code: 124,
        width: 20.0,
        height: 49.0,
        x: 115.0,
        x_advance: 13.0,
        x_offset: -3.0,
        y: 0.0,
        y_offset: 1.0,
    });

    comic_sans.chars.insert(125, Char {
        code: 125,
        width: 27.0,
        height: 48.0,
        x: 162.0,
        x_advance: 12.0,
        x_offset: -8.0,
        y: 0.0,
        y_offset: 2.0,
    });

    comic_sans.chars.insert(126, Char {
        code: 126,
        width: 33.0,
        height: 24.0,
        x: 350.0,
        x_advance: 19.0,
        x_offset: -7.0,
        y: 218.0,
        y_offset: 13.0,
    });

    comic_sans
}
