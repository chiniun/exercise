use std::cmp::Ordering;

pub fn bubble(s: &mut Vec<i32>) {
    let l = s.len();
    for i in 0..l {
        for j in 0..l {
            if s[i] < s[j] {
                let tmp = s[i];
                s[i] = s[j];
                s[j] = tmp;
            }
        }
    }
}

pub fn trait_bubble<T: Ord + Copy>(s: &mut [T]) {
    let l = s.len();
    for j in 0..l {
        for i in 0..(l - j) {
            if i + 1 >= l {
                break;
            }
            if Ordering::Greater == s[i].cmp(&s[i + 1]) {
                let tmp = s[i];
                s[i] = s[i + 1];
                s[i + 1] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_bubble() {
        let mut s = vec![1, 3, 2, 4, 10, 99, 123, 50];
        bubble(&mut s);
        println!("{:?}", &s);
    }
}
