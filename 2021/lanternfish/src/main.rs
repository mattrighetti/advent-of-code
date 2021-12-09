struct Model {
    snapshot: Vec<i8>,
    day: i32
}

impl Model {
    fn new(snapshot: Vec<i8>) -> Self {
        Model { snapshot, day: 0 }
    }
}

impl Iterator for Model {
    type Item = Vec<i8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.day += 1;

        let mut new = 0;
        for num in self.snapshot.iter_mut() {
            if *num == 0 {
                *num = 6;
                new += 1;
            } else {
                *num -= 1;
            }
        }

        for _ in 0..new {
            self.snapshot.push(8);
        }

        Some(self.snapshot.clone())
    }
}

fn main() {
    let snapshot = vec![3,5,1,2,5,4,1,5,1,2,5,5,1,3,1,5,1,3,2,1,5,1,1,1,2,3,1,3,1,2,1,1,5,1,5,4,5,5,3,3,1,5,1,1,5,5,1,3,5,5,3,2,2,4,1,5,3,4,2,5,4,1,2,2,5,1,1,2,4,4,1,3,1,3,1,1,2,2,1,1,5,1,1,4,4,5,5,1,2,1,4,1,1,4,4,3,4,2,2,3,3,2,1,3,3,2,1,1,1,2,1,4,2,2,1,5,5,3,4,5,5,2,5,2,2,5,3,3,1,2,4,2,1,5,1,1,2,3,5,5,1,1,5,5,1,4,5,3,5,2,3,2,4,3,1,4,2,5,1,3,2,1,1,3,4,2,1,1,1,1,2,1,4,3,1,3,1,2,4,1,2,4,3,2,3,5,5,3,3,1,2,3,4,5,2,4,5,1,1,1,4,5,3,5,3,5,1,1,5,1,5,3,1,2,3,4,1,1,4,1,2,4,1,5,4,1,5,4,2,1,5,2,1,3,5,5,4,5,5,1,1,4,1,2,3,5,3,3,1,1,1,4,3,1,1,4,1,5,3,5,1,4,2,5,1,1,4,4,4,2,5,1,2,5,2,1,3,1,5,1,2,1,1,5,2,4,2,1,3,5,5,4,1,1,1,5,5,2,1,1];

    // Part one
    let mut model = Model::new(snapshot.clone()).into_iter();
    let mut num_fishes = model.nth(79).unwrap().len();
    println!("{}", num_fishes);

    // Part two 
    // bruteforce
    model = Model::new(snapshot).into_iter();
    num_fishes = model.nth(255).unwrap().len();
    println!("{}", num_fishes);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterator() {
        let mut snapshot = vec![0, 1, 2];
        let mut model = Model::new(snapshot);
        assert_eq!(model.next().unwrap(), vec![6, 0, 1, 8]);
        assert_eq!(model.next().unwrap(), vec![5, 6, 0, 7, 8]);
        assert_eq!(model.next().unwrap(), vec![4, 5, 6, 6, 7, 8]);
    }
}