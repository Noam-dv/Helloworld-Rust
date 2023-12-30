use std::cmp;
use std::convert::TryInto;

fn main() {
    let _tuple = ("a", 10);
    let _vector = vec![1,2,3,4,5,6,7,8];
    let _test_data_type = {
        let _a:i32 = 1;
        let _b:i32 = 2;
    };

    let _min:i32 = cmp::min(10, 20);
    let _max:i32 = cmp::max(20, 30);


    let _str:&str = "test";
    let _str_len:i32 = _str.len().try_into().unwrap();
    
    println!("Process not interrupted");
}  
