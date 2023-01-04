use std::fmt::Debug;
pub fn bubble_sort<T:PartialOrd + Debug>(v: &mut [T]){
    println!("length of the vec is {:?}",v.len());
    for p in 0..v.len(){
        println!("{:?}", p);
        let mut sorted = true;
        for i in 0..(v.len()-1) - p{
            if v[i] > v[i+1]{
                v.swap(i, i+1);
                sorted = false;
            }  
        }
        println!("{:?}", v);
        if sorted{
            return;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug> (mut v: Vec<T>) -> Vec<T>{

    //sort the left half,
    //sort the right half, O(n * ln(n))  1024 * 10 < 1024 * 1024
    //bring the sorted halfs together -> O(n)
    println!("MergeSort {:?}",v);
    if v.len()<=1{
        return v;
    }
    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);
    //bring them together again add whiechever is lower the fornt of a or font of b

    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();

    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some( ref a_val) => match b_peek{
                Some( ref b_val) =>{

                 if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                }else{
                    res.push(a_peek.take().unwrap());
                    a_peek = a_it.next();
                }
            }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                   return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek{
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }, 
        }
    }
}

pub fn pivot<T: PartialOrd + Debug>(v: &mut [T]) -> usize{
    let mut p = 0;

    for i in 1..v.len() {
        //move our pivot forward 1,  and put this element before it.
        if v[i] < v[p] {
            v.swap(p+1, i);
            v.swap(p, p+1);
            p += 1;
        }
        println!("p value :  {}",p);
    }
    p
}


pub fn quicker_sort<T:PartialOrd + Debug>(v: &mut [T]){

    if v.len() <= 1{
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);

    let (a,b) = v.split_at_mut(p);
    quicker_sort(a);
    quicker_sort(&mut b[1..]); //middle element already sorted

}
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![10,1,3,12,4,5,2];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1,2,3,4,5,10,12]);

    }

    #[test]
    fn test_merge_sort(){
        let v = vec![10,1,3,12,4,5,2];
        let v = merge_sort(v);
        assert_eq!(v, vec![1,2,3,4,5,10,12]);
    }
    #[test]
    fn test_pivots() {

        let mut qv = vec![8, 7, 2, 1, 0, 9, 6];
        let p = pivot(&mut qv);
        println!("p value :: {}",p);
        for x in 0..qv.len(){
            assert!((qv[x] < qv[p]) == (x < p)) ;
        }
       // assert_eq!(qv, vec![0,1,2,6,7,8,9]);
    }
    #[test]
    fn test_quicker_sort(){

        let mut qv = vec![8, 7, 2, 1, 0, 9, 6];
        quicker_sort(&mut qv);
        assert_eq!(qv, vec![1,2,0,6,7,8,9]);
    }

 }
