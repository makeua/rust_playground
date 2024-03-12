
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

fn str_str(haystack: String, needle: String) -> i32 {
    let mut str = haystack.chars().skip(0);
    let mut s = needle.chars();
    let mut i = 0;
    let mut j = 0;

    loop {
        let c = str.next();
        let v = s.next();

        if c == None || v == None { 
            break;
        }

        if c.unwrap() == v.unwrap() {
            i += 1;
            j += 1;
        } else {
            i -= j;
            i += 1;
            str = haystack.chars().skip(i);
            s = needle.chars();
            j = 0;
        }
    }
    if j >= needle.len() {
        return (i - j) as i32;
    }
    -1
}

fn main() {
    let mut v = vec![1];
    let r = remove_duplicates(&mut v);
    for i in 0..(r as usize) {
        print!("{} ", v[i]);
    }
}
