#[macro_export]
macro_rules! define_bitflags {
    (
        pub bitflag $name_struct:ident: $type:ty {
            $(
                $flag:ident = $bit:expr
            ),* $(,)+
        }
    ) => {
        ::bitflags::bitflags! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name_struct: $type {
                $(
                    const $flag = $bit;
                )*
            }
        }
    }
}
