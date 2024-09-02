use std::collections::HashSet;
fn main() {
    let n =4;
    
    let subgroups=generate_all_subgroups(n);

    println!("{:?}",subgroups);
}
fn divisors(n:u64)->Vec<u64>{
    let mut div=Vec::new();
    for i in 1..=n{
        if n%i==0{
            div.push(i)
        }
    }
    div
}

fn generate_subgroup(g:u64,n:u64)->HashSet<u64>{
    let mut subgroup=HashSet::new();
    let mut current=0;
    while !subgroup.contains(&current){
        subgroup.insert(current);
        current=(current+g)%n;
    }
    subgroup
}

fn generate_all_subgroups(n:u64)->Vec<HashSet<u64>>{
    let mut subgroups =Vec::new();
    for d in divisors(n){
        let generator=n/d;
        let subgroup=generate_subgroup(generator,n);
        subgroups.push(subgroup)
    }
    subgroups
}