enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicutly `use` each name so they area available without manual scoping.
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`
    use crate::Work::*;

    // Same as `Status::Poor`
    let status = Poor;
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
