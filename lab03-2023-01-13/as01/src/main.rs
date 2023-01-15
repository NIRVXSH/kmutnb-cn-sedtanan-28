//use std::collections::HashSet;
fn main() {
    
    //A
    let string = "this cat this bat this rat";

    //step1 word
    let mut split : Vec<&str> = string.split(" ").collect();
    
    //step2 uniqe
    //let uniqe: HashSet<_> = split.iter().cloned().collect(); import hashset เข้ามาแล้วใช้โคลนเวกเตอร์แล้วก็จัดลำดับแยกคำซ้ำออก
    //println!("{:?}", uniqe);
    split.sort_unstable();
    split.dedup();
    

    //step3 count
    let mut length=split.len();

    //B
    println!("{}", length);
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_splitandcount() {
       // Set the expected output 
       let string = "this cat this bat this rat";
       let mut split : Vec<&str> = string.split(" ").collect();
       split.sort_unstable();
        split.dedup();
        let mut length=split.len();
       assert_eq!(length,4)
   }
}

