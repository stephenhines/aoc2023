use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

const POS_MIN: f64 = 200000000000000.0;
const POS_MAX: f64 = 400000000000000.0;

#[derive(Clone, Copy, Debug)]
struct Hail {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hail {
    fn new(px: f64, py: f64, pz: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        }
    }

    fn parallel(self, other: &Hail) -> bool {
        if self.vx == other.vx && self.vy == other.vy {
            return true;
        }
        let left = self.vy * other.vx;
        let right = other.vy * self.vx;
        left == right
    }

    fn intersects(self, other: &Hail, pos_min: f64, pos_max: f64) -> bool {
        // https://stackoverflow.com/questions/73079419/intersection-of-two-vector
        if self.parallel(other) {
            //println!{"parallel {:?} {:?}", self, other};
            return false;
        }

        // I couldn't remember all my linear algebra.
        // https://stackoverflow.com/questions/2931573/determining-if-two-rays-intersect
        let dx = other.px - self.px;
        let dy = other.py - self.py;
        let det = other.vx * self.vy - other.vy * self.vx;
        let u = (dy * other.vx - dx * other.vy) / det;
        let v = (dy * self.vx - dx * self.vy) / det;
        // The times need to be in the future, and not in the past.
        if u >= 0.0 && v >= 0.0 {
            //println!{"intersection between {:?} {:?}", self, other};
            //println!{"u: {}, v:{}", u, v};
            let new_x = self.px + self.vx * u;
            let new_y = self.py + self.vy * u;
            if new_x >= pos_min && new_x <= pos_max && new_y >= pos_min && new_y <= pos_max {
                //println!{"new_x: {}, new_y:{}", new_x, new_y};
                return true;
            }
        }

        false
    }
}

fn get_intersections(lines: &[String], pos_min: f64, pos_max: f64) -> usize {
    let mut hail = Vec::new();
    for line in lines {
        let toks: Vec<_> = line.split('@').collect();
        let pos_toks: Vec<_> = toks[0].split(',').collect();
        let vel_toks: Vec<_> = toks[1].split(',').collect();
        let px = pos_toks[0].trim().parse().unwrap();
        let py = pos_toks[1].trim().parse().unwrap();
        let pz = pos_toks[2].trim().parse().unwrap();
        let vx = vel_toks[0].trim().parse().unwrap();
        let vy = vel_toks[1].trim().parse().unwrap();
        let vz = vel_toks[2].trim().parse().unwrap();
        hail.push(Hail::new(px, py, pz, vx, vy, vz));
    }

    //println!{"hail: {:?}", hail};

    let mut intersections = 0;
    for x in 0..hail.len() {
        for y in x + 1..hail.len() {
            if hail[x].intersects(&hail[y], pos_min, pos_max) {
                intersections += 1;
            }
        }

        //println!{"{:?}, m: {}, b: {}", hail[x], hail[x].get_m(), hail[x].get_b()};
    }

    println! {"intersections: {}", intersections};
    intersections
}

#[test]
fn test_prelim() {
    let intersections = get_intersections(&get_input("prelim.txt"), 7.0, 27.0);
    assert_eq!(intersections, 2);
}

#[test]
fn test_part1() {
    let intersections = get_intersections(&get_input("input.txt"), POS_MIN, POS_MAX);
    assert_eq!(intersections, 15262);
}

fn main() {
    get_intersections(&get_input("prelim.txt"), 7.0, 27.0);
    get_intersections(&get_input("input.txt"), POS_MIN, POS_MAX);
}
