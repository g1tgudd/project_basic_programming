use std::collections::HashMap;
use chrono::prelude::*;
use std::io;

#[derive(Clone, Debug)]
struct Employee {
    name: String,
    id: i32
}

struct Employees {
    inner: HashMap<String, Employee>
}

impl Employees {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn add(&mut self, emp: Employee) {
        self.inner.insert(emp.name.clone(), emp);s
    }

    fn get(&self) -> Vec<&Employee> {
        let mut employees = vec![];

        for emp in self.inner.values() {
            employees.push(emp)
        }
        employees
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name: &str, id: i32) -> bool {
        match self.inner.get_mut(name) {
            Some(emp) => {
                emp.id = id;
                true
            },
            None => false
        }
    }
}

fn get_input() -> Option<String> {
    let mut errormsg = String::new();
    while io::stdin().read_line(&mut errormsg).is_err() {
        println!("Please enter your name again")
    }
    let input = errormsg.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_id() -> Option<i32> {
    print!("Status: ");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<i32, _> = input.parse();
        match parsed_input {
            Ok(id) => return Some(id),
            Err(_) => println!("Please enter a valid employee ID!")
        }
    }
}

fn add_employee(employees: &mut Employees) {
    println!("Employee name: ");
    let name = match get_input() {
        Some(input) => input,
        None => return
    };
    println!("Employee ID: ");
    let id  = match get_id() {
        Some(id) => id,
        None => return
    };
    let employee = Employee { name, id };
    employees.add(employee);
    println!("Employee added");
}

fn view_employee(employees: &mut Employees) {
    for emp in employees.get() {
        println!("=== Employee List ===");
        println!("{:?}", emp);
    }
}

fn remove_employee(employees: &mut Employees) {
    for emp in employees.get() {
        println!("{:?}", emp);
    }
    println!("Enter Employee name to remove:");
    let name = match get_input() {
        Some(input) => input,
        None => return
    };
    if employees.remove(&name) {
        println!("Status: Employee removed!")
    } else {
        println!("Status: Employee not found!")
    }
}

fn update_employeeid(employees: &mut Employees) {
    for emp in employees.get() {
        println!("{:?}", emp);
    }
    println!("Enter employee name:");
    let name = match get_input() {
        Some(input) => input,
        None => return
    };
    println!("Enter updated employee ID:");
    let id  = match get_id() {
        Some(id) => id,
        None => return
    };
    if employees.update(&name, id) {
        println!("Employee ID updated")
    } else {
        println!("Employee not found!")
    }
}

fn main_menu() {
    fn show() {
        println!("");
        let local: DateTime<Local> = Local::now();
        println!("{}", local);
        println!("=== Employee Database ===");
        println!("1. Add Employee");
        println!("2. View Employees");
        println!("3. Remove Employee");
        println!("4. Update Employee ID");
        println!("Enter Selection:");
    }

    let mut employees = Employees::new();

    loop {
        show();
        let input = match get_input() {
            Some(input) => input,
            None => return
        };
        match input.as_str() {
            "1" => add_employee(&mut employees),
            "2" => view_employee(&mut employees),
            "3" => remove_employee(&mut employees),
            "4" => update_employeeid(&mut employees),
            _ => break
        }
    }
}

fn main() {
    main_menu();
}