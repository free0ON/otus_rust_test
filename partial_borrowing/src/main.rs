struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.y
    }
}

#[cfg(test)]
//use crate::Point;
#[test]
fn borrow_test() {
    let mut point: Point = Point { x: 1.0, y: 1.0 };
    let ref mut x_ref = point.x;
    let ref mut y_ref = point.y;
    *x_ref *= 2.0;
    *y_ref *= 2.0;
    assert_eq!(*x_ref, 2.0);
    assert_eq!(*y_ref, 2.0);
}

#[cfg(feature = "partial_borrowing")]
#[test]
#[ignore = "cannot partial borrow"]
fn partial_borrow_test() {
    let mut point: Point = Point { x: 1.0, y: 1.0 };
    let x_mut = point.x_mut();
    let y_mut = point.y_mut();
    //          ^~~~~
    // error: cannot borrow `point` as mutable more than once at a time
    *x_mut *= 2.0;
    *y_mut *= 2.0;
}
