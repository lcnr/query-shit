use query_shit::*;

fn test_queries(queries: Vec<Query>) {
    println!("queries: {:?}", queries);
    let (f, ref vars) = to_formula(queries);
    println!("formula: {}", f.display(vars));
    if let Some(assignment) = f.compute_assignment(vars) {
        println!("valid: {assignment:?}");
    } else {
        println!("not valid!");
    }
}

fn main() {
    test_queries(vec![Query::ReadComponent("T"), Query::ReadComponent("T")]);
    test_queries(vec![Query::ReadComponent("T"), Query::WriteComponent("T")]);
    test_queries(vec![
        Query::All(vec![
            Query::Any(vec![Query::ReadComponent("A"), Query::ReadComponent("C")]),
            Query::WriteComponent("B"),
        ]),
        Query::All(vec![Query::WriteComponent("B"), Query::Without("A")]),
    ]);
    test_queries(vec![Query::WriteComponent("T"), Query::ReadComponent("T")]);
}
