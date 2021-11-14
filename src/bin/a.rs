#[allow(unused_imports)]
use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{fastout, input};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(unused_imports)]
use rand::rngs::ThreadRng;
#[allow(unused_imports)]
use rand::seq::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};
#[allow(unused_imports)]
use std::io::Write;
use std::time::SystemTime;

#[allow(dead_code)]
const MOD: usize = 1e9 as usize + 7;

const ORDER_TOTAL: usize = 1_000;
const SELECT_ORDER_NUM: usize = 50;

const SIDE: usize = 800;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
impl Coord {
    fn new(p: (isize, isize)) -> Self {
        Coord { x: p.0, y: p.1 }
    }
    fn from_usize_pair(p: (usize, usize)) -> Self {
        Coord {
            x: p.0 as isize,
            y: p.1 as isize,
        }
    }

    fn in_field(&self) -> bool {
        (0 <= self.x && self.x <= SIDE as isize) && (0 <= self.y && self.y <= SIDE as isize)
    }

    // ペアへの変換
    fn to_pair(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    // マンハッタン距離
    fn distance(&self, that: &Self) -> usize {
        ((self.x - that.x).abs() + (self.y - that.y).abs()) as usize
    }

    fn mk_4dir(&self) -> Vec<Self> {
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|&p| self.plus(&Coord::new(p)))
            .filter(|&pos| pos.in_field())
            .collect()
    }

    fn com_to_delta(com: char) -> Self {
        match com {
            'U' => Coord::new((0, -1)),
            'D' => Coord::new((0, 1)),
            'L' => Coord::new((-1, 0)),
            'R' => Coord::new((1, 0)),
            _ => unreachable!(),
        }
    }

    // 四則演算
    fn plus(&self, that: &Self) -> Self {
        Coord::new((self.x + that.x, self.y + that.y))
    }
    fn minus(&self, that: &Self) -> Self {
        Coord::new((self.x - that.x, self.y - that.y))
    }

    fn access_matrix<'a, T>(&'a self, mat: &'a Vec<Vec<T>>) -> &'a T {
        &mat[self.y as usize][self.x as usize]
    }

    fn set_matrix<T>(&self, mat: &mut Vec<Vec<T>>, e: T) {
        mat[self.y as usize][self.x as usize] = e;
    }
}

#[allow(dead_code)]
struct Request {
    id: usize, // 出力にしか使わない
    s: Coord,
    g: Coord,
}
impl Request {
    fn calc_sg_dist(&self) -> usize {
        self.s.distance(&self.g)
    }
}

struct Input {
    reqs: Vec<Request>, // オーダー一覧. id-1 で引ける
}
impl Input {
    fn new(reqs: Vec<Request>) -> Self {
        Self { reqs }
    }
}

#[allow(dead_code)]
struct State {
    pos: Coord,
    choice: Vec<usize>,
    route: Vec<Coord>,     // 現在地も含む
    todo: BTreeSet<Coord>, // これから踏まなければならない地点
    moved_dist: usize,     // ここまでの移動距離
}
impl State {
    // 始点に降り立った状態
    fn new() -> Self {
        Self {
            pos: Coord::new((400, 400)),
            choice: vec![],
            route: vec![Coord::new((400, 400))],
            todo: BTreeSet::new(),
            moved_dist: 0,
        }
    }

    fn move_to(&mut self, to: &Coord) {
        self.route.push(to.clone());
        self.moved_dist += self.pos.distance(to);

        self.pos = to.clone();
    }

    // リクエストを選んで、その始点に移る
    fn choose_and_move(&mut self, req: &Request) {
        self.choice.push(req.id);
        self.todo.insert(req.g.clone());

        self.move_to(&req.s);
    }

    // 結果出力
    fn print(&self) {
        print!("{}", self.choice.len());
        for req in &self.choice {
            print!(" {}", req);
        }
        println!();

        print!("{}", self.route.len());
        for pos in &self.route {
            print!(" {} {}", pos.x, pos.y);
        }
        println!();
    }
}

#[fastout]
fn main() {
    let system_time = SystemTime::now();
    let mut _rng = thread_rng();

    let office: Coord = Coord::new((400, 400));

    // input
    let mut reqs = Vec::with_capacity(ORDER_TOTAL);
    for i in 1..=ORDER_TOTAL {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }

        let s = Coord::from_usize_pair((a, b));
        let g = Coord::from_usize_pair((c, d));

        let req = Request { id: i, s, g };
        reqs.push(req)
    }

    let input = Input::new(reqs);

    // solve
    let mut st = State::new();

    st.route.push(office.clone());
    for i in 0..SELECT_ORDER_NUM {
        let req = &input.reqs[i];
        st.choice.push(req.id);
        st.route.push(req.s.clone());
        st.route.push(req.g.clone());
    }
    st.route.push(office.clone());

    // outout
    st.print();

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}
