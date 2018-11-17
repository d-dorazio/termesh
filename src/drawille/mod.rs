//! Small subset of the awesome [drawille](https://github.com/asciimoo/drawille)
//! library. The way this thing works is pretty clever: Unicode supports Braille
//! characters which essentially are dots in a 4 (rows) x 2 (columns) matrix and
//! we can use these dots patterns to emulate corners, slope, density, etc...
//! The best part is that the elements in the palette are composable by "or-ing"
//! the offsets, for example composing a high left dot with a left middle dot
//! gives the character that has a two dot vertical bar on the left!
//!
//! Since each character is a 4x2 matrix the user coordinates are remapped in
//! 4x2 cells.
//!

pub mod line;

use std::collections::BTreeMap;

use crate::utils::btree_minmax;
use crate::vector3::Vector3;

static BRAILLE_PATTERN_BLANK: char = '\u{2800}';
static BRAILLE_OFFSET_MAP: [[u8; 2]; 4] = [
    [0x01, 0x08], // "⠁" , "⠈"
    [0x02, 0x10], // "⠂" , "⠐"
    [0x04, 0x20], // "⠄" , "⠐"
    [0x40, 0x80], // "⡀" , "⢀"
];

fn canvas_pos(x: f32, y: f32) -> (i32, i32) {
    (
        (x.round() / 2.0).floor() as i32,
        (y.round() / 4.0).floor() as i32,
    )
}

fn braille_offset_at(x: f32, y: f32) -> u8 {
    let mut xoff = x.round() % 2.0;
    if xoff < 0.0 {
        xoff += 2.0;
    }

    let mut yoff = y.round() % 4.0;
    if yoff < 0.0 {
        yoff += 4.0;
    }

    BRAILLE_OFFSET_MAP[yoff as usize][xoff as usize]
}

#[derive(Debug)]
pub struct Canvas {
    rows: BTreeMap<i32, BTreeMap<i32, u8>>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            rows: BTreeMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.rows.clear();
    }

    pub fn triangle(&mut self, p0: Vector3, p1: Vector3, p2: Vector3) {
        self.line(p0, p1);
        self.line(p0, p2);
        self.line(p1, p2);
    }

    pub fn line(&mut self, p0: Vector3, p1: Vector3) {
        for p in line::Line::new(p0.round(), p1.round()) {
            self.set(p);
        }
    }

    pub fn set(&mut self, p: Vector3) {
        let (c, r) = canvas_pos(p.x, p.y);

        *self.rows.entry(r).or_default().entry(c).or_default() |= braille_offset_at(p.x, p.y);
    }

    pub fn is_set(&self, x: f32, y: f32) -> bool {
        let dot_index = braille_offset_at(x, y);
        let (x, y) = canvas_pos(x, y);

        self.rows
            .get(&y)
            .and_then(|row| row.get(&x))
            .map_or(false, |c| c & dot_index != 0)
    }

    pub fn rows(&self) -> Rows {
        Rows::new(self)
    }
}

#[derive(Debug)]
pub struct Rows<'a> {
    canvas: &'a Canvas,
    min_row: i32,
    max_row: i32,
    min_col: i32,
}

impl<'a> Rows<'a> {
    fn new(canvas: &'a Canvas) -> Self {
        let (min_row, max_row, min_col) = match btree_minmax(&canvas.rows) {
            None => (i32::max_value(), 0, 0),
            Some((&min_row, &max_row)) => {
                let min_c = canvas
                    .rows
                    .values()
                    .flat_map(|row| btree_minmax(row).map(|(m, _)| m))
                    .min();

                // if btree_minmax(&canvas.rows) succeeded there's no way this can fail
                let &min_c = min_c.unwrap();

                (min_row, max_row, min_c)
            }
        };

        Self {
            canvas,
            min_row,
            max_row,
            min_col,
        }
    }
}

