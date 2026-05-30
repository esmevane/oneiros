use std::collections::{HashMap, HashSet};

use crate::*;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AliasError {
    #[error("alias {name:?} failed to parse: {source}")]
    Parse {
        name: String,
        #[source]
        source: LensParseError,
    },
    #[error("alias cycle detected: {chain}")]
    Cycle { chain: String },
}

/// Pre-parses each alias source string at construction time, then expands
/// matching bare symbols during a walk over a parsed lens. The grammar
/// stays fixed — aliases are pure sugar.
pub(crate) struct AliasResolver {
    parsed: HashMap<String, Lens>,
}

impl AliasResolver {
    pub(crate) fn new(aliases: &HashMap<String, String>) -> Result<Self, AliasError> {
        let mut parsed = HashMap::with_capacity(aliases.len());
        for (name, source) in aliases {
            let lens = Lens::parse(source).map_err(|source| AliasError::Parse {
                name: name.clone(),
                source,
            })?;
            parsed.insert(name.clone(), lens);
        }
        Ok(Self { parsed })
    }

    pub(crate) fn expand(&self, lens: Lens) -> Result<Lens, AliasError> {
        let mut visiting: HashSet<String> = HashSet::new();
        self.walk(lens, &mut visiting)
    }

    fn walk(&self, lens: Lens, visiting: &mut HashSet<String>) -> Result<Lens, AliasError> {
        match lens {
            Lens::Symbol(identifier) => {
                if let Some(body) = self.parsed.get(identifier.as_str()) {
                    if !visiting.insert(identifier.as_str().to_string()) {
                        let chain = visiting
                            .iter()
                            .cloned()
                            .chain(std::iter::once(identifier.as_str().to_string()))
                            .collect::<Vec<_>>()
                            .join(" → ");
                        return Err(AliasError::Cycle { chain });
                    }
                    let expanded = self.walk(body.clone(), visiting)?;
                    visiting.remove(identifier.as_str());
                    Ok(expanded)
                } else {
                    Ok(Lens::Symbol(identifier))
                }
            }
            Lens::Predicate(predicate) => {
                let mut args = Vec::with_capacity(predicate.args.len());
                for arg in predicate.args {
                    args.push(self.walk(arg, visiting)?);
                }
                Ok(Lens::predicate(predicate.name, args))
            }
            Lens::Union(left, right) => Ok(Lens::union(
                self.walk(*left, visiting)?,
                self.walk(*right, visiting)?,
            )),
            Lens::Intersection(left, right) => Ok(Lens::intersection(
                self.walk(*left, visiting)?,
                self.walk(*right, visiting)?,
            )),
            Lens::Difference(left, right) => Ok(Lens::difference(
                self.walk(*left, visiting)?,
                self.walk(*right, visiting)?,
            )),
            literal @ (Lens::Ref(_) | Lens::String(_) | Lens::Integer(_)) => Ok(literal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aliases(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn unknown_symbol_passes_through_unchanged() {
        let resolver = AliasResolver::new(&aliases(&[])).unwrap();
        let parsed = Lens::parse("texture(observation)").unwrap();
        let expanded = resolver.expand(parsed.clone()).unwrap();
        assert_eq!(expanded.to_string(), parsed.to_string());
    }

    #[test]
    fn matching_symbol_expands_to_alias_body() {
        let resolver = AliasResolver::new(&aliases(&[("mine", "agent(gov.process)")])).unwrap();
        let parsed = Lens::parse("mine").unwrap();
        let expanded = resolver.expand(parsed).unwrap();
        assert_eq!(expanded.to_string(), "agent(gov.process)");
    }

    #[test]
    fn alias_expands_inside_predicate_arg() {
        let resolver = AliasResolver::new(&aliases(&[("mine", "agent(gov.process)")])).unwrap();
        let parsed = Lens::parse("from(mine)").unwrap();
        let expanded = resolver.expand(parsed).unwrap();
        assert_eq!(expanded.to_string(), "from(agent(gov.process))");
    }

    #[test]
    fn nested_aliases_expand_transitively() {
        let resolver = AliasResolver::new(&aliases(&[
            ("a", "agent(gov.process)"),
            ("b", "a & texture(observation)"),
        ]))
        .unwrap();
        let parsed = Lens::parse("b").unwrap();
        let expanded = resolver.expand(parsed).unwrap();
        assert_eq!(
            expanded.to_string(),
            "(agent(gov.process) & texture(observation))"
        );
    }

    #[test]
    fn circular_alias_rejects() {
        let resolver = AliasResolver::new(&aliases(&[("a", "b"), ("b", "a")])).unwrap();
        let parsed = Lens::parse("a").unwrap();
        let err = resolver.expand(parsed).expect_err("cycle must reject");
        assert!(matches!(err, AliasError::Cycle { .. }));
    }
}
