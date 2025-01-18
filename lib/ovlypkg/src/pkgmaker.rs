use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WasmMetadata {
    name: String,
    version: String,
    description: Option<String>,
    dependencies: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct WasmModule {
    metadata: WasmMetadata,
    content: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct WasmPackage {
    modules: Vec<WasmModule>,
    package_metadata: WasmMetadata,
}

pub fn package_wasm_files(
    wasm_files: Vec<(&Path, WasmMetadata)>,
    package_metadata: WasmMetadata,
) -> io::Result<Vec<u8>> {
    let mut modules = Vec::new();

    // Load and package each WASM file
    for (path, metadata) in wasm_files {
        let mut content = Vec::new();
        fs::File::open(path)?.read_to_end(&mut content)?;

        modules.push(WasmModule {
            metadata,
            content,
        });
    }

    let package = WasmPackage {
        modules,
        package_metadata,
    };

    // Serialize the package to bytes
    bincode::serialize(&package)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}

pub fn unarchive_wasm_package(package_data: &[u8]) -> io::Result<(WasmPackage, Vec<(String, Vec<u8>)>)> {
    // Deserialize the package
    let package: WasmPackage = bincode::deserialize(package_data)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Extract WASM modules
    let modules: Vec<(String, Vec<u8>)> = package.modules
        .iter()
        .map(|module| (module.metadata.name.clone(), module.content.clone()))
        .collect();

    Ok((package, modules))
}

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
    let wasm_files = vec![
        (Path::new("example.wasm"), module_metadata),
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

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_package_and_unarchive() -> io::Result<()> {
        // Create a temporary WASM file
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(b"mock wasm content")?;

        // Create test metadata
        let mut dependencies = HashMap::new();
        dependencies.insert("test-dep".to_string(), "1.0.0".to_string());

        let module_metadata = WasmMetadata {
            name: "test-module".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test module".to_string()),
            dependencies,
        };

        let package_metadata = WasmMetadata {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test package".to_string()),
            dependencies: HashMap::new(),
        };

        // Package the test file
        let wasm_files = vec![
            (temp_file.path(), module_metadata),
        ];

        let package_data = package_wasm_files(wasm_files, package_metadata)?;

        // Unarchive and verify
        let (package, modules) = unarchive_wasm_package(&package_data)?;

        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].0, "test-module");
        assert_eq!(modules[0].1, b"mock wasm content");
        assert_eq!(package.package_metadata.name, "test-package");

        Ok(())
    }
}
