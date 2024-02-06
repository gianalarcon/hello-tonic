use hello::greeter_client::GreeterClient;
use hello::HelloRequest;

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Default)]
struct Employee {
    name: String,
    gender: String,
    age: String,
    company: String,
    department: String,
    id: String,
    location: String,
}

// Implement fn new() for Employee, Use #[Default] to set default values
impl Employee {
    fn new() -> Employee {
        Default::default()
    }

    fn collect_info(&mut self) {
        self.name = get_input("Name: ");
        self.gender = get_input("Gender: ");
        self.age = get_input("Age: ");
        self.company = get_input("Company: ");
        self.department = get_input("Department: ");
        self.id = get_input("ID: ");
        self.location = get_input("Location: ");
    }
}

fn get_input(x: &str) -> String {
    println!("{:?}", x);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    println!("Enter START");
    let mut conv_status = String::new();
    std::io::stdin().read_line(&mut conv_status).unwrap();
    if conv_status.trim() == "START".to_string() {
        let mut emp = Employee::new();
        emp.collect_info();
        let request = tonic::Request::new(HelloRequest {
            name: emp.name.into(),
            gender: emp.gender.into(),
            age: emp.age.into(),
            company: emp.company.into(),
            department: emp.department.into(),
            id: emp.id.into(),
            location: emp.location.into(),
        });
        println!("Sending request to GRPC server...");
        let response = client.say_hello(request).await?;
        println!("RESPONSE={:?}", response);
        Ok(())
    } else {
        panic!("Invalid input");
    }
}
