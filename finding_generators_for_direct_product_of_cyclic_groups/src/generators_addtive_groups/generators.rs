// find the generators for Zn (additive group)
//  Approach 1: Finding generators based on coprime numbers
// NOTE: ALL GROUPS OF (Z,+) are cyclic groups
// so we only need to find the generators of these cyclic groups
// all generators for cyclic additive groups are coprimes of n

pub fn find_generators_one(n:u64)->Vec<u64>{
    let mut generators=Vec::new();
     for i in 0..n{
         if gcd(i,n)==1{
             generators.push(i)
         }
     }
     generators
 }
fn gcd(a:u64,b:u64)->u64{
    if b==0{
        return a;
    }
    else{
        gcd(b,a%b)
    }
}