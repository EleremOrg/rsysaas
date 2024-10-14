struct Shopify {}
struct Regular {}

struct Customer<T> {
    name: String,
    email: String,
    password: String,
    meta: Option<T>,
}
