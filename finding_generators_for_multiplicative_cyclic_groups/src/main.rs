fn main() {
    let n = 7;
    println!("The generators of Z_{n}* are {:?}",find_generators(n))
   
}


fn euler_totient(mut n:u32)->u32{
    let mut p=2;
    let mut result=n;
    
    while p*p<=n{
        if n%p==0{
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    // If n is greater than 1, then it's a prime number larger than sqrt(n)
    if n > 1 {
        result -= result / n;
    }
    
    result
}
fn find_generators(n:u32)->Vec<u32>{
    
    let mut generators=Vec::new();
    for g in 2..n{
        let mut count=1;
        let mut res = g % n;
        while res !=1{
            res= (res*g) % n;
            count +=1;
        }
        if count == euler_totient(n){
            generators.push(g);
        }  
        
    }
    generators
    
    
}
