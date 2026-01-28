use rustok_blog::BlogModule;
use rustok_commerce::CommerceModule;
use rustok_core::ModuleRegistry;

pub fn build_registry() -> ModuleRegistry {
    ModuleRegistry::new()
        .register(CommerceModule)
        .register(BlogModule)
}
