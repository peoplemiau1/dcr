pub fn setup(_args: &[String]) -> i32 {
    println!("Setting up DCR registries...");
    match crate::core::registry::RegistryManager::load() {
        Ok(manager) => {
            println!("Loaded {} registries.", manager.config.registry.len());
            for (name, reg) in manager.config.registry {
                println!("- {}: {} (priority {})", name, reg.url, reg.priority);
            }
            0
        }
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    }
}
