use std::vec;

fn main()
{
    let classes: Vec<&'static str> = vec!["Application Design", "Software QA", "Small Business Solution"];

    println!("{:#?}", Person::create_person("David", 20, String::from("David"), classes))
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person
{
    name: &'static str,
    age: u8,
    major: String,
    classes: Vec<&'static str>
}

impl Person 
{
    fn create_person(name: &'static str, age: u8, 
        major: String, classes: Vec<&'static str>) -> Person
    {
        Person
        {
            name: name,
            age: age,
            major: major,
            classes: classes
        }
    }
}