use std::collections::HashSet;
fn main() {
    // println!("{:?}",generate_all_subgroups(8));
    println!("{:?}",generate_subgroups(15,7));
}
fn gcd(mut a:u32,mut b:u32)->u32{
    while b!=0{
        let temp=b;
        b=a%b;
        a=temp;
    }
    a
}
// ones relatively prime to n
fn elements_of_multiplicative_groups(n:u32)->Vec<u32>{
    let mut elements=Vec::new();
    for i in 1..n{
        if gcd(i,n)==1{
            elements.push(i)
        }
    }
    elements
}
// generate cyclic subgroups
fn generate_subgroups(n:u32,generator:u32)->Vec<u32>{
    let mut subgroup=Vec::new();
    let mut current=generator%n;
    println!("{current}");

    while !subgroup.contains(&1){
        subgroup.push(current);
        // println!("{current}");
        current=(current*generator)%n;
        
    }
    subgroup.sort_unstable();
    subgroup

}

// generates all groups
fn generate_all_subgroups(n:u32)->Vec<Vec<u32>>{
    let elements=elements_of_multiplicative_groups(n);
    let mut subgroups=Vec::new();
    let mut seen_groups=HashSet::new();
    if !elements.is_empty(){
        subgroups.push(elements.clone());
        seen_groups.insert(elements.clone());
    }
    for &e in &elements{
        let subgroup=generate_subgroups(n,e);
        if !seen_groups.contains(&subgroup){
            seen_groups.insert(subgroup.clone());
            subgroups.push(subgroup)
        }
    }
    subgroups

}
