macro_rules! define_bitflags {
    (
        $(#[$outer:meta])*
        let prefix = $prefix:ident;
        pub struct $BitFlags:ident: $Type:ty {
            $(
                $(#[$inner:meta])*
                $Flag:ident $(as $cast:ty)*
            )+
        }
    ) => {
        ::bitflags::bitflags! {
            #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[repr(transparent)]
            $(#[$outer])*
            pub struct $BitFlags: $T {
                $(
                    $(#[$inner])*
                    const $Flag = $prefix::$Flag $(as $cast)*;
                )+
            }
        }
    }
}