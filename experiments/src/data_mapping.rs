/*
    1. Define a 3 way translation system, where values from System B and C
        can translate to/from System A as long as there is a mapping for it
    2.
*/

// You need to bring the trait into scope to use it!
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

trait CoreTranslation {
    fn translate_to_display(&self) -> String;
}

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
trait BiDirectionalCommonTranslation<T: CoreTranslation> {
    fn translate_to_common_enum(&self) -> T;

    fn translate_to_common_display(&self) -> String {
        self.translate_to_common_enum().translate_to_display()
    }

    fn translate_from_common_enum<NC>(common_enum: T, enum_to_translate_to: NC) -> NC
    where
        NC: BiDirectionalCommonTranslation<T> + CoreTranslation,
    {
		NC::
    }

    fn translate_from_common_enum_to_display<NC>(common_enum: T, enum_to_translate_to: NC) -> String
    where
        NC: BiDirectionalCommonTranslation<T> + CoreTranslation,
    {
        Self::translate_from_common_enum(common_enum, enum_to_translate_to)
            .translate_to_common_display()
    }
}

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

#[cfg(tests)]
mod test {
    #[test]
    fn translate_from_non_common_display_to_common_enum() {}
}