impl<'a> Iterator for Rows<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.min_row > self.max_row {
            return None;
        }

        let row = match self.canvas.rows.get(&self.min_row) {
            None => String::new(),
            Some(row) => match btree_minmax(row) {
                None => String::new(),
                Some((_, &max_c)) => (self.min_col..=max_c)
                    .map(|x| {
                        row.get(&x).map_or(BRAILLE_PATTERN_BLANK, |&off| {
                            std::char::from_u32(BRAILLE_PATTERN_BLANK as u32 + u32::from(off))
                                .unwrap()
                        })
                    })
                    .collect(),
            },
        };

        self.min_row += 1;

        Some(row)
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Canvas::new()
    }
}

#[cfg(test)]
mod tests {
    use maplit::btreemap;

    use super::{Canvas, Vector3};

    #[test]
    fn test_set() {
        let mut c = Canvas::new();

        c.set(Vector3::new(0.0, 0.0, 0.0));

        assert_eq!(
            c.rows,
            btreemap!{
                0 => btreemap!{0 => 1}
            }
        );
    }

    #[test]
    fn test_clear() {
        let mut c = Canvas::new();

        c.set(Vector3::new(1.0, 1.0, 0.0));
        c.clear();

        assert_eq!(c.rows, btreemap!{});
    }

    #[test]
    fn test_get() {
        let mut c = Canvas::new();

        assert!(!c.is_set(0.0, 0.0));

        c.set(Vector3::new(0.0, 0.0, 0.0));
        assert!(c.is_set(0.0, 0.0));
        assert!(!c.is_set(0.0, 1.0));
        assert!(!c.is_set(1.0, 0.0));
        assert!(!c.is_set(1.0, 1.0));
    }

    #[test]
    fn test_frame() {
        let mut c = Canvas::new();
        assert_eq!(c.rows().collect::<Vec<_>>(), Vec::<String>::new());

        c.set(Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(c.rows().collect::<Vec<_>>(), vec!["⠁".to_string()]);
    }

    #[test]
    fn test_rect() {
        let mut c = Canvas::new();

        c.line(Vector3::new(0.0, 0.0, 0.0), Vector3::new(20.0, 0.0, 0.0));
        c.line(Vector3::new(20.0, 0.0, 0.0), Vector3::new(20.0, 20.0, 0.0));
        c.line(Vector3::new(0.0, 20.0, 0.0), Vector3::new(20.0, 20.0, 0.0));
        c.line(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 20.0, 0.0));
        c.line(Vector3::new(0.0, 0.0, 0.0), Vector3::new(20.0, 20.0, 0.0));
        c.line(Vector3::new(20.0, 0.0, 0.0), Vector3::new(0.0, 20.0, 0.0));

        assert_eq!(
            c.rows().collect::<Vec<_>>(),
            vec![
                "⡟⢍⠉⠉⠉⠉⠉⠉⢉⠝⡇",
                "⡇⠀⠑⢄⠀⠀⢀⠔⠁⠀⡇",
                "⡇⠀⠀⠀⢑⢔⠁⠀⠀⠀⡇",
                "⡇⠀⢀⠔⠁⠀⠑⢄⠀⠀⡇",
                "⣇⠔⠁⠀⠀⠀⠀⠀⠑⢄⡇",
                "⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠁",
            ]
        );
    }

    #[test]
    fn test_sine_example() {
        let mut s = Canvas::new();

        for x in (0_u16..3600).step_by(20) {
            s.set(Vector3::new(
                f32::from(x) / 20.0,
                (4.0 + f32::from(x).to_radians().sin() * 4.0).round(),
                0.0,
            ));
        }

        let rows = s.rows().collect::<Vec<_>>();

        assert_eq!(rows, vec![
            "⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂⠀⠀⠀⠀⠀⡐⠊⠑⢂",
            "⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊⠀⠀⠀⠀⠑⣀⠀⣀⠊",
            "⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⠀⠀⠀⠀⠉",
            ]);
    }
}
