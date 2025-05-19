// Implements From<$error> for $self
#[macro_export]
macro_rules! error_impl_from {
    ($self:ident; $( $ident:ident ),*) => {
        $(
            paste::paste! {
                impl From<[<$ident Error>]> for $self {
                    fn from(error: [<$ident Error>]) -> Self {
                        Self::$ident(error)
                    }
                }
            }
        )*
    };
}
