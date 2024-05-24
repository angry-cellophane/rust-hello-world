fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

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

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}

fn main() {
    let v = vec![1,2,3,4,5];
    let max = max(&v);
    println!("max = {:?}", max);

    let s1 = String::from("hello");
    let s3: &str;
    {
        let s2 = String::from("world!");
        s3 = longest(s1.as_str(), s2.as_str());
        println!("longest string = {}", s3);
    }

    let words = String::from("Hello world!");
    let first_word = first_word(words.as_str());
    println!("first word = {}", first_word);
}
