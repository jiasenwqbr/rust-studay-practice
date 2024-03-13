// fn main() {
//     // let v1 = vec![1, 2, 3];
//     // let v1_iter = v1.iter();
//     // println!("{:?}", v1);
//     // println!("{:?}", v1_iter);
//     // for v in v1_iter {
//     //     println!("{}", v);
//     // }

//     // let v1 = vec![1, 2, 3];
//     // let v1_iter = v1.iter();
//     // let total: i32 = v1_iter.sum();
//     // println!("{}", total);

//     let v1: Vec<i32> = vec![1, 2, 3];
//     let _ = v1.iter().map(|x| x + 1);
//     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

//     println!("{:?}", v1);
//     println!("{:?}", v2);
// }

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
