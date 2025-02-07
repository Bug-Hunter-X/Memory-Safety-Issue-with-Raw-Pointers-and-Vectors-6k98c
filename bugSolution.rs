fn main() {
    let mut v = vec![1, 2, 3];
    let len = v.len();
    let mut v_copy = Vec::with_capacity(len);
    unsafe {
        std::ptr::copy_nonoverlapping(v.as_ptr(), v_copy.as_mut_ptr(), len);
    }
    let ptr = v_copy.as_mut_ptr();
    unsafe {
        *ptr = 10;
    }
    println!("{:?}", v_copy);
} 