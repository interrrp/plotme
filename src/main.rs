use evalexpr::{Context, DefaultNumericTypes, build_operator_tree, context_map};
use std::env::args;

fn main() {
    let expr_str = args().nth(1).unwrap();
    let expr = build_operator_tree::<DefaultNumericTypes>(&expr_str).unwrap();

    let mut screen = Screen::new(128, 64);

    let hsw = (screen.width / 2) as isize;
    let hsh = (screen.height / 2) as isize;

    for y in -hsh..=hsh {
        for x in -hsw..=hsw {
            let mut ctx = context_map! {
                "x" => float (x as f64),
                "y" => float (y as f64),
            }
            .unwrap();

            expr.eval_with_context_mut(&mut ctx).unwrap();

            let x_val = hsw as f64 + ctx.get_value("x").unwrap().as_float().unwrap();
            let y_val = hsh as f64 - ctx.get_value("y").unwrap().as_float().unwrap();

            if x_val <= 0.0
                || x_val >= screen.width as f64
                || y_val <= 0.0
                || y_val >= screen.height as f64
            {
                continue;
            }

            screen.set(x_val as usize, y_val as usize, true);
        }
    }

    screen.render();
}

struct Screen {
    width: usize,
    height: usize,
    buf: Vec<bool>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        let mut s = Screen {
            width,
            height,
            buf: Vec::new(),
        };

        for _ in 0..width * height {
            s.buf.push(false);
        }

        s
    }

    fn set(&mut self, x: usize, y: usize, on: bool) {
        self.buf[y * self.width + x] = on;
    }

    fn render(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match (x, y) {
                    (x, y) if self.buf[y * self.width + x] => print!("▀"),
                    (x, y) if x == self.width / 2 && y == self.height / 2 => print!("┼"),
                    (0, 0) => print!("┌"),
                    (x, 0) if x == self.width - 1 => print!("┐"),
                    (0, y) if y == self.height - 1 => print!("└"),
                    (x, y) if x == self.width - 1 && y == self.height - 1 => print!("┘"),
                    (0, y) if y == self.height / 2 => print!("├"),
                    (x, y) if x == self.width - 1 && y == self.height / 2 => print!("┤"),
                    (x, 0) if x == self.width / 2 => print!("┬"),
                    (x, y) if x == self.width / 2 && y == self.height - 1 => print!("┴"),
                    (_, y) if y == self.height / 2 || y == 0 || y == self.height - 1 => print!("─"),
                    (x, _) if x == self.width / 2 || x == 0 || x == self.width - 1 => print!("│"),
                    _ => print!(" "),
                }
            }
            println!();
        }
    }
}
