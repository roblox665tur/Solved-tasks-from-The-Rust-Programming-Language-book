use crate::utils::*;
use std::collections::HashMap;
use std::io::Write;

pub fn company() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        clear_screen();
        println!("1.Action (remove/add/transfer/show department)");
        println!("2.Show all employees");
        println!("3.Exit ");

        let inp: usize = match input().parse() {
            Ok(n) => n,
            Err(err) => {
                println!("Invalid input: {}", err);
                stay_screen();
                continue;
            }
        };
        match inp {
            1 => {
                action(&mut company);
            }
            2 => {
                company_hashmap(&mut company);
            }
            3 => {
                return;
            }
            _ => {
                println!("Invalid input");
                stay_screen();
            }
        }
    }
}

pub fn company_hashmap(company: &mut HashMap<String, Vec<String>>) {
    clear_screen();
    if company.is_empty() {
        println!("Empty");
        stay_screen();
        return;
    }
    for (dep, employees) in company.into_iter() {
        println!("Department: {}", dep);
        print!("Employees: ");
        for employee in employees {
            print!("{}, ", employee);
        }
        println!();
        println!();
    }
    stay_screen();
}

fn action(company: &mut HashMap<String, Vec<String>>) -> Option<()> {
    clear_screen();
    print!("Input: ");
    std::io::stdout().flush().unwrap();

    let inp = input().to_ascii_lowercase();

    if let Some((output)) = parser(inp.as_str()) {
        match output.action {
            Action::Add => {
                add_employee(company, output.object?, output.destination);
            }
            Action::Remove => {
                remove_employee(company, output.object?, output.destination);
            }
            Action::Transfer => {
                transfer_employee(company, output.object?, output.source?, output.destination);
            }
            Action::Show => {
                show_department(company, output.destination);
            }
        }
    }
    Some(())
}

fn show_department(company: &HashMap<String, Vec<String>>, department: String) {
    if let Some(employees) = company.get(&department) {
        println!("Employees in {}: {:?}", department, employees);
    } else {
        println!("This department does not exist");
    }
    stay_screen();
}

pub fn add_employee(
    company: &mut HashMap<String, Vec<String>>,
    employee: String,
    department: String,
) {
    if !company.contains_key(&department) {
        company.insert(department, vec![employee]);
    } else {
        company.entry(department).or_default().push(employee);
    }
}

pub fn remove_employee(
    company: &mut HashMap<String, Vec<String>>,
    employee: String,
    department: String,
) {
    if !company.contains_key(&department) {
        println!("This department does not exist");
        return;
    }
    if let Some(employees) = company.get_mut(&department) {
        employees.retain(|e| e != &employee);
    }
}

pub fn transfer_employee(
    company: &mut HashMap<String, Vec<String>>,
    employee: String,
    department_from: String,
    department_to: String,
) {
    if !company.contains_key(&department_from) {
        println!("The department_from does not exist");
        return;
    }
    if let Some(employees) = company.get_mut(&department_from) {
        employees.retain(|e| e != &employee);
    }
    if !company.contains_key(&department_to) {
        company.insert(department_to, vec![employee]);
    } else {
        company.entry(department_to).or_default().push(employee);
    }
}
