#![allow(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

// Note: constants module not needed for this example
use std::process::Command;

fn main() {
    println!("Testing string utility functions:");

    // Test kebab-case conversion
    let test_code = r#"
        use nestgate_core::utils::string;

        fn main() {
            let result = string::to_kebab_case("CamelCase");
            println!("to_kebab_case(\"CamelCase\") = \"{}\"", result);

            let snake_result = string::to_snake_case("CamelCase");
            println!("to_snake_case(\"CamelCase\") = \"{}\"", snake_result);

            let camel_result = string::to_camel_case("snake_case");
            println!("to_camel_case(\"snake_case\") = \"{}\"", camel_result);
        }
    "#;

    println!("Expected outputs:");
    println!("to_kebab_case(\"CamelCase\") should return \"camel-case\"");
    println!("to_snake_case(\"CamelCase\") should return \"camel_case\"");
    println!("to_camel_case(\"snake_case\") should return \"snakeCase\"");

    std::fs::write("temp_test.rs", test_code).unwrap();

    let output = Command::new("cargo")
        .args(["run", "--bin", "temp_test"])
        .output()
        .expect("Failed to run test");

    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    println!("Error: {}", String::from_utf8_lossy(&output.stderr));

    std::fs::remove_file("temp_test.rs").ok();
}
