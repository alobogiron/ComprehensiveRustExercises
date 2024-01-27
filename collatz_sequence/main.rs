fn collatz_length(n: i32) -> u32{
    let mut n_len = 0;
    if n == 1{
        n_len =  n_len + 1;
    }else{
        n_len += 1;
        if (n % 2) == 0{
           n_len +=  collatz_length(n/2);
        }else{
           n_len += collatz_length((3*n)+1);
        }
    }

    n_len

}

fn main (){
    let n = 4;
    println!("{}", collatz_length(n));
}
