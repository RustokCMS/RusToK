use crate::models::tenants;

#[derive(Clone, Debug)]
pub struct TenantContext {
    pub tenant: tenants::Model,
}

impl TenantContext {
    pub fn new(tenant: tenants::Model) -> Self {
        Self { tenant }
    }
}

pub trait TenantContextExt {
    fn tenant_context(&self) -> Option<&TenantContext>;
}

impl TenantContextExt for http::request::Parts {
    fn tenant_context(&self) -> Option<&TenantContext> {
        self.extensions.get::<TenantContext>()
    }
}
