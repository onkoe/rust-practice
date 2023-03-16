struct Person {
    name: String,
    age: u8,
    email: String,
}

impl Person {
    fn new() -> Person {
        Person {
            name: "Unnamed Person".to_string(),
            age: 0,
            email: "template@rust-lang.org".to_string(),
        }
    }

    fn yellow_pages(lookup: Person) {
        println!(
            "{} is {} years old. You can reach them at {}!",
            lookup.name, lookup.age, lookup.email
        );
    }

    fn set_name(mut self, new_name: String) -> Person {
        self.name = new_name;
        return self;
    }

    fn set_age(mut self, new_age: u8) -> Person {
        self.age = new_age;
        return self;
    }

    fn set_email(mut self, new_email: String) -> Person {
        self.email = new_email;
        return self;
    }
}

fn main() {
    let bruh = Person::new();
    let oakley: Person = Person::new() // he's definitely a person
        .set_age(4)
        .set_name("Oakley".to_string())
        .set_email("oakley@ou.edu".to_string());

    Person::yellow_pages(bruh);
    println!();
    Person::yellow_pages(oakley);
}
