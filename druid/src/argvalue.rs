////  Derived from https://github.com/projectfluent/fluent-rs/blob/master/fluent-bundle/src/types.rs
#[derive(Debug, PartialEq, Clone)]
pub enum ArgValue<'source> {
    String(Cow<'source, str>),
    Number(Cow<'source, str>),
    Error(DisplayableNode<'source>),
    None,
}

impl<'source> ArgValue<'source> {
    pub fn into_number<S: ToString>(v: S) -> Self {
        let s = v.to_string();
        match f64::from_str(&s) {
            Ok(_) => ArgValue::Number(s.into()),
            Err(_) => ArgValue::String(s.into()),
        }
    }

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

    pub fn to_string(&self) -> Cow<'source, str> {
        match self {
            ArgValue::String(s) => s.clone(),
            ArgValue::Number(n) => n.clone(),
            ArgValue::Error(d) => format!("{{{}}}", d.to_string()).into(),
            ArgValue::None => "???".into(),
        }
    }
}

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

impl<'source> From<String> for ArgValue<'source> {
    fn from(s: String) -> Self {
        ArgValue::String(s.into())
    }
}

impl<'source> From<&'source str> for ArgValue<'source> {
    fn from(s: &'source str) -> Self {
        ArgValue::String(s.into())
    }
}

macro_rules! from_num {
    ($num:ty) => {
        impl<'source> From<$num> for ArgValue<'source> {
            fn from(n: $num) -> Self {
                ArgValue::Number(n.to_string().into())
            }
        }
        impl<'source> From<&'source $num> for ArgValue<'source> {
            fn from(n: &'source $num) -> Self {
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