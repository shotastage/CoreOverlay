use std::env;

pub fn platform() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "windows"
    }
    #[cfg(target_os = "linux")]
    {
        "linux"
    }
    #[cfg(target_os = "macos")]
    {
        "macos"
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        "other"
    }
}

pub fn arch() -> &'static str {
    env::consts::ARCH
}

pub fn os_info() -> &'static str {
    env::consts::OS
}

pub fn ovr_user_agent() -> &'static str {
    "OVERLAY/0.1.0 COREOVERLAY/0.1.0 DHT/Kademlia WASMER/3.1.0"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_ovr_arch() {
        let arch_str = arch();

        println!("arch_str: {}", arch_str);
        assert_eq!(env::consts::ARCH, arch_str);
    }

    #[test]
    fn test_ovr_os() {
        let os_str = os_info();
        println!("os_str: {}", os_str);
        assert_eq!(env::consts::OS, os_str);
    }

    #[test]
    fn test_ovr_user_agent() {
        let user_agent_str = ovr_user_agent();
        println!("user_agent_str: {}", user_agent_str);
        assert_eq!(
            "OVERLAY/0.1.0 COREOVERLAY/0.1.0 DHT/Kademlia WASMER/3.1.0",
            user_agent_str
        );
    }
}
