use evalexpr::{DefaultNumericTypes, Value, build_operator_tree, context_map};
use std::env::args;

fn main() {
    let expr_str = args().nth(1).unwrap();
    let expr = build_operator_tree::<DefaultNumericTypes>(&expr_str).unwrap();

    let mut screen = Screen::new(64, 32);

    let hsw = (screen.width / 2) as isize;
    let hsh = (screen.height / 2) as isize;
    let true_val = Value::from(true);

    for y in -hsh..=hsh {
        for x in -hsw..=hsw {
            let ctx = context_map! {
                "x" => float (x as f64),
                "y" => float (y as f64),
            }
            .unwrap();

            if expr.eval_with_context(&ctx).unwrap() == true_val {
                screen.set((x + hsw) as usize, (y + hsh) as usize, true);
            }
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
        Screen {
            width,
            height,
            buf: vec![false; (width * height) as usize],
        }
    }

    fn set(&mut self, x: usize, y: usize, on: bool) {
        self.buf[y * self.width + x] = on;
    }

    fn render(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.render_pixel(x, y);
            }
            println!();
        }
    }

    fn render_pixel(&self, x: usize, y: usize) {
        match (x, y) {
            (x, y) if self.buf[y * self.width + x] => print!("█"),
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
}
