use std::collections::HashSet;
fn main() {
    let zn =2;
    let zm=3;
    
    let subgroups=generate_all_subgroups_product(zn,zm);

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


fn generate_all_subgroups_product(n:u64,m:u64)->Vec<HashSet<(u64,u64)>>{
    let subgroups_n=generate_all_subgroups(n);
    let subgroups_m=generate_all_subgroups(m);

    let mut subgroups_product=Vec::new();

    for subgroup_n in &subgroups_n{
        for subgroup_m in &subgroups_m{
            let mut subgroup_product=HashSet::new();
            for &a in subgroup_n{
                for &b in subgroup_m{
                    subgroup_product.insert((a,b));
                }
            }
            subgroups_product.push(subgroup_product)
        }
    }
    subgroups_product
}