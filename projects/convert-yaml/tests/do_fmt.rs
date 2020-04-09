use arc_from_yaml::file_to_arc;

fn test_file(name: &str) {
    let input = format!("tests/{}.yaml", name);
    let output = format!("tests/out/{}.arc", name);
    file_to_arc(&input, &output).unwrap_or_default()
}

#[test]
fn test_easy() {
    test_file("easy_1");
    test_file("easy_2");
    test_file("easy_3");
    test_file("easy_4");
    test_file("easy_5");
}

#[test]
fn test_normal() {
    // test_file("normal_1");
    test_file("normal_2");
    test_file("normal_3");
    test_file("normal_4");
    test_file("normal_5");
}

#[test]
fn test_hard() {
    test_file("hard_1");
}
