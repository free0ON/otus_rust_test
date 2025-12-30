struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T>
//where T: Copy,
{
    pub fn x_mut(&mut self) -> *mut T {
        &raw mut self.x
    }

    pub fn y_mut(&mut self) -> *mut T {
        &raw mut self.y
    }
}

struct PointTuple<T>(T, T); // x, y

impl<T> PointTuple<T> {
    pub fn x_mut(&mut self) -> *mut T {
        &raw mut self.0
    }

    pub fn y_mut(&mut self) -> *mut T {
        &raw mut self.1
    }
}

fn main() {}

#[cfg(test)]
// use crate::Point;
#[test]
fn borrow_test() {
    let mut point: Point<i32> = Point::<i32> { x: 1, y: 1 };
    let ref mut x_ref = point.x;
    let ref mut y_ref = point.y;
    *x_ref *= 2;
    *y_ref *= 2;
    assert_eq!(*x_ref, 2);
    assert_eq!(*y_ref, 2);
}

#[cfg(feature = "partial_borrowing")]
#[test]
#[ignore = "cannot partial borrow"]
fn partial_borrow_test() {
    let mut point: Point<i32> = Point::<i32> { x: 1, y: 1 };
    let mut x_mut = point.x_mut();
    let mut y_mut = point.y_mut();

    //                    ^~~~~
    // error: cannot borrow `point` as mutable more than once at a time
    x_mut *= 2;
    y_mut *= 2;

    assert_eq!(point.x, 2);
    assert_eq!(point.y, 2);
}

#[test]
fn partial_raw_tupple_test() {
    let mut point_tuple = PointTuple::<i32>(1, 1);
    let x_tuple_mut = point_tuple.x_mut();
    let y_tuple_mut = point_tuple.y_mut();
    unsafe {
        *x_tuple_mut = 2;
        *y_tuple_mut = 2;
    }
    assert_eq!(point_tuple.0, 2);
    assert_eq!(point_tuple.1, 2);
}

#[test]
fn partial_raw_stuct_test() {
    let mut point: Point<i32> = Point::<i32> { x: 1, y: 1 };
    let x_mut = point.x_mut();
    let y_mut = point.y_mut();
    unsafe {
        *x_mut *= 2;
        *y_mut *= 2;
    }
    assert_eq!(point.x, 2);
    assert_eq!(point.y, 2);
}
