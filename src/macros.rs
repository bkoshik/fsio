#[macro_export]
macro_rules! define_bitflags {
    (
        $(#[$outer:meta])*
        let prefix = $Prefix:ident;
        pub struct $BitFlags:ident: $Type:ty {
            $($Flag:ident),* $(,)+
        }
    ) => {
        ::bitflags::bitflags! {
            #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[repr(transparent)]
            $(#[$outer])*
            pub struct $BitFlags: $Type {
                $(
                    const $Flag = $Prefix::$Flag as $Type;
                )+
            }
        }
    }
}