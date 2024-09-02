mod generators_addtive_groups;
// mod generators_multiplicative_groups;
use generators_addtive_groups::generators::find_generators_one;
// use generators_multiplicative_groups::generator::find_generators;
fn euler_totient(mut n: u64) -> u64 {
    let mut p = 2;
    let mut result = n;
    
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    
    result
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn find_generators(n: u64) -> Vec<u64> {
    let mut generators = Vec::new();
    let totient = euler_totient(n);
    
    for g in 2..n {
        if gcd(g, n) != 1 {
            continue; // Skip if g is not coprime with n
        }
        
        let mut count = 1;
        let mut res = g % n;
        while res != 1 {
            res = (res * g) % n;
            count += 1;
        }
        if count == totient {
            generators.push(g);
        }
    }
    
    generators
}

fn main() {
    let zn: u64 = 7;
    let zm:u64=9;
    let additive_gen=find_generators_one(zn);
    let additive_gen_two=find_generators_one(zm);
    println!("The generators for Z_{zn} are {:?}",additive_gen);
    println!("The generators for Z_{zm} are {:?}",additive_gen_two);
    let multiplicative_gen = find_generators(zn);
    if multiplicative_gen.is_empty() {
        println!("Z_{}* has no generators", zn);
    } else {
        println!("Generators for Z_{}* (multiplicative group) are {:?}", zn, multiplicative_gen);
    }
    let mut all_generators=Vec::new();
    // for Zn*Zm*
    for i in &additive_gen{
        for j in &multiplicative_gen{
            all_generators.push((i,j));     
        }
    }
    println!("The generators of Z_{zn} * Z_{zn}* are {:?}",all_generators);

    // for Zn*Zn
    for i in &additive_gen{
        for m in &additive_gen_two{
            all_generators.push((i,m));     
        }
    }
    println!("The generators of Z_{zn} * Z_{zm} are {:?}",all_generators);
}
