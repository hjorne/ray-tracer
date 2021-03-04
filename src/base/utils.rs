#[macro_export]
macro_rules! impl_eq {
    ($t:ty; $($field:ident),*) => {
        impl PartialEq for $t {
            fn eq(&self, rhs: &Self) -> bool {
                $(self.$field.xeq(rhs.$field) &&)* true
            }
        }

        impl Eq for $t {}
    };
}
