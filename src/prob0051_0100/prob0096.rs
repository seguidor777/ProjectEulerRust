#[link(name = "prob0096", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use core::num::{ ToStrRadix };
use core::iterator::{ IteratorUtil };
use common::extiter::{ ExtIteratorUtil, Range };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 96,
    answer: "24702",
    solver: solve
};

static BOARD_WIDTH: uint = 9;
static BOARD_HEIGHT: uint = 9;
static GROUP_WIDTH: uint = 3;
static GROUP_HEIGHT: uint = 3;
static MAX_NUMBER: uint = 9;
type BITS = u16;
static MASK_ALL: BITS = 0x1ff;

struct SuDoku {
    name: ~str,
    map: [[BITS, .. BOARD_WIDTH], .. BOARD_HEIGHT]
}

impl TotalEq for SuDoku {
    #[inline(always)]
    fn equals(&self, other: &SuDoku) -> bool {
        self.name == other.name && Range::new(0, BOARD_HEIGHT).all(|&y| self.map[y] == other.map[y])
    }
}

impl Eq for SuDoku {
    #[inline(always)]
    fn eq(&self, other: &SuDoku) -> bool { self.equals(other) }
    #[inline(always)]
    fn ne(&self, other: &SuDoku) -> bool { !self.equals(other) }
}

impl ToStr for SuDoku {
    fn to_str(&self) -> ~str {
        let rows = do self.map.map |row| {
            let cells = do row.map |&cell| {
                if cell.population_count() == 1 {
                    ~"_"
                } else {
                    (1u << cell.trailing_zeros()).to_str()
                }
            };
            str::concat(cells)
        };
        return self.name + "\n" + str::connect(rows, "\n");
    }
}

impl Clone for SuDoku {
    fn clone(&self) -> SuDoku {
        SuDoku { name: self.name.clone(), map: self.map }
    }
}

impl SuDoku {
    fn get_num(&self, x: uint, y: uint) -> BITS {
        match self.map[y][x].population_count() {
            0 => -1,
            1 => self.map[y][x].trailing_zeros() + 1,
            _ => 0
        }
    }

    fn to_str_debug(&self) -> ~str {
        let row_strs = do vec::build_sized(BOARD_HEIGHT) |push| {
            for uint::range(0, BOARD_HEIGHT) |y| {
                let cell_strs = do vec::build_sized(BOARD_WIDTH) |push| {
                    for uint::range(0, BOARD_WIDTH) |x| {
                        let s = self.map[y][x].to_str_radix(2);
                        push(str::replace(fmt!("%s:%s",
                                               self.get_num(x, y).to_str(),
                                               str::repeat("0", MAX_NUMBER - s.len()) + s), "0", "_"));
                    }
                };
                push(str::connect(cell_strs, " "));
            }
        };
        return self.name + "\n" + str::connect(row_strs, "\n");
    }
}

fn read_sudoku<T: Reader>(r: T) -> SuDoku {
    let mut sudoku = SuDoku {
        name: r.read_line(),
        map: [[MASK_ALL, .. BOARD_WIDTH], .. BOARD_HEIGHT]
    };

    for uint::range(0, BOARD_HEIGHT) |y| {
        let line = r.read_line();
        for uint::range(0, BOARD_WIDTH) |x| {
            let n = char::to_digit(line[x] as char, 10).get();
            if n != 0 { sudoku.map[y][x] = 1 << (n - 1); }
        }
    }
    return sudoku;
}

fn solve_sudoku(mut puzzle: SuDoku) -> ~[SuDoku] {
    let group_it = Range::new(0, GROUP_WIDTH * GROUP_HEIGHT)
        .transform(|i| (i % GROUP_WIDTH, i / GROUP_WIDTH));

    loop {
        let bkup = puzzle.clone();

        for uint::range(0, BOARD_HEIGHT) |y| {
            for uint::range(0, BOARD_WIDTH) |x| {
                if puzzle.map[y][x].population_count() != 1 { loop; }

                let x0 = x / GROUP_WIDTH * GROUP_WIDTH, y0 = y / GROUP_HEIGHT * GROUP_HEIGHT;
                let row = Range::new(0, BOARD_WIDTH).transform(|x| (x, y));
                let col = Range::new(0, BOARD_HEIGHT).transform(|y| (x, y));
                let grp = group_it.transform(|(dx, dy)| (x0 + dx, y0 + dy));

                let mut it = row.chain(col).chain(grp).filter(|&pos: &(uint, uint)| pos != (x, y));
                let mask = !puzzle.map[y][x] & MASK_ALL;
                for it.advance |(x, y)| { puzzle.map[y][x] &= mask; }
            }
        }

        for uint::range(0, MAX_NUMBER) |n| {
            let bit = 1 << n;

            for uint::range(0, BOARD_HEIGHT) |y| {
                let it = Range::new(0, BOARD_WIDTH).filter(|&x| puzzle.map[y][x] & bit != 0);
                if it.count_elem() != 1 { loop; }
                puzzle.map[y][it.first()] = bit;
            }

            for uint::range(0, BOARD_WIDTH) |x| {
                let it = Range::new(0, BOARD_HEIGHT).filter(|&y| puzzle.map[y][x] & bit != 0);
                if it.count_elem() != 1 { loop; }
                puzzle.map[it.first()][x] = bit;
            }

            for uint::range_step(0, BOARD_HEIGHT, GROUP_WIDTH as int) |y0| {
                for uint::range_step(0, BOARD_WIDTH, GROUP_HEIGHT as int) |x0| {
                    let it = group_it
                        .transform(|(dx, dy)| (x0 + dx, y0 + dy))
                        .filter(|&(x, y)| puzzle.map[y][x] & bit != 0);
                    if it.count_elem() != 1 { loop; }
                    let (x, y) = it.first();
                    puzzle.map[y][x] = bit;
                }
            }
        }

        if puzzle == bkup { break; }
    }

    let it = Range::new(0, BOARD_HEIGHT * BOARD_WIDTH)
        .transform(|i| (i % BOARD_WIDTH, i / BOARD_WIDTH))
        .transform(|(x, y)| (x, y, puzzle.map[y][x].population_count()));

    if it.any(|&(_x, _y, cnt)| cnt == 0) { return ~[]; }
    if it.all(|&(_x, _y, cnt)| cnt == 1) { return ~[puzzle]; }

    let (x, y, _cnt) = it
        .filter(|&(_x, _y, cnt)| cnt > 1)
        .min_as(|&(_x, _y, cnt)| cnt);

    let mut answers = ~[];
    for uint::range(0, MAX_NUMBER) |n| {
        let bit = 1 << n;
        if puzzle.map[y][x] & bit == 0 { loop; }
        
        let mut p2 = puzzle.clone();
        p2.map[y][x] = bit;
        answers.push_all(solve_sudoku(p2));
    }

    return answers;
}

pub fn solve() -> ~str {
    let result = io::file_reader(&Path("files/sudoku.txt")).map(|&file| {
        let mut puzzles = ~[];
        while !file.eof() { puzzles.push(read_sudoku(file)); }
        puzzles
    }).map(|&puzzles| {
        let mut answers = ~[];
        for puzzles.each |&p| {
            let mut ans = solve_sudoku(p);
            assert_eq!(ans.len(), 1);
            answers.push(ans.pop());
        }
        answers
    }).map(|&answers| {
        let mut sum = 0;
        for answers.each |ans| {
            sum += 100 * ans.get_num(0, 0) + 10 * ans.get_num(1, 0) + ans.get_num(2, 0);
        }
        sum
    });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}
