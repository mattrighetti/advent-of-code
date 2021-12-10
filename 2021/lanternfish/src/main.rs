struct Model {
    snapshot: Vec<i64>,
    day: i64
}

impl Model {
    fn new(snapshot: &Vec<i64>) -> Self {
        let v_0 = snapshot.into_iter().map(|x| x.to_owned()).filter(|x| *x == 0).collect::<Vec<i64>>().len() as i64;
        let v_1 = snapshot.into_iter().map(|x| x.to_owned()).filter(|x| *x == 1).collect::<Vec<i64>>().len() as i64;
        let v_2 = snapshot.into_iter().map(|x| x.to_owned()).filter(|x| *x == 2).collect::<Vec<i64>>().len() as i64;
        let v_3 = snapshot.into_iter().map(|x| x.to_owned()).filter(|x| *x == 3).collect::<Vec<i64>>().len() as i64;
        let v_4 = snapshot.into_iter().map(|x| x.to_owned()).filter(|x| *x == 4).collect::<Vec<i64>>().len() as i64;
        let v_5 = snapshot.into_iter().map(|x| x.to_owned()).filter(|x| *x == 5).collect::<Vec<i64>>().len() as i64;
        let v_6 = snapshot.into_iter().map(|x| x.to_owned()).filter(|x| *x == 6).collect::<Vec<i64>>().len() as i64;
        let v_7 = snapshot.into_iter().map(|x| x.to_owned()).filter(|x| *x == 7).collect::<Vec<i64>>().len() as i64;
        let v_8 = snapshot.into_iter().map(|x| x.to_owned()).filter(|x| *x == 8).collect::<Vec<i64>>().len() as i64;

        Model {
            snapshot: vec![v_0, v_1, v_2, v_3, v_4, v_5, v_6, v_7, v_8],
            day: 0 
        }
    }
}

impl Iterator for Model {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.day += 1;

        let newborns = self.snapshot[0];
        self.snapshot[0] = self.snapshot[1];
        self.snapshot[1] = self.snapshot[2];
        self.snapshot[2] = self.snapshot[3];
        self.snapshot[3] = self.snapshot[4];
        self.snapshot[4] = self.snapshot[5];
        self.snapshot[5] = self.snapshot[6];
        self.snapshot[6] = self.snapshot[7] + newborns;
        self.snapshot[7] = self.snapshot[8];
        self.snapshot[8] = newborns;

        self.snapshot.clone()
            .into_iter()
            .reduce(|x, y| x + y)
    }
}

fn main() {
    let snapshot = vec![3,5,1,2,5,4,1,5,1,2,5,5,1,3,1,5,1,3,2,1,5,1,1,1,2,3,1,3,1,2,1,1,5,1,5,4,5,5,3,3,1,5,1,1,5,5,1,3,5,5,3,2,2,4,1,5,3,4,2,5,4,1,2,2,5,1,1,2,4,4,1,3,1,3,1,1,2,2,1,1,5,1,1,4,4,5,5,1,2,1,4,1,1,4,4,3,4,2,2,3,3,2,1,3,3,2,1,1,1,2,1,4,2,2,1,5,5,3,4,5,5,2,5,2,2,5,3,3,1,2,4,2,1,5,1,1,2,3,5,5,1,1,5,5,1,4,5,3,5,2,3,2,4,3,1,4,2,5,1,3,2,1,1,3,4,2,1,1,1,1,2,1,4,3,1,3,1,2,4,1,2,4,3,2,3,5,5,3,3,1,2,3,4,5,2,4,5,1,1,1,4,5,3,5,3,5,1,1,5,1,5,3,1,2,3,4,1,1,4,1,2,4,1,5,4,1,5,4,2,1,5,2,1,3,5,5,4,5,5,1,1,4,1,2,3,5,3,3,1,1,1,4,3,1,1,4,1,5,3,5,1,4,2,5,1,1,4,4,4,2,5,1,2,5,2,1,3,1,5,1,2,1,1,5,2,4,2,1,3,5,5,4,1,1,1,5,5,2,1,1];

    // Part one
    let mut model = Model::new(&snapshot).into_iter();
    let mut num_fishes = model.nth(79).unwrap();
    println!("80 days: {}", num_fishes);

    // Part two
    model = Model::new(&snapshot).into_iter();
    num_fishes = model.nth(255).unwrap();
    println!("256 days: {}", num_fishes);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_model() {
        let snapshot = vec![0, 1, 2, 8, 8];
        let model = Model::new(&snapshot);
        assert_eq!(1, model.snapshot[0]);
        assert_eq!(1, model.snapshot[1]);
        assert_eq!(1, model.snapshot[2]);
        assert_eq!(0, model.snapshot[3]);
        assert_eq!(0, model.snapshot[4]);
        assert_eq!(0, model.snapshot[5]);
        assert_eq!(0, model.snapshot[6]);
        assert_eq!(0, model.snapshot[7]);
        assert_eq!(2, model.snapshot[8]);
    }

    #[test]
    fn test_iterator() {
        let snapshot = vec![0, 1, 2];
        let mut model = Model::new(&snapshot);
        assert_eq!(1, model.snapshot[0]);
        assert_eq!(model.next().unwrap(), 4);
        assert_eq!(model.next().unwrap(), 5);
        assert_eq!(model.next().unwrap(), 6);
    }
}