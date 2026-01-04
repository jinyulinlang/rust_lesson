// 实现一个缓存，只处理第一次传入的值并保存
struct Cacher<T>
    where T: Fn(u32) -> u32 {
    cal: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(cal: T) -> Cacher<T> {
        Cacher {
            cal,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => {
                if v > arg {
                    v
                } else {
                    self.value = Some(arg);
                    arg
                }
            }
            None => {
                let v = (self.cal)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut c = Cacher::new(|x| x + 1);
    let v = c.value(12);
    println!("{}", v);
    let v2 = c.value(15);
    println!("{}", v2);
    let v3 = c.value;
    println!("{}", v3.unwrap());
    let v = vec![1, 2, 3, 4];
    // for val in v.iter() {
    //     println!("{}", val)
    // }
    let mut v1 = v.iter();
    if let Some(m) = v1.next() {
        println!("{}", m);
    };

    let mut mus = vec![1, 2, 3];
    let mut mu_iter = mus.iter_mut();
    if let Some(m) = mu_iter.next() {
        *m = 3;
    }
    println!("mus={:?}", mus);
    for m in mus.iter_mut() {
        println!("{}", m)
    }


    let v1 = vec![1, 2, 3];
    let v1_it = v1.iter();
    let sum: i32 = v1_it.sum();
    println!("sum={}", sum);
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("v2={:?}", v2);

    println!("Hello, world!");
    Users/jinyulinlang/soft/rust/rust-home/rust_lesson/rust_closure/target/release/rust_lesson
}
