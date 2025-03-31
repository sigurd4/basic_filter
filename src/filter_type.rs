use serde::{Deserialize, Serialize};

enum_try_from::impl_enum_try_from!(
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Default)]
    pub enum FilterType
    {
        #[default]
        LowPass = 0,
        Peak = 1,
        HighPass = 2,
    },
    u8,
    (),
    ()
);