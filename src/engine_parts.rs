// inspsired by https://github.com/whiteand/advent-of-code/blob/18314d1973d407c88047f02899f0139a2bdd5480/src/y23/y23d03.rs
struct Number {
    row: usize,
    start_col: usize,
    end_col: usize,
    value: usize,
}

impl Number {
    fn new(row: usize, start_col: usize, value: usize) -> Self {
        Self {
            row,
            start_col,
            end_col: start_col,
            value,
        }
    }

    fn push(&mut self, ascii_digit: u8) {
        self.end_col += 1;
        self.value *= 10;
        self.value += (ascii_digit - b'0') as usize;
    }
}

struct Symbol {
    row: usize,
    col: usize,
}
impl Symbol {
    fn new(row: usize, col: usize) -> Symbol {
        Symbol { row, col }
    }
}

struct Map {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
    id_texture: Vec<Vec<usize>>,
    rows: usize,
    cols: usize,
}

fn symbol_index_to_id(index: usize) -> usize {
    (index << 1) + 1
}
fn number_index_to_id(index: usize) -> usize {
    (index + 1) << 1
}
fn number_id_to_index(id: usize) -> usize {
    (id - 2) >> 1
}
fn is_number_id(id: usize) -> bool {
    id & 1 == 0 && id != 0
}

impl Map {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            numbers: Vec::new(),
            symbols: Vec::new(),
            id_texture: vec![vec![0; cols]; rows],
            rows,
            cols,
        }
    }

    fn push_number(&mut self, num: Number) -> &mut Self {
        let ind = self.numbers.len();
        let id = number_index_to_id(ind);
        for col in num.start_col..=num.end_col {
            self.id_texture[num.row][col] = id;
        }
        self.numbers.push(num);
        self
    }

    fn push_symbol(&mut self, sym: Symbol) -> &mut Self {
        let id = symbol_index_to_id(self.symbols.len());
        self.id_texture[sym.row][sym.col] = id;
        self.symbols.push(sym);
        self
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Map {
        let lines = input
            .lines()
            .map(|line| line.as_bytes())
            .collect::<Vec<&[u8]>>();

        let mut res = Map::new(
            lines.len(),
            lines.get(0).map(|e| e.len()).unwrap_or_default(),
        );

        for (row, bs) in lines.into_iter().enumerate() {
            let mut col = 0;
            while col < bs.len() {
                match bs[col] {
                    b'0'..=b'9' => {
                        let mut num = Number::new(row, col, (bs[col] - b'0') as usize);
                        col += 1;
                        while col < bs.len() && bs[col].is_ascii_digit() {
                            num.push(bs[col]);
                            col += 1;
                        }
                        res.push_number(num);
                    }
                    b'.' => {
                        col += 1;
                        continue;
                    }
                    _ch => {
                        let sym = Symbol::new(row, col);
                        res.push_symbol(sym);
                        col += 1;
                    }
                }
            }
        }
        res
    }
}

fn neighbours(
    rows: usize,
    cols: usize,
    row: usize,
    col: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let min_r = std::cmp::max(0, row.saturating_sub(1));
    let max_r = std::cmp::min(rows, row + 2);
    let min_c = std::cmp::max(0, col.saturating_sub(1));
    let max_c = std::cmp::min(cols, col + 2);
    (min_r..max_r).flat_map(move |r| (min_c..max_c).map(move |c| (r, c)))
}

pub fn sum(input: &str) -> usize {
    let map = Map::from(input);
    let numbers_n = map.numbers.len();
    let mut sum = 0usize;
    let mut counted_ids = Vec::with_capacity(numbers_n);

    counted_ids.resize(numbers_n, false);

    for sym in &map.symbols {
        for (row, col) in neighbours(map.rows, map.cols, sym.row, sym.col) {
            let id = map.id_texture[row][col];
            if !is_number_id(id) {
                continue;
            }
            let ind = number_id_to_index(id);
            if counted_ids[ind] {
                continue;
            }
            let num = &map.numbers[ind];
            sum += num.value;
            counted_ids[ind] = true;
        }
    }
    sum
}

pub fn gear_ratio(input: &str) -> usize {
    let map = Map::from(input);
    let mut sum: usize = 0;

    'symb_loop: for sym in map.symbols {
        let mut prod = 1usize;
        let mut counted_ids = (0, 0);

        for (row, col) in neighbours(map.rows, map.cols, sym.row, sym.col) {
            let id = map.id_texture[row][col];

            if id == 0 {
                continue;
            }
            if !is_number_id(id) {
                continue;
            }
            let num = &map.numbers[number_id_to_index(id)];

            if counted_ids.0 == 0 {
                counted_ids.0 = id;
                prod *= num.value;
                continue;
            }
            if counted_ids.0 == id {
                continue;
            }
            if counted_ids.1 == 0 {
                counted_ids.1 = id;
                prod *= num.value;
                continue;
            }
            if counted_ids.1 != id {
                continue 'symb_loop;
            }
        }
        if counted_ids.1 == 0 {
            continue;
        }
        sum += prod;
    }

    sum
}
