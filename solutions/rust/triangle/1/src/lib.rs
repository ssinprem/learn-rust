 use std::ops::Add;

pub struct Triangle<T> (T,T,T);

impl<T> Triangle<T> 
where T : Copy + Add + Default + PartialEq {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>>
        where <T as Add>::Output: PartialOrd<T> {
        if sides[0] == T::default() ||
           sides[1] == T::default() ||
           sides[2] == T::default()
        {
            None
        } else if sides[0] + sides[1] >= sides[2] &&
           sides[0] + sides[2] >= sides[1] &&
           sides[1] + sides[2] >= sides[0]
        {
            Some(Self(
                sides[0],
                sides[1],
                sides[2]
            ))
        } else {
            None
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
