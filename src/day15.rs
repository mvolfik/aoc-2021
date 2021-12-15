use std::cmp::Ordering;
use std::str::Lines;

type Position = (i32, i32);

pub(crate) fn main(
    stdin: Lines,
) -> Result<(Result<String, String>, Result<String, String>), String> {
    let mut grid = WrappedGrid::new(stdin, (1, 1))?;
    let pt1 = search(&grid).map(|x| x.to_string());
    //Option::<()>::None.unwrap();
    grid.set_repeat((5, 5));
    let pt2 = search(&grid).map(|x| x.to_string());
    Ok((pt1, pt2))
}

struct WrappedGrid {
    data: Vec<Vec<u32>>,
    border: Position,
    wrap_border: Position,
}
impl WrappedGrid {
    fn get(&self, (x, y): Position) -> Option<u32> {
        if x > self.border.0 || y > self.border.1 || x < 0 || y < 0 {
            return None;
        }
        let (mx, x) = ((x / self.wrap_border.0) as u32, x % self.wrap_border.0);
        let (my, y) = ((y / self.wrap_border.1) as u32, y % self.wrap_border.1);
        return Some((self.data[y as usize][x as usize] - 1 + mx + my) % 9 + 1);
    }
    fn set_repeat(&mut self, repeat: Position) {
        self.border = (
            self.wrap_border.0 * repeat.0 - 1,
            self.wrap_border.1 * repeat.1 - 1,
        );
    }
    fn new(data: Lines, repeat: Position) -> Result<Self, String> {
        let mut grid_data = Vec::new();
        let mut w = None;
        for l in data {
            let mut grid_l = Vec::new();
            for c in l.chars() {
                grid_l.push(c.to_digit(10).ok_or("Expected digit")?)
            }
            let new_linew = grid_l.len();
            match w {
                None => w = Some(new_linew),
                Some(x) => {
                    if new_linew != x {
                        return Err("Lines are not same length".to_string());
                    }
                }
            }
            grid_data.push(grid_l)
        }
        let wrap_border = (w.ok_or("No input data")? as i32, grid_data.len() as i32);
        let mut out = Self {
            data: grid_data,
            border: (0, 0),
            wrap_border,
        };
        out.set_repeat(repeat);
        Ok(out)
    }
}

struct PathPoint {
    cost: u32,
    pos: Position,
}

impl PartialEq for PathPoint {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl Eq for PathPoint {}
impl Ord for PathPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}
impl PartialOrd for PathPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(grid: &WrappedGrid) -> Result<u32, String> {
    let mut q = std::collections::BinaryHeap::new();
    q.push(PathPoint {
        cost: 0,
        pos: (0, 0),
    });
    let mut visited = std::collections::HashSet::new();
    while !q.is_empty() {
        let PathPoint { cost, pos } = q.pop().unwrap();
        if visited.contains(&pos) {
            continue;
        }
        for i in 0..4 {
            let (a, b) = (i / 2, i % 2);
            let pos2 = (
                pos.0 as i32 + (a - 2 * (a & b)),
                pos.1 as i32 + ((1 - a) * (1 - 2 * b)),
            );
            if visited.contains(&pos2) {
                continue;
            }
            let nextval = match grid.get(pos2) {
                None => continue,
                Some(x) => x,
            };
            let newc = cost + nextval;
            if pos2 == grid.border {
                return Ok(newc);
            }
            q.push(PathPoint {
                cost: newc,
                pos: pos2,
            })
        }

        visited.insert(pos);
    }
    Err("No path found".to_string())
}
