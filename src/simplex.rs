use std::fmt;
use point::Point;

// For now, hardcode the Simplex for a 2D space.
// TODO: Think how to update it to a N-dimensional space
pub struct Simplex<F> {
    //f: fn(Point) -> f64,
    pub f: F,
    pub points: [Point; 3],
    pub values: [f64; 3],

    pub alpha: f64,
    pub beta: f64
}

impl<F> fmt::Display for Simplex<F> where F: FnMut(Point) -> f64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p0 = self.points[0];
        let p1 = self.points[1];
        let p2 = self.points[2];
        let vals = self.values;

        try!(write!(f, "({:9.3e}, {:9.3e}) -> {:9.3e}\n", p0.x, p0.y, vals[0]));
        try!(write!(f, "({:9.3e}, {:9.3e}) -> {:9.3e}\n", p1.x, p1.y, vals[1]));
        write!(f,      "({:9.3e}, {:9.3e}) -> {:9.3e}",   p2.x, p2.y, vals[2])
    }
}

impl<F> Simplex<F> where F: FnMut(Point) -> f64 {

    pub fn evolve(&mut self) {
        let mut i_max = self.find_index_of_max();
        let mut i_min = self.find_index_of_min();

        let mut p_max = self.points[i_max];
        let mut p_min = self.points[i_min];

        let mut dp = p_max - p_min;
        let mut dp_mod = (dp.x * dp.x + dp.y * dp.y).sqrt();

        while dp_mod > 1e-2 {
            self.step();

            i_max = self.find_index_of_max();
            i_min = self.find_index_of_min();

            p_max = self.points[i_max];
            p_min = self.points[i_min];

            dp = p_max - p_min;
            dp_mod = (dp.x * dp.x + dp.y * dp.y).sqrt();
        }
    }

    fn step(&mut self) {
        let i_max = self.find_index_of_max();
        let i_min = self.find_index_of_min();

        let p_max = self.points[i_max];
        let p_min = self.points[i_min];

        let mut p_new = Point { x: 0.0, y: 0.0 };

        for (i,p) in self.points.iter().enumerate() {
            if i == i_max {continue}
            
            p_new += p;
        }
        
        p_new /= 2.0;

        let direction_of_p_new = p_new - p_max;
        p_new = p_new * 2.0 - p_max; // p_max + 2 * (p_new - p_max)

        let y_new = (self.f)(p_new);

        // The new point is better than the best point so far!
        if y_new < self.values[i_min] {
            let p_new_scaled = p_max + 
                direction_of_p_new * (2.0 * self.alpha);

            let y_new_scaled = (self.f)(p_new_scaled);

            if y_new_scaled < y_new {
                self.points[i_max] = p_new_scaled;
                self.values[i_max] = y_new_scaled;
            }
            else {
                self.points[i_max] = p_new;
                self.values[i_max] = y_new;
            }
        }
        // The new point is better than the worst...
        else if y_new < self.values[i_max] {
            let p_new_scaled = p_max + 
                direction_of_p_new * (2.0 * self.beta);

            let y_new_scaled = (self.f)(p_new_scaled);

            if y_new_scaled < y_new {
                self.points[i_max] = p_new_scaled;
                self.values[i_max] = y_new_scaled;
            }
            else {
                self.points[i_max] = p_new;
                self.values[i_max] = y_new;
            }
        }
        // The new point is worse than the worst...
        else {
            for (i,p) in self.points.clone().iter().enumerate() {
                if i == i_min { continue }

                self.points[i] = (*p + p_min) / 2.0;
                self.values[i] = (self.f)(self.points[i]);
            }
        }
    }

    fn find_index_of_max(&self) -> usize {
        let mut i_max = 0;
        let mut v_max = self.values[0];

        for (i,v) in self.values.iter().enumerate() {
            if v_max < *v {
                i_max = i;
                v_max = *v;
            }
        }

        i_max
    }

    fn find_index_of_min(&self) -> usize {
        let mut i_min = 0;
        let mut v_min = self.values[0];

        for (i,v) in self.values.iter().enumerate() {
            if v_min > *v {
                i_min = i;
                v_min = *v;
            }
        }

        i_min
    }
}
