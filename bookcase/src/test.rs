#[macro_export]
macro_rules! line_str {
    () => {
        &format!("expected on line: {}", line!())
    };
}
