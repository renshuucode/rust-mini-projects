use rust_mini_projects::todo::Todo;

#[test]
fn test_add_task() {
    let mut todo = Todo::new();
    todo.add("Buy milk".to_string());
    assert_eq!(todo.tasks.len(), 1);
    assert_eq!(todo.tasks.get(&1), Some(&"Buy milk".to_string()));
}

#[test]
fn test_list_tasks() {
    let mut todo = Todo::new();
    todo.add("Buy milk".to_string());
    todo.add("Do laundry".to_string());

    // 捕获标准输出
    use std::io::Write;
    let mut output = Vec::new();
    writeln!(&mut output, "1: Buy milk").unwrap();
    writeln!(&mut output, "2: Do laundry").unwrap();

    let expected_output = String::from_utf8(output).unwrap();

    // 重定向标准输出
    let mut buffer = Vec::new();
    todo.list_to_buffer(&mut buffer);
    let actual_output = String::from_utf8(buffer).unwrap();

    assert_eq!(actual_output, expected_output);
}

#[test]
fn test_remove_task() {
    let mut todo = Todo::new();
    todo.add("Buy milk".to_string());
    todo.add("Do laundry".to_string());

    todo.remove(1);
    assert_eq!(todo.tasks.len(), 1);
    assert_eq!(todo.tasks.get(&1), None);
    assert_eq!(todo.tasks.get(&2), Some(&"Do laundry".to_string()));
}