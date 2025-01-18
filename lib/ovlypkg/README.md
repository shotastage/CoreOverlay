# CoreOverlay Package Manager

>> Now under construction...

## Package Formats

`*.ovpkg` is packaged WASM binaries with metadata.

`*.ovmeta` is meta information file exported from `*.ovpkg`.

`*.owsm` is extracted WASM binary from `*.ovpkg`.

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

    // Package multiple WASM files
    let module_metadata1 = WasmMetadata {
        name: "module1".to_string(),
        version: "1.0.0".to_string(),
        description: Some("First WASM module".to_string()),
        dependencies: dependencies.clone(),
    };

    let module_metadata2 = WasmMetadata {
        name: "module2".to_string(),
        version: "1.0.0".to_string(),
        description: Some("Second WASM module".to_string()),
        dependencies: dependencies,
    };

    let wasm_files = vec![
        (Path::new("module1.wasm"), module_metadata1),
        (Path::new("module2.wasm"), module_metadata2),
    ];

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
