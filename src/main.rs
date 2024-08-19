use std::{
    collections::HashMap,
    io::{self, Write},
};

struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    fn add_employee(&mut self, name: &str, department: &str) {
        self.departments
            .entry(department.to_string())
            .or_insert_with(Vec::new)
            .push(name.to_string())
    }

    fn get_department(&self, department: &str) -> Option<&Vec<String>> {
        self.departments.get(department)
    }
}

fn main() {
    let mut company = Company::new();

    loop {
        print!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {

            ["List" ,"all"] => {
                for d in &company.departments {
                    println!("Department - {}", d.0);
                    for e in d.1 {
                        println!("{}", e);
                    }
                }
            }

            ["List", department] => match company.get_department(department) {
                Some(employees) => {
                    println!("Employees in {}:", department);
                    for e in employees {
                        println!("{}", e);
                    }
                }
                None => {
                    println!("No employees in - {}", department);
                }
            },

            ["Add", name, "to", department] => {
                company.add_employee(name, department);
                println!("Employee {} added to {}", name, department);
            }

            _ => println!("Invalid command. Use 'Add [name] to [department]', 'List [department]', or 'List all'")
        }
    }
}
