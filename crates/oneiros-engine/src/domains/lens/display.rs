use crate::*;

impl core::fmt::Display for Lens {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Lens::Symbol(identifier) => identifier.fmt(f),
            Lens::String(literal) => write!(f, "\"{}\"", escape_string(literal.as_str())),
            Lens::Ref(reference) => write!(f, "ref:{reference}"),
            Lens::Integer(literal) => literal.fmt(f),
            Lens::Predicate(predicate) => predicate.fmt(f),
            Lens::Union(left, right) => write!(f, "({left} | {right})"),
            Lens::Intersection(left, right) => write!(f, "({left} & {right})"),
            Lens::Difference(left, right) => write!(f, "({left} ~ {right})"),
        }
    }
}

impl core::fmt::Display for Predicate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}(", self.name)?;
        for (index, arg) in self.args.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            arg.fmt(f)?;
        }
        write!(f, ")")
    }
}

fn escape_string(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for character in raw.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            other => out.push(other),
        }
    }
    out
}
