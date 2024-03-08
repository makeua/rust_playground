
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut k = 0;
    let mut a = nums[0];
    let mut i = 1;
    while i < nums.len() {
        if a != nums[i] {
            nums[k] = a;
            k += 1;
            a = nums[i];
        }
        i += 1;
    }
    nums[k] = a;
    k += 1;
    k as i32
}

fn main() {
    let mut v = vec![1];
    let r = remove_duplicates(&mut v);
    for i in 0..(r as usize) {
        print!("{} ", v[i]);
    }
}
