#[macro_use]
extern crate derive_more;

mod simplex;
mod point;

use point::Point;

fn main() {
    let fun = |p: Point| { p.x.cos() * p.y.sin() * (-(p.x*p.x+p.y*p.y) / 10.0).exp() };

    let pp = [Point { x: 0.5, y: 0.0 },
              Point { x:-2.0, y: 2.0 },
              Point { x: 0.0, y:-2.0 }];

    let yy = [fun(pp[0]),
              fun(pp[1]),
              fun(pp[2])];

    let mut s = simplex::Simplex {
        f: fun,
        points: pp,
        values: yy,

        alpha: 1.5,
        beta: 0.66,
    };

    println!("{}\n", s);
    s.evolve();
    println!("{}\n", s);
}
