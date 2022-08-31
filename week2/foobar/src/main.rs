pub struct Grid {
    num_rows: usize,
    num_cols: usize,
    elems: Vec<usize>,
}

impl Grid {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        // self.num_rows = num_rows;
        Grid {
            num_rows: num_rows,
            num_cols: num_cols,
            elems: vec![0, num_rows * num_cols],
        }
    }
    pub fn show(&mut self) -> Vec<usize> {
        // let u = self.elems;
        self.elems = vec![0; 111];
        // self.elems
        vec![0; 111]
    }
}

fn main() {
    let w = Grid::new(10, 10);
    // println!("{}", w.show());
    let t = w.num_cols;
    let u = w.num_cols;
    // let s = String::from("Hello, world!");
    // println!("{}", s);
    // println!("{}", s);
}
