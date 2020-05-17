use super::SortOrder;

pub fn sort<T:Ord>(x:&mut[T],order:&SortOrder) -> Result<(),String>{
    if x.len().is_power_of_two() {
        match *order{
            SortOrder::Ascending => do_sort(x, true),
            SortOrder::Descending => do_sort(x, false),
        };
        Ok(())
    }else{
        Err(format!("The length of x is not a power of two. (x.len(): {}.",x.len()))
    }
    
}

fn do_sort<T:Ord>(x: &mut [T],up:bool){
    //未実装の意味．コンパイルは通るが実行できない．
    if x.len() > 1{
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point],true);
        do_sort(&mut x[mid_point..],false);
        sub_sort(x, up)
    }
}


//ここから下はpubがついていないので，外からアクセスはできない．

fn sub_sort<T:Ord>(x: &mut [T],up:bool){
    if x.len() > 1{
        compare_and_swap(x, up);
        let mid_point = x.len()/2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T:Ord>(x: &mut[T],up:bool){
    let mid_point = x.len()/2;
    for i in 0..mid_point{
        if (x[i] > x[mid_point+i]) == up {
            x.swap(i, mid_point+i);
        }
    }
}


// -------------test-------------
#[cfg(test)]
mod tests{
    //親モジュールの(first)sort関数を使用する
    use super::sort;
    use crate::SortOrder::*;

    //#testのついた関数はテスト時のみ実行される
    #[test]
    fn sort_str_ascending(){
        let mut x = vec!["Rust","is","fast","and","memory-effecient","with","no","GC"];
        assert_eq!(sort(&mut x,&Ascending),Ok(()));
        assert_eq!(x,vec!["GC","Rust","and","fast","is","memory-effecient","no","with"]);

    }

    /** f64はOrdトレイトが実装されていないため，コンパイルが通らない
     *  #[test]
        fn sort_f64_ascending(){
            let mut x = vec![20.9,20.0,-10.0,11.0];
            sort(&mut x,true);
            assert_eq!(x,vec![-10.0,11.0,20.0,20.9]);
        } 
    */ 

    #[test]
    fn sort_str_descending(){
        let mut x = vec!["Rust","is","fast","and","memory-effecient","with","no","GC"];
        assert_eq!(sort(&mut x, &Descending),Ok(()));
        assert_eq!(x,vec!["with","no","memory-effecient","is","fast","and","Rust","GC"]);
    }

    #[test]
    fn sort_to_fail() {
        let mut x = vec![10,1,43];
        assert!(sort(&mut x, &Ascending).is_err());
    }
    
}