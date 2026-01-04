use std::ops::Add;

fn main() {
    let a = 12;
    let b = 15;
    let ret = add(a, b);
    println!("The sum of {} and {} is {}", a, b, ret);
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    // let p3 = p1 + p2;
    let p3 = add(p1, p2);
    println!("{:?}+{:?} = {:?}", p1, p2, p3);
    let hp1 = HeavyPoint { x: 1, y: 2 };
    let hp2 = HeavyPoint { x: 3, y: 4 };
    let hp3 = &hp1 + &hp2;
    println!("{:?}+{:?} = {:?}", hp1, hp2, hp3);
}

fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn add2<T>(a: &T, b: &T) -> T
where
    // 高级生命周期绑定（晚绑定） for<'a> ,
    for<'a> &'a T: Add<Output = T>,
{
    a + b
}
#[derive(Debug, Clone, Copy)]
struct Point<T: Add<Output = T>> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct HeavyPoint<T: Add<Output = T>> {
    x: T,
    y: T,
}
impl<T: Add<Output = T> + Copy> Add for &HeavyPoint<T> {
    type Output = Point<T>;
    fn add(self, other: &HeavyPoint<T>) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
