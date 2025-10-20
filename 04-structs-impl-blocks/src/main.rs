// Next rung 🪜 coming up:

// on 4 — Structs + Impl Blocks (BLessuilding Your Own Data Types)

// Structs are the blueprint for creating custom data types (like Go structs or JS objects).

// Let’s go step-by-step:

// 🧩 Example 1: Basic Struct


// struct Startup {
//     name: String,
//     category: String,
//     monthly_revenue: f64,
// }

// fn main() {
//     let summafy = Startup {
//         name: String::from("Summafy.io"),
//         category: String::from("AI & Productivity"),
//         monthly_revenue: 1200.50,
//     };

//     println!(
//         "{} is in {} category and earns ${} monthly",
//         summafy.name, summafy.category, summafy.monthly_revenue
//     );
// }
// // 🧠 Concept:

// // struct defines the shape of your data

// // You use String::from() for owned strings

// // Fields are accessed with dot notation (summafy.name)



// 🧩 Example 2: Add Behavior (Impl Block)

// Now let’s give our struct some methods, like in OOP:

// 🎯 Your Challenge:

// Add a new field called employees: u32

// Add a method avg_revenue_per_employee() → divide annual revenue by employees

// Print it in the description


struct Startup {
    name: String,
    category: String,
    monthly_revenue: f64,
    employees: u32
}

impl Startup {
    fn annual_revenue(&self) -> f64 {
        self.monthly_revenue * 12.0
    }

    fn describe(&self) {
        println!(
            "{} operates in {} and makes ${} per year.We have {} empolyees each with an average revenue of ${} per year.",
            self.name,
            self.category,
            self.annual_revenue(),
            self.employees,
            self.avg_revenue_per_employee()
        );
    }

    fn avg_revenue_per_employee(&self) -> f64 {
        self.annual_revenue() / (self.employees as f64)
    }
}

fn main() {
    let summafy = Startup {
        name: String::from("Summafy.io"),
        category: String::from("AI & Productivity"),
        monthly_revenue: 1200.50,
        employees: 10,
    };

    summafy.describe();
}


