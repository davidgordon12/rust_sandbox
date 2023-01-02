fn main() {
    let mut array: [usize; 50] = [0; 50];

    for i in 0..array.len()         // exclusive of the top number
    {
        array[i] = i * i;
    }

    for i in 0..=array.len() - 1    //inclusive of the top number
    {                           
        array[i] = i * (i + 1);
    }

    println!("{:#?}", array);
}

fn build_vector
{
    
}
