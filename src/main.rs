use std::ops::Index;

struct ArrayStack<T> {
    array: Vec<T>,
    n: usize
}

impl<T> ArrayStack<T> {
    pub fn new() -> Self {
        // 配列のnew(C++)ではなく空ベクタで実装
        ArrayStack{array:Vec::new(), n:0}
    }
    fn size(&self) -> usize { self.n }
    fn get(&self, i: usize) -> &T { &self.array[i] }
    fn set(&mut self, i: usize, x: T) {
        // Tを返す方法がよく分からなくて放置
        self.array[i] = x
    }
    fn add(&mut self, x: T) {
        self.array.push(x);
        self.n += 1
    }    
}

// 添字アクセスの実装
impl<T> Index<usize> for ArrayStack<T> {
    type Output = T;
    fn index<'a>(&'a self, i: usize) -> &'a T {
        // assertが未実装
        &self.array[i]
    }
}


fn main() {
    let mut a = ArrayStack::new();
    for _ in 0..3 {
        a.add(0);
    }
    println!("{}", a.size());
    println!("{:?}", a.array);
    a.set(0, 1);
    a.set(1, 2);
    a.set(2, 4);
    println!("{:?}", a.array);

    for i in 0..3 {
        println!("a[{}] = {}, by index {}", i, a.get(i), a[i]);
    }
}
