 use std::ops::Add;

pub struct Triangle<T> (T,T,T);

impl<T> Triangle<T> 
where T : Default + Copy + PartialEq + PartialOrd + Add<Output=T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let mut sort_sides = sides;
        sort_sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if sides.iter().any(|s| *s == T::default()) ||
           sort_sides[0] + sort_sides[1] < sort_sides[2]
        {
            None
        } else {
            Some(Self(
                sides[0],
                sides[1],
                sides[2]
            ))
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0.eq(&self.1) && self.1.eq(&self.2)
    }

    pub fn is_scalene(&self) -> bool {
        ! self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.0.eq(&self.1) || self.1.eq(&self.2) || self.2.eq(&self.0)
    }
}
