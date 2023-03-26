
mod bubble_sort;
mod c4;
use crate::c4::Duration;
use crate::c4::RECT;
use crate::c4::SQUARE;
use crate::c4::TRIANG;
use crate::c4::calc;
fn main() {
    //1. bubble sort
    let mut arr = [3, 2, 1, 4];
    println!("before sort {:?}", arr);
    bubble_sort::bubble_sort(&mut arr);
    println!("after sort {:?}", arr);

    //2. C4-1 Light
    let light = c4::Light::GREEN;
    println!("duration {}", light.duration());
    let light = c4::Light::YELLOW;
    println!("duration {}", light.duration());
    let light = c4::Light::RED;
    println!("duration {}", light.duration());

    //3. C4-2 sum
    let arr: [u32;5] = [1, 2, 3, 4, 5];
    match c4::sum(&arr) {
        Some(v) => {
            println!("sum is {}", v);
        }
        None => {
            println!("sum is overlow");
        }
    }
    let arr: [u32;5] = [u32::max_value() - 1, 2, 3, 4, 5];
    match c4::sum(&arr) {
        Some(v) => {
            println!("sum is {}", v);
        }
        None => {
            println!("sum is overlow");
        }
    }

    //4. C4-3 area
    let t = TRIANG {
        base: 2,
        height: 10,
    };
    println!("triangle area:{}", calc(t));
    let t = SQUARE {
        length: 2,
    };
    println!("SQUARE area:{}", calc(t));
    let t = RECT{
        length: 2,
        weight: 4,
    };
    println!("RECT area:{}", calc(t));
}
