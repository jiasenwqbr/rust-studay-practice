fn main() {
    let mut ave = AveragedCollection {
        list: vec![100, 90, 70, 80],
        average: 0.0,
    };
    ave.add(60);
    ave.add(50);
    println!("the average is {}", ave.average());
    println!("the list is {:?}", ave.list);
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }
    pub fn average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
