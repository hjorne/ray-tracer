use super::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    grid: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            grid: vec![vec![Default::default(); width]; height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.grid[y][x] = color.clone();
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &Color {
        &self.grid[y][x]
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::from("");
        let mut line_len = 0;
        ppm.push_str("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");
        for row in self.grid.iter() {
            for c in row {
                let cstr = c.to_string();
                if line_len + cstr.len() < 70 {
                    if &ppm[ppm.len() - 1..] != "\n" {
                        ppm.push(' ');
                        line_len += 1;
                    }
                    ppm.push_str(&cstr);
                    line_len += cstr.len();
                } else {
                    ppm.push('\n');
                    ppm.push_str(&cstr);
                    line_len = cstr.len();
                }
            }
            ppm.push('\n');
            line_len = 0;
        }
        ppm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_constr_test() {
        let canvas = Canvas::new(10, 20);
        let black = Color::new(0.0, 0.0, 0.0);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        for i in 0..canvas.width {
            for j in 0..canvas.height {
                assert_eq!(canvas.get_pixel(i, j), &black);
            }
        }
    }

    #[test]
    fn canvas_rw_test() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, &red);
        assert_eq!(canvas.get_pixel(2, 3), &red);
    }

    #[test]
    fn canvas_ppm_header_test() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        let split: Vec<_> = ppm.split('\n').collect();
        assert_eq!(split[0], "P3");
        assert_eq!(split[1], "5 3");
        assert_eq!(split[2], "255");
    }

    #[test]
    fn canvas_ppm_full_test() {
        let mut canvas = Canvas::new(5, 3);
        canvas.write_pixel(0, 0, &Color::new(1.5, 0.0, 0.0));
        canvas.write_pixel(2, 1, &Color::new(0.0, 0.5, 0.0));
        canvas.write_pixel(4, 2, &Color::new(-0.5, 0.0, 1.0));
        let ppm = canvas.to_ppm();
        let split: Vec<_> = ppm.split('\n').collect();
        assert_eq!(split[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(split[4], "0 0 0 0 0 0 0 127 0 0 0 0 0 0 0");
        assert_eq!(split[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn canvas_ppm_long_test() {
        let mut canvas = Canvas::new(10, 2);

        for i in 0..canvas.width {
            for j in 0..canvas.height {
                canvas.write_pixel(i, j, &Color::new(1.0, 0.8, 0.6));
            }
        }

        let ppm = canvas.to_ppm();
        let split: Vec<_> = ppm.split('\n').collect();
        for line in split {
            assert!(line.len() < 70);
        }
    }

    #[test]
    fn canvas_ppm_newline_test() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        assert_eq!(&ppm[ppm.len() - 1..], "\n");
    }
}
