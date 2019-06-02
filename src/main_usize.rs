#[warn(non_snake_case)]
struct ArrayStack {
    array: Vec<usize>,
    n: usize
}

impl ArrayStack {
    pub fn new() -> Self {
        ArrayStack{array:Vec::new(), n:0}
    }
    fn size(&self) -> usize {
        self.n
    }
    fn get(&self, i: usize) -> &usize {
        &self.array[i]
    }
    fn add(&mut self, x: usize) {
        self.array.push(x);
        self.n += 1
    }
    fn set(&mut self, i: usize, x: usize) {
        self.array[i] = x;
    }
}

fn main() {
    let mut a = ArrayStack::new();
    for i in 0..3 {
        a.add(0);
    }
    println!("{}", a.size());
    println!("{:?}", a.array);
    a.set(0, 1);
    a.set(1, 2);
    a.set(2, 4);
    println!("{:?}", a.array);
}
