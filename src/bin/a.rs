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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    choice: Vec<usize>,    // 選んだ注文
    choiced: Vec<bool>,    // 選ばれた注文が true
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
            choiced: vec![false; ORDER_TOTAL],
            route: vec![Coord::new((400, 400))],
            todo: BTreeSet::new(),
            moved_dist: 0,
        }
    }

    fn move_to(&mut self, to: &Coord) {
        self.route.push(to.clone());
        self.moved_dist += self.pos.distance(to);

        self.todo.remove(&to);

        self.pos = to.clone();
    }

    fn choose(&mut self, req: &Request) {
        self.choice.push(req.id);
        self.choiced[req.id - 1] = true;

        self.todo.insert(req.s.clone());
        self.todo.insert(req.g.clone());
    }

    // リクエストを選んで、その始点に移る
    fn choose_and_move(&mut self, req: &Request) {
        self.choose(&req);
        self.move_to(&req.s);
    }

    // ここまでの移動距離でスコア算出
    fn calc_score(&self) -> usize {
        (1e8 / (1000.0 + self.moved_dist as f64)).round() as usize
    }

    // 未チョイスのリクエストの内、以下基準で良さそうなものを選ぶ。
    // - 現地点との始点の近さ
    // - 始点-終点間の距離の近さ
    fn search_looks_good_req(&self, input: &Input) -> Request {
        let mut res: Option<&Request> = None;
        fn calc(st: &State, req: &Request) -> usize {
            // TODO: ここ変動させて全パターン試すとか
            st.pos.distance(&req.s) + req.calc_sg_dist() / 4
        }

        let hoge = 550;
        let under_hoge: Vec<&Request> = input
            .reqs
            .iter()
            .filter(|req| req.s.x <= hoge && req.s.y <= hoge && req.g.x <= hoge && req.g.y <= hoge)
            .collect();
        eprintln!("under_hoge len: {}", under_hoge.len());
        for req in &under_hoge {
            if !self.choiced[req.id - 1] {
                match res {
                    None => {
                        res = Some(&req);
                    }
                    Some(now) => {
                        // now より良さそうなら
                        if calc(&self, &req) < calc(&self, &now) {
                            res = Some(&req);
                        }
                    }
                }
            }
        }

        (*res.unwrap()).clone()
    }

    // 最も近いtodo座標を返す
    fn search_nearest_todo(&self) -> Coord {
        let mut res: Option<&Coord> = None;
        for to in &self.todo {
            match res {
                None => {
                    res = Some(&to);
                }
                Some(now) => {
                    // now より近ければ
                    if self.pos.distance(&to) < self.pos.distance(&now) {
                        res = Some(&to);
                    }
                }
            }
        }

        (res.unwrap()).clone()
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

    fn solve(&mut self, input: &Input, system_time: &SystemTime) {
        // const 的なアレ
        let office: Coord = Coord::new((400, 400));

        // 最も近い始点に向かう
        while self.choice.len() < SELECT_ORDER_NUM {
            let req = self.search_looks_good_req(&input);
            // TODO: 途中で消化できるtodoは消化する <= 遠すぎて通り道にないのよな
            self.choose_and_move(&req);
        }

        // todoを処理していく
        let mut nodes: Vec<Coord> = self.todo.clone().into_iter().collect();
        nodes.push(office.clone());
        nodes.push(self.pos.clone());
        nodes.rotate_right(1);
        // eprintln!("{:?}", nodes);

        let path = (0..nodes.len()).collect();
        let mut table = vec![vec![0; nodes.len()]; nodes.len()];
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                let dist = nodes[i].distance(&nodes[j]);
                table[i][j] = dist;
                table[j][i] = dist;
            }
        }

        let mut yn = Yamanobori::new(path, table, system_time.clone());
        yn.run(1_000);

        for i in 1..yn.path.len() {
            let to: Coord = nodes[yn.path[i]];
            self.move_to(&to);
        }

        // 終点へ
        self.move_to(&office);
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
    st.solve(&input, &system_time);

    eprintln!("score: {}", st.calc_score());
    eprintln!("todo_len: {}", st.todo.len());
    // outout
    st.print();

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}

#[allow(dead_code)]
struct Yamanobori {
    path: Vec<usize>,
    score: usize,
    table: Vec<Vec<usize>>, // 二点間の距離
    main_start_time: SystemTime,
}
#[allow(dead_code)]
impl Yamanobori {
    fn new(
        start_path: Vec<usize>,
        table: Vec<Vec<usize>>,
        main_start_time: SystemTime,
    ) -> Yamanobori {
        let mut score = 0;
        let path_length = start_path.len();

        // 初期スコアの作成
        for i in 0..path_length - 1 {
            score += table[start_path[i]][start_path[i + 1]];
        }

        Yamanobori {
            path: start_path,
            score,
            table,
            main_start_time,
        }
    }

    // [li, ri] を反転
    fn range_reverse(&mut self, li: usize, ri: usize) {
        let diff = (ri - li) + 1;
        for i in 0..diff / 2 {
            self.path.swap(li + i, ri - i);
        }
    }

    fn access_table_by_path_id(&self, i1: usize, i2: usize) -> usize {
        self.table[self.path[i1]][self.path[i2]]
    }

    // end_time: main関数の開始後からの時間を指す
    fn run(
        &mut self,
        end_time: u128, // ミリ秒表記
    ) {
        let mut rand = rand_pcg::Pcg64Mcg::new(890482);
        let path_length = self.path.len();

        while self.main_start_time.elapsed().unwrap().as_millis() < end_time {
            for _ in 0..1000 {
                // TODO: 1, -1 か
                let mut lci = rand.gen_range(0, path_length); // left cut i
                let mut rci = rand.gen_range(0, path_length); // right cut i
                if lci > rci {
                    // swap
                    lci ^= rci;
                    rci ^= lci;
                    lci ^= rci;
                }

                if lci == rci || (lci == 0 || rci == path_length - 1) {
                    continue;
                }

                let pre = self.access_table_by_path_id(lci - 1, lci)
                    + self.access_table_by_path_id(rci, rci + 1);
                let next = self.access_table_by_path_id(lci - 1, rci)
                    + self.access_table_by_path_id(lci, rci + 1);

                if next < pre {
                    self.score += next;
                    self.score -= pre;

                    self.range_reverse(lci, rci);
                }
            }
        }
    }
}
