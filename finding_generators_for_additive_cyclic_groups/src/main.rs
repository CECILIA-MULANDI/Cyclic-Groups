fn main() {
    let n=8;
    println!("The generators of {n} are:{:?} ",find_generators_one(n));
    println!("The generators of {n} are:{:?} ",find_generators_two(n));

}
//  Approach 1: Finding generators based on coprime numbers
// NOTE: ALL GROUPS OF (Z,+) are cyclic groups
// so we only need to find the generators of these cyclic groups
// all generators for cyclic additive groups are coprimes of n
fn find_generators_one(n:u32)->Vec<u32>{
   let mut generators=Vec::new();
    for i in 0..n{
        if gcd(i,n)==1{
            generators.push(i)
        }
    }
    generators
}
// this is the second approach to find generators
fn find_generators_two(n:u32)->Vec<u32>{
    let mut generators=Vec::new();
     for i in 0..n{
         if is_generator(i,n){
             generators.push(i)
         }
     }
     generators
}

/*
a=the value in question eg 3
n=mod eg mod 8

*/
fn is_generator(a:u32,n:u32)->bool{
    let mut generator=a;
    let mut count=1;
    // we need to find the order of the number a
    // if the order of a == n then it is a generator else not
    // the order is a*count=e(0)


    while generator !=0{
        generator=(generator+a)%n;
        count+=1;
        if count==n{
            break;
        }

    }
    // it will evaluate this to see if the count is equals to n
    // meaning if the order of the element is equals to n
    // if so then it evaluates to true meaning it is a generator 
    // else it is not
    count==n
}

fn gcd(a:u32,b:u32)->u32{
    if b==0{
        return a;
    }
    else{
        gcd(b,a%b)
    }
}