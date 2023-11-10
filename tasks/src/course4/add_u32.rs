

pub fn add_u32(s:&[u32])-> Option<u32>{
    let mut ret = 0;
    for i in s.iter(){
        let tmp = ret;
        ret = ret + i;
        if ret < tmp{
            return None;
        }
    }

    return Some(ret);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let i : [u32;2]= [1,2];
       let lt = add_u32(&i);
       assert_eq!(lt,Some(3));
    }
}
