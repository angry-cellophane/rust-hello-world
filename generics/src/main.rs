fn max<T: std::cmp::PartialOrd>(list: &[T]) -> Option<&T> {
    if list.len() == 0 {
        return None;
    }

    let mut max = &list[0];
    for v in list {
        if v > max {
            max = v;
        }
    }

    Some(max)
}

fn main() {
    let v = vec![1,2,3,4,5];
    let max = max(&v);
    println!("max = {:?}", max);
}
