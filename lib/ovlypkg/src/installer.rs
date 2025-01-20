use devhelper::under_construction;

/// Fake install is the function that is used to install the package
///
/// # Arguments
/// - `target`: target ovpkg to install
///
/// # Returns
/// - Always returns 0
pub fn fake_install(target: &str) -> usize {
    under_construction::message();
    println!("Fake install");
    println!("{}", target);

    return 0;
}


/// install_package_to_dht is the function that performs a production install to a running DHT network on CoreOverlay.
///
/// # Arguments
/// - `target`: target ovpkg to install
///
/// # Returns
/// - Always returns 0
pub fn install_package_to_dht(target: &str) -> usize {
    under_construction::message();
    println!("Install package to DHT");
    println!("{}", target);

    return 0;
}
