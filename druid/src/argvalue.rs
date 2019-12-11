////  Derived from https://github.com/projectfluent/fluent-rs/blob/master/fluent-bundle/src/types.rs

use core::fmt::Write;

/// Max number of arg values
type MaxArgValues = heapless::consts::U2;
/// Hash map of arg names to arg values
pub type ArgValues = heapless::FnvIndexMap<&'static str, ArgValue, MaxArgValues>;

type MaxLocalizedString = heapless::consts::U20; //// Max length of localized strings
type String = heapless::String::<MaxLocalizedString>; ////

#[derive(Debug, PartialEq, Clone)]
pub enum ArgValue {
    String(String),
    U32(u32),
    Error(String),
    None,
}

impl ArgValue {
    /*
        pub fn into_number(v: String) -> Self {
            let s = v.to_string();
            match f64::from_str(&s) {
                Ok(_) => ArgValue::Number(s.into()),
                Err(_) => ArgValue::String(s.into()),
            }
        }
    */

    /*
        pub fn matches<R: Borrow<FluentResource>>(
            &self,
            other: &ArgValue,
            scope: &Scope<R>,
        ) -> bool {
            match (self, other) {
                (&ArgValue::String(ref a), &ArgValue::String(ref b)) => a == b,
                (&ArgValue::Number(ref a), &ArgValue::Number(ref b)) => a == b,
                (&ArgValue::String(ref a), &ArgValue::Number(ref b)) => {
                    let cat = match a.as_ref() {
                        "zero" => PluralCategory::ZERO,
                        "one" => PluralCategory::ONE,
                        "two" => PluralCategory::TWO,
                        "few" => PluralCategory::FEW,
                        "many" => PluralCategory::MANY,
                        "other" => PluralCategory::OTHER,
                        _ => return false,
                    };
                    let pr = &scope.bundle.plural_rules;
                    pr.select(b.as_ref()) == Ok(cat)
                }
                _ => false,
            }
        }
    */

    pub fn to_string(&self) -> String {
        match self {
            ArgValue::String(s) => s.clone(),
            ArgValue::U32(v) => {
                let mut buffer = String::new();
                write!(&mut buffer, "{}", v)
                    .expect("arg fail");
                buffer
            }
            ArgValue::Error(s) => "Error".into(),
            ArgValue::None => "???".into(),
        }
    }
}

/*
    impl<'source> fmt::Display for ArgValue<'source> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ArgValue::String(s) => f.write_str(s),
                ArgValue::Number(n) => f.write_str(n),
                ArgValue::Error(d) => write!(f, "{{{}}}", d),
                ArgValue::None => f.write_str("???"),
            }
        }
    }
*/

impl From<String> for ArgValue {
    fn from(s: String) -> Self {
        ArgValue::String(s.into())
    }
}

impl From<&str> for ArgValue {
    fn from(s: &str) -> Self {
        ArgValue::String(s.into())
    }
}

impl From<u32> for ArgValue {
    fn from(v: u32) -> Self {
        ArgValue::U32(v)
    }
}

/*
    macro_rules! from_num {
        ($num:ty) => {
            impl From<$num> for ArgValue {
                fn from(n: $num) -> Self {
                    ArgValue::Number(n.to_string().into())
                }
            }
            impl From<& $num> for ArgValue {
                fn from(n: & $num) -> Self {
                    ArgValue::Number(n.to_string().into())
                }
            }
        };
    }
    from_num!(i8);
    from_num!(i16);
    from_num!(i32);
    from_num!(i64);
    from_num!(i128);
    from_num!(isize);
    from_num!(u8);
    from_num!(u16);
    from_num!(u32);
    from_num!(u64);
    from_num!(u128);
    from_num!(usize);
    from_num!(f32);
    from_num!(f64);
*/