use rand::{Rng,SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32>{
    //RNGを初期化する．再現性を持たせるため毎回同じシード値を使う
    let mut rng = Pcg64Mcg::from_seed([0;16]);
    // let mut v = Vec::with_capacity(n);

    //0からn-1までの合計n回，繰り返し乱数を生成し，ベクタに追加する
    
    //for _ in 0..n {
        //RNGのsampleメソッドは引数として与えられた分布に従う乱数を一つ生成する
        //Standard分布は生成する値が数値型のときは一様分布になる
        //つまり，その型がとりうる全ての値が同じ確率でh捨現する
        //v.push(rng.sample(&Standard));
    //}

    //ベクタを返すだけ
    //v

    rng.sample_iter(&Standard).take(n).collect()

}

pub fn is_sorted_ascending<T:Ord>(x:&[T]) -> bool {
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T:Ord>(x:&[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}