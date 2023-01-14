fn main() {
    // tuples
    let tup = ("Kabilan", 22, 64.2);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // struct
    // 1) classic struct(C like)
    struct Employee {name: String, age: u16}
    let employee_1 = Employee {
        name: String::from("Kabilan"),
        age: 31
    };
    println!("{}, {}",employee_1.name, employee_1.age);
}