# CoreOverlay Package Manager

>> Now under construction...


## Usage for Packaging

```rust

// Example usage
fn main() -> io::Result<()> {
    // Create sample metadata
    let mut dependencies = HashMap::new();
    dependencies.insert("dep1".to_string(), "1.0.0".to_string());

    let module_metadata = WasmMetadata {
        name: "example-module".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Example WASM module".to_string()),
        dependencies,
    };

    let package_metadata = WasmMetadata {
        name: "example-package".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Example WASM package".to_string()),
        dependencies: HashMap::new(),
    };

    // Package WASM files
    let wasm_files = vec![(Path::new("example.wasm"), module_metadata)];

    let package_data = package_wasm_files(wasm_files, package_metadata)?;

    // Save package to file
    fs::write("package.wasm", &package_data)?;

    // Later, unarchive the package
    let package_data = fs::read("package.wasm")?;
    let (package, modules) = unarchive_wasm_package(&package_data)?;

    // Process extracted modules
    for (name, content) in modules {
        println!("Extracted module: {} ({} bytes)", name, content.len());
    }

    Ok(())
}
```
