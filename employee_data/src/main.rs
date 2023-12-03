use std::collections::HashMap;

fn main() {
    let mut sales = Vec::new();

    sales.push("amir");
    sales.push("philips");
    sales.push("jason");

    sales.sort_by(|a, b| a.cmp(b));

    let mut sales_data = HashMap::new();

    for employee in &sales{
        sales_data.entry(employee).or_insert(0);
    }

    println!("{:?}", sales_data);
}
