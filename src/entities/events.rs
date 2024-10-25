#[derive(Debug, sqlx::Type, Copy, Clone)]
pub enum Source {
    Shopify,
    Core,
    Prestashop,
}

#[derive(Debug, sqlx::Type, Copy, Clone)]
pub enum Command {
    Create,
    Update,
    Upsert,
}
