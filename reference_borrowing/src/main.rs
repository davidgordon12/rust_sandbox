fn main() 
{
    let mut ms = String::from("string on heap");

    let len = calculate_length(&ms); // passing a reference so that ms doesn't get dropped

    println!("The length of <{}> is <{}>.", ms, len); // we can still use ms here since it's ownership wasn't moved and dropped

    /* ================================== */

    change_mutable(&mut ms); // 

    let len = calculate_length(&ms); // passing a reference so that ms doesn't get dropped

    println!("The length of <{}> is <{}>.", ms, len); // we can still use ms here since it's ownership wasn't moved and dropped
}

fn calculate_length(s: &String) -> usize
{
    s.trim().len()
}

    /* ================================== */


/*fn change(s: &String) 
{
    s.push_str(" with push"); // this breaks because we are simply borrowing s. we cannot modify something we dont own
}*/

fn change_mutable(s: &mut String) 
{
    s.push_str(" with push"); // this works because we are telling the compiler that it has permission to mutate s
}