use std::cmp::Ordering;




pub fn bubble(s :&mut Vec<i32>){
    let l = s.len();
    for i in 0..l{
        for j in 0..l{
            if s[i]< s[j]{
                let tmp = s[i];
                s[i] = s[j];
                s[j] = tmp;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_bubble() {
        let mut s = vec![1,3,2,4,10,99,123,50];
        bubble(&mut s);
        println!("{:?}",&s);
    }
 
}