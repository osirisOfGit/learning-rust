/*
    1. Define a 3 way translation system, where (A <==> B <==> C)
    2. Be able to translate from the String Display Value or Enum variant

    Using Strum for Enum Manipulation - https://docs.rs/strum/latest
*/

use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

trait CoreTranslation: IntoEnumIterator + PartialEq + Display {
    fn translate_to_display(&self) -> String;

    fn translate_from_display(display_val: &String) -> Option<Self> {
        Self::iter()
            .filter(|val| val.translate_to_display().eq(display_val))
            .last()
    }
}

#[derive(EnumIter, PartialEq, Debug, Display)]
#[strum(prefix = "CommonSystem:")]
enum CommonSystem {
    First,
    Second,
    Third,
}

impl CoreTranslation for CommonSystem {
    fn translate_to_display(&self) -> String {
        match self {
            Self::First => String::from("commonFirst"),
            Self::Second => String::from("commonSecond"),
            Self::Third => String::from("commonThird"),
        }
    }
}

// Technically, this could just use CommonSystem instead of T,
// but to take it a step further i'm just designing this system to work
// with any kind of Common data model, based on a real-world (large enterprise)
// scenario i had to solve in Java
trait BiDirectionalCommonTranslation<T: CoreTranslation>: CoreTranslation {
    fn translate_to_common_enum(&self) -> T;

    fn translate_to_common_display(&self) -> String {
        self.translate_to_common_enum().translate_to_display()
    }

    fn translate_from_common_enum(common_enum: &T) -> Option<Self> {
        Self::iter()
            .filter(|system_enum| system_enum.translate_to_common_enum().eq(common_enum))
            .last()
    }

    fn translate_from_common_enum_to_display(common_enum: &T) -> Option<String> {
        Self::translate_from_common_enum(&common_enum).map(|sys_e| sys_e.translate_to_display())
    }
}

#[derive(EnumIter, PartialEq, Debug, Display)]
#[strum(prefix = "LegacySystem:")]
enum LegacySystem {
    LegacyFirst,
    LegacySecond,
    LegacyThird,
}

impl CoreTranslation for LegacySystem {
    fn translate_to_display(&self) -> String {
        match self {
            Self::LegacyFirst => String::from("legacyFirst"),
            Self::LegacySecond => String::from("legacySecond"),
            Self::LegacyThird => String::from("legacyThird"),
        }
    }
}

impl BiDirectionalCommonTranslation<CommonSystem> for LegacySystem {
    fn translate_to_common_enum(&self) -> CommonSystem {
        match self {
            Self::LegacyFirst => CommonSystem::First,
            Self::LegacySecond => CommonSystem::Second,
            Self::LegacyThird => CommonSystem::Third,
        }
    }
}

#[derive(EnumIter, PartialEq, Debug, Display)]
#[strum(prefix = "ModernSystem:")]
enum ModernSystem {
    ModernFirst,
    ModernSecond,
    ModernThird,
}

impl CoreTranslation for ModernSystem {
    fn translate_to_display(&self) -> String {
        match self {
            Self::ModernFirst => String::from("modernFirst"),
            Self::ModernSecond => String::from("modernSecond"),
            Self::ModernThird => String::from("modernThird"),
        }
    }
}

impl BiDirectionalCommonTranslation<CommonSystem> for ModernSystem {
    fn translate_to_common_enum(&self) -> CommonSystem {
        match self {
            Self::ModernFirst => CommonSystem::First,
            Self::ModernSecond => CommonSystem::Second,
            Self::ModernThird => CommonSystem::Third,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_them_all() {
        // Repeat for CommonSystem
        for modern in ModernSystem::iter() {
            let (expected_display, expected_common) = match modern {
                ModernSystem::ModernFirst => ("modernFirst", CommonSystem::First),
                ModernSystem::ModernSecond => ("modernSecond", CommonSystem::Second),
                ModernSystem::ModernThird => ("modernThird", CommonSystem::Third),
            };

            let display_value = String::from(expected_display);

            assert_eq!(
                display_value,
                modern.translate_to_display(),
                "{modern} did not translate into {display_value}"
            );
            assert_eq!(
                expected_common,
                modern.translate_to_common_enum(),
                "{modern} did not translate to {expected_common}"
            );
            assert_eq!(
                expected_common.translate_to_display(),
                modern.translate_to_common_display(),
                "{modern} did not translate into {expected_common}"
            );

            let modern = Some(modern);
            assert_eq!(
                modern,
                ModernSystem::translate_from_common_enum(&expected_common)
            );

            assert_eq!(modern, ModernSystem::translate_from_display(&display_value));

            assert_eq!(
                Some(display_value),
                ModernSystem::translate_from_common_enum_to_display(&expected_common)
            );
        }
    }

    #[test]
    fn translate_e2e() {
        assert_eq!(
            Some(ModernSystem::ModernThird),
            ModernSystem::translate_from_common_enum(
                &LegacySystem::LegacyThird.translate_to_common_enum()
            )
        )
    }
}
