use std::collections::BTreeMap;

mod input;

type Employee = String;
type Department = Vec<Employee>;
type Company = BTreeMap<String, Department>;

#[derive(Copy, Clone, Debug)]
enum UserAction {
    AddNewDepartment,
    AddEmployeeToDepartment,
    ListDepartments,
    ListDepartmentEmployees,
    ListCompanyEmployeesByDepartment,
    Quit,
}

fn read_employee_name() -> Employee {
    input::read_line("Enter employee name:")
}

fn add_new_department(company: &mut Company) -> () {
    let department_name = input::read_line("Enter department name:");
    if company.contains_key(&department_name) {
        println!("Department '{department_name}' already exists.");
    } else {
        company.insert(department_name.clone(), Vec::new());
        println!("Department '{department_name}' successfully added.");
    }
}

fn read_department<'a>(company: &'a Company, prompt: &str) -> Option<(&'a String, &'a Department)> {
    if company.is_empty() {
        println!("No departments exist yet. Add new departments first.");
        return None;
    }

    let department_names: Vec<&String> = company.keys().collect();

    let menu_items: Vec<_> = department_names
        .iter()
        .map(|&name| input::MenuItem {
            label: name,
            option: name,
        })
        .collect();

    let name = input::read_menu_option(menu_items.iter(), prompt);
    let department = company.get(name)?;
    Some((name, department))
}

fn read_department_mut<'a>(
    company: &'a mut Company,
    prompt: &str,
) -> Option<(String, &'a mut Department)> {
    if company.len() == 0 {
        println!("No departments exist yet. Add new departments first.");
        return None;
    }

    let department_names: Vec<&String> = company.keys().collect();

    let menu_items: Vec<_> = department_names
        .iter()
        .map(|&name| input::MenuItemRef {
            label: name,
            option: name,
        })
        .collect();

    let name = input::read_menu_option_ref(menu_items.iter(), prompt).clone();
    let department = company.get_mut(&name)?;
    Some((name, department))
}

fn add_employee_to_department(company: &mut Company) -> () {
    let (name, department) = match read_department_mut(company, "Select department:") {
        Some((name, department)) => (name, department),
        None => return,
    };

    let employee_name = read_employee_name();
    if department.contains(&employee_name) {
        println!("Employee '{employee_name}' is already in department '{name}'.");
    } else {
        department.push(employee_name.clone());
        department.sort();
        println!("Employee '{employee_name}' successfully added to department '{name}'.");
    }
}

fn list_departments(company: &Company) -> () {
    if company.is_empty() {
        println!("No departments exist yet. Add new departments first.");
        return;
    }

    println!("Departments:");
    for department_name in company.keys() {
        println!("    {department_name}");
    }
}

fn list_selected_department_employees(company: &Company) -> () {
    let (_name, department) = match read_department(&company, "Select department:") {
        Some(department) => department,
        None => return,
    };

    list_department_employees(&department);
}

fn list_department_employees(department: &Department) -> () {
    if department.is_empty() {
        println!("    No employees in department.");
        return;
    }

    for employee in department {
        println!("	{employee}");
    }
}

fn list_company_employees_by_department(company: &Company) -> () {
    for (name, department) in company.iter() {
        println!("{name}:");
        list_department_employees(department);
    }
}

const NUM_ACTIONS: usize = 6;
fn get_action_menu() -> &'static [input::MenuItem<'static, UserAction>; NUM_ACTIONS] {
    static ACTION_MENU: [input::MenuItem<UserAction>; NUM_ACTIONS] = [
        input::MenuItem {
            label: "Add new department",
            option: UserAction::AddNewDepartment,
        },
        input::MenuItem {
            label: "Add employee to department",
            option: UserAction::AddEmployeeToDepartment,
        },
        input::MenuItem {
            label: "List all departments",
            option: UserAction::ListDepartments,
        },
        input::MenuItem {
            label: "List all employees in a department",
            option: UserAction::ListDepartmentEmployees,
        },
        input::MenuItem {
            label: "List all employees in company by department",
            option: UserAction::ListCompanyEmployeesByDepartment,
        },
        input::MenuItem {
            label: "Quit",
            option: UserAction::Quit,
        },
    ];

    &ACTION_MENU
}

fn main() {
    let mut company: Company = BTreeMap::new();
    let menu = get_action_menu();
    let menu_iter = menu.iter();

    loop {
        match input::read_menu_option::<UserAction>(menu_iter.clone(), "\nSelect an action:") {
            UserAction::AddNewDepartment => add_new_department(&mut company),
            UserAction::AddEmployeeToDepartment => add_employee_to_department(&mut company),
            UserAction::ListDepartments => list_departments(&company),
            UserAction::ListDepartmentEmployees => list_selected_department_employees(&company),
            UserAction::ListCompanyEmployeesByDepartment => {
                list_company_employees_by_department(&company)
            }
            UserAction::Quit => break,
        }
    }
}
