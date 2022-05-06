use query_shit::*;

fn main() {
    let (f, ref vars) = to_formula(vec![Query::ReadComponent("A"), Query::WriteComponent("A")]);
    println!(
        "{}",
        f.display(vars)
    );
    if let Some(assignment) = f.falsify(vars) {
        println!("{assignment:?}");
    }
}
