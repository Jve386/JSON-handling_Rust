use serde::{Deserialize, Serialize};

// Define a struct 'Person' with attributes: name (String), age (u8), and phones (Vec<String>).
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

// Define a struct 'Company' with attributes: name (String) and address (String).
#[derive(Serialize, Deserialize)]
struct Company {
    name: String,
    address: String,
}

fn main() {
    // Create an instance of 'Person'.
    let person: Person = Person {
        name: String::from("Javier de la Vega"),
        // Parse the string "30" to u8 for the 'age' field.
        age: "30".parse().expect("Failed to parse age"),
        phones: vec![String::from("555-1234"), String::from("555-4321")],
    };

    // Create an instance of 'Company'.
    let company: Company = Company {
        name: String::from("Jve386"),
        address: String::from("Street 123"),
    };

    // Serialize 'person' to a JSON string and print it.
    let json_person = serde_json::to_string(&person).unwrap();
    println!("Person: {}", json_person);

    // Serialize 'company' to a JSON string and print it.
    let json_company = serde_json::to_string(&company).unwrap();
    println!("Company: {}", json_company);
}
