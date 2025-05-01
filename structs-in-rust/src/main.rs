fn main() {
    let user = User {
        id: 1,
        username: String::from("john_doe"),
        email: String::from("john@doe.com"),
    };

    println!("User ID: {}", user.id);
    println!("Username: {}", user.username);
    println!("Email: {}", user.email);

    // struct init shorthand
    let username = String::from("jane_doe");
    let email = String::from("jane@doe.com");

    let user_b = User {
        id: 2,
        username,
        email,
    };

    // struct update syntax
    let user_c = { User {
        id: 3,
        ..user // must be last param of struct w/ no trailing comma
    }};

    // debugging with structs
    println!("User B: {:?}", user_b);
    println!("{:?}", user_c);

    // accessing fields of tuple struct
    let c = Color(255, 0, 0);
    println!("The color is RGB ({}, {}, {})", c.0, c.1, c.2);

    let another_unit = UnitStruct;
    println!("{}", another_unit.describe());

    let mut mushu = Dragon {
        name: "Mushu".to_string(),
        level: 1,
        fire_power: 10,
    };

    println!("{:?}", mushu);

    mushu.make_sound();
    mushu.attack();

    mushu.level_up();
    println!("{:?}", mushu);

    // use associated function to instantiate new dragon
    let new_dragon = Dragon::new("Smaug");
    println!("{:?}", new_dragon);
}

// struct with named fields
#[derive(Debug)] // necessary for using {:?}
struct User {
    id: i32,
    username: String,
    email: String,
}

// tuple struct
struct Color(i32, i32, i32);

// unit struct
/*
    useful for implementing traits on a type, but
    don't need to store data
*/
struct UnitStruct;

// define a trait
trait Describable {
    fn describe(&self) -> String;
}

// implement describable trait on UnitStruct
impl Describable for UnitStruct {
    fn describe(&self) -> String {
        "This is a unit struct implementing the Describable trait".to_string()
    }
}

#[derive(Debug)]
struct Dragon {
    name: String,
    level: u32,
    fire_power: u32,
}

// implementing methods
impl Dragon {
    // method with immutable reference to self
    fn make_sound(&self) {
        println!("{} roars!", self.name);
    }

    // method with mutable reference to self
    fn level_up(&mut self) -> (u32, u32) {
        self.level += 1;
        self.fire_power += 10;

        println!("{} leveled up to level {}!", self.name, self.level);

        (self.level, self.fire_power) // return tuple
    }

    fn attack(&self) {
        println!("{} attacks with a fireball!", self.name);
    }

    // associated function
    fn new(name: &str) -> Self {
        let level = 1;
        Self {
            name: name.to_string(),
            level,
            fire_power: level * 10,
        }
    }

}
