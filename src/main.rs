// b2 🔛 b10
// b2 🔛 b16
// b10 🔛 b16

const BASE_16: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];
const BASE_10: &[&str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const BASE_2: &[&str] = &["0", "1"];

fn main() {
    let base_2 = Base::new_base(CommonBase::Base2);
    let base_10 = Base::new_base(CommonBase::Base10);
    let base_16 = Base::new_base(CommonBase::Base16);

    println!("[base 2] {:?}", base_2);
    println!("[base 10] {:?}", base_10);
    println!("[base 16] {:?}", base_16);
}

/// Convert base 2 to base 16 or base 10
#[derive(Debug)]
struct Base<'a> {
    base: usize,
    sys: Vec<&'a str>,
}

impl<'a> Base<'a> {
    fn new(sys: Vec<&'a str>) -> Self {
        Self {
            base: sys.len(),
            sys,
        }
    }

    fn new_base(base: CommonBase) -> Self {
        let sys = match base {
            CommonBase::Base2 => BASE_2,
            CommonBase::Base10 => BASE_10,
            CommonBase::Base16 => BASE_16,
        };

        Self {
            base: sys.len(),
            sys: sys.to_vec(),
        }
    }

    // fn from_b10<N: ToPrimitive>(num: N) -> Vec<u8> {
    //     let mut bits = Vec::new();
    //     let mut curr_num = num.to_i128().unwrap();
    //     loop {
    //         bits.push((curr_num % 2) as u8);
    //         curr_num = curr_num / 2;

    //         if curr_num == 0 {
    //             break;
    //         }
    //     }
    //     bits.reverse();

    //     bits
    // }
}

enum CommonBase {
    Base10,
    Base2,
    Base16,
}
