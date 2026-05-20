use crate::*;

impl Lens {
    pub(crate) fn parse(source: &str) -> Result<Lens, LensParseError> {
        let mut parser = Parser::new(source);
        let lens = parser.parse_union()?;
        parser.expect_eof()?;
        Ok(lens)
    }
}

struct Parser<'src> {
    source: &'src str,
    cursor: usize,
}

impl<'src> Parser<'src> {
    fn new(source: &'src str) -> Self {
        Self { source, cursor: 0 }
    }

    fn parse_union(&mut self) -> Result<Lens, LensParseError> {
        let mut left = self.parse_intersection()?;
        loop {
            self.skip_whitespace();
            match self.peek_char() {
                Some('|') => {
                    self.advance(1);
                    let right = self.parse_intersection()?;
                    left = Lens::union(left, right);
                }
                Some('~') => {
                    self.advance(1);
                    let right = self.parse_intersection()?;
                    left = Lens::difference(left, right);
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_intersection(&mut self) -> Result<Lens, LensParseError> {
        let mut left = self.parse_primary()?;
        loop {
            self.skip_whitespace();
            match self.peek_char() {
                Some('&') => {
                    self.advance(1);
                    let right = self.parse_primary()?;
                    left = Lens::intersection(left, right);
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Lens, LensParseError> {
        self.skip_whitespace();
        match self.peek_char() {
            Some('(') => {
                self.advance(1);
                let inner = self.parse_union()?;
                self.skip_whitespace();
                self.expect(')', "closing paren")?;
                Ok(inner)
            }
            Some('"') => {
                let literal = self.parse_string_literal()?;
                Ok(Lens::string(StringLiteral::new(literal)))
            }
            Some(character) if character.is_ascii_digit() => {
                let integer = self.parse_integer_literal()?;
                Ok(Lens::integer(integer))
            }
            Some(character) if is_identifier_start(character) => {
                self.parse_identifier_or_predicate()
            }
            Some(found) => Err(LensParseError::UnexpectedChar {
                found,
                at: self.cursor,
                expected: "predicate, symbol, integer, string, ref, or '('",
            }),
            None => Err(LensParseError::UnexpectedEof {
                at: self.cursor,
                expected: "predicate, symbol, integer, string, ref, or '('",
            }),
        }
    }

    fn parse_identifier_or_predicate(&mut self) -> Result<Lens, LensParseError> {
        let text = self.parse_identifier_text()?;
        if text == "ref" && self.peek_char() == Some(':') {
            self.advance(1);
            let body_start = self.cursor;
            let reference = self.parse_ref_body()?;
            let token: crate::RefToken = format!("ref:{reference}").parse().map_err(|_| {
                LensParseError::InvalidRef {
                    at: body_start,
                    reason: "ref body did not decode to a valid token",
                }
            })?;
            return Ok(Lens::reference(token));
        }
        self.skip_whitespace();
        if self.peek_char() == Some('(') {
            self.advance(1);
            let args = self.parse_argument_list()?;
            return Ok(Lens::predicate(PredicateName::new(text), args));
        }
        Ok(Lens::symbol(Identifier::new(text)))
    }

    fn parse_argument_list(&mut self) -> Result<Vec<Lens>, LensParseError> {
        let mut args = Vec::new();
        self.skip_whitespace();
        if self.peek_char() == Some(')') {
            self.advance(1);
            return Ok(args);
        }
        loop {
            let argument = self.parse_union()?;
            args.push(argument);
            self.skip_whitespace();
            match self.peek_char() {
                Some(',') => {
                    self.advance(1);
                    self.skip_whitespace();
                    if self.peek_char() == Some(')') {
                        return Err(LensParseError::MissingArgument { at: self.cursor });
                    }
                }
                Some(')') => {
                    self.advance(1);
                    return Ok(args);
                }
                Some(found) => {
                    return Err(LensParseError::UnexpectedChar {
                        found,
                        at: self.cursor,
                        expected: "',' or ')'",
                    });
                }
                None => {
                    return Err(LensParseError::UnexpectedEof {
                        at: self.cursor,
                        expected: "',' or ')'",
                    });
                }
            }
        }
    }

    fn parse_identifier_text(&mut self) -> Result<String, LensParseError> {
        let start = self.cursor;
        while let Some(character) = self.peek_char() {
            if is_identifier_continue(character) {
                self.advance(character.len_utf8());
            } else {
                break;
            }
        }
        if self.cursor == start {
            return Err(match self.peek_char() {
                Some(found) => LensParseError::UnexpectedChar {
                    found,
                    at: self.cursor,
                    expected: "identifier",
                },
                None => LensParseError::UnexpectedEof {
                    at: self.cursor,
                    expected: "identifier",
                },
            });
        }
        Ok(self.source[start..self.cursor].to_string())
    }

    fn parse_string_literal(&mut self) -> Result<String, LensParseError> {
        let opened_at = self.cursor;
        self.expect('"', "opening quote")?;
        let mut value = String::new();
        loop {
            match self.peek_char() {
                Some('"') => {
                    self.advance(1);
                    return Ok(value);
                }
                Some('\\') => {
                    self.advance(1);
                    match self.peek_char() {
                        Some('"') => {
                            value.push('"');
                            self.advance(1);
                        }
                        Some('\\') => {
                            value.push('\\');
                            self.advance(1);
                        }
                        Some(found) => {
                            return Err(LensParseError::UnexpectedChar {
                                found,
                                at: self.cursor,
                                expected: "escape sequence",
                            });
                        }
                        None => {
                            return Err(LensParseError::UnterminatedString { at: opened_at });
                        }
                    }
                }
                Some(character) => {
                    value.push(character);
                    self.advance(character.len_utf8());
                }
                None => return Err(LensParseError::UnterminatedString { at: opened_at }),
            }
        }
    }

    fn parse_integer_literal(&mut self) -> Result<IntegerLiteral, LensParseError> {
        let start = self.cursor;
        while let Some(character) = self.peek_char() {
            if character.is_ascii_digit() || character == '_' {
                self.advance(character.len_utf8());
            } else {
                break;
            }
        }
        let raw = &self.source[start..self.cursor];
        let cleaned: String = raw.chars().filter(|character| *character != '_').collect();
        cleaned
            .parse::<i64>()
            .map(IntegerLiteral::new)
            .map_err(|_| LensParseError::InvalidInteger {
                at: start,
                raw: raw.to_string(),
            })
    }

    fn parse_ref_body(&mut self) -> Result<String, LensParseError> {
        let start = self.cursor;
        while let Some(character) = self.peek_char() {
            if is_ref_continue(character) {
                self.advance(character.len_utf8());
            } else {
                break;
            }
        }
        if self.cursor == start {
            return Err(LensParseError::InvalidRef {
                at: start,
                reason: "ref: requires at least one base64url character",
            });
        }
        Ok(self.source[start..self.cursor].to_string())
    }

    fn expect_eof(&mut self) -> Result<(), LensParseError> {
        self.skip_whitespace();
        match self.peek_char() {
            None => Ok(()),
            Some(_) => Err(LensParseError::TrailingInput {
                at: self.cursor,
                found: self.source[self.cursor..].to_string(),
            }),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(character) = self.peek_char() {
            if character.is_whitespace() {
                self.advance(character.len_utf8());
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
    }

    fn advance(&mut self, count: usize) {
        self.cursor += count;
    }

    fn expect(&mut self, character: char, expected: &'static str) -> Result<(), LensParseError> {
        match self.peek_char() {
            Some(found) if found == character => {
                self.advance(character.len_utf8());
                Ok(())
            }
            Some(found) => Err(LensParseError::UnexpectedChar {
                found,
                at: self.cursor,
                expected,
            }),
            None => Err(LensParseError::UnexpectedEof {
                at: self.cursor,
                expected,
            }),
        }
    }
}

fn is_identifier_start(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

fn is_identifier_continue(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_' || character == '.' || character == '-'
}

fn is_ref_continue(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_' || character == '-'
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// Build a [`crate::RefToken`] from a base64url body for test fixtures.
    /// The body must decode to a valid [`crate::Ref`].
    fn ref_token(body: &str) -> crate::RefToken {
        format!("ref:{body}")
            .parse()
            .expect("test ref body must decode to a valid token")
    }

    #[test]
    fn parses_bare_symbol_as_lens() {
        let parsed = Lens::parse("governor.process").expect("parses");
        assert_eq!(parsed, Lens::symbol("governor.process"));
    }

    #[test]
    fn parses_bare_ref_as_lens() {
        let parsed = Lens::parse("ref:AAYQAZ46gMbJfZKr0qOpgknFfA").expect("parses");
        assert_eq!(parsed, Lens::reference(ref_token("AAYQAZ46gMbJfZKr0qOpgknFfA")));
    }

    #[test]
    fn parses_predicate_with_symbol_argument() {
        let parsed = Lens::parse("agent(governor.process)").expect("parses");
        assert_eq!(
            parsed,
            Lens::predicate("agent", [Lens::symbol("governor.process")])
        );
    }

    #[test]
    fn parses_predicate_with_string_argument() {
        let parsed = Lens::parse(r#"search("naming as mechanism")"#).expect("parses");
        assert_eq!(
            parsed,
            Lens::predicate("search", [Lens::string("naming as mechanism")])
        );
    }

    #[test]
    fn parses_predicate_with_ref_argument() {
        let parsed = Lens::parse("mentions(ref:AAYQAZ46gMbJfZKr0qOpgknFfA)").expect("parses");
        assert_eq!(
            parsed,
            Lens::predicate("mentions", [Lens::reference(ref_token("AAYQAZ46gMbJfZKr0qOpgknFfA"))])
        );
    }

    #[test]
    fn parses_nested_predicate_as_argument() {
        let parsed = Lens::parse("mentions(level(working))").expect("parses");
        assert_eq!(
            parsed,
            Lens::predicate(
                "mentions",
                [Lens::predicate("level", [Lens::symbol("working")])]
            )
        );
    }

    #[test]
    fn parses_set_expression_as_argument() {
        let parsed =
            Lens::parse("emitted_by(texture(observation) | texture(handoff))").expect("parses");
        assert_eq!(
            parsed,
            Lens::predicate(
                "emitted_by",
                [Lens::union(
                    Lens::predicate("texture", [Lens::symbol("observation")]),
                    Lens::predicate("texture", [Lens::symbol("handoff")]),
                )]
            )
        );
    }

    #[test]
    fn parses_predicate_with_no_arguments() {
        let parsed = Lens::parse("everything()").expect("parses");
        assert_eq!(parsed, Lens::predicate("everything", []));
    }

    #[test]
    fn parses_predicate_with_multiple_arguments() {
        let parsed =
            Lens::parse("between(ref:AAYQAZ46gMbJfZKr0qOpgknFfA, ref:AAYQAZ47MrcTcuGpm-evDKpj7Q)")
                .expect("parses");
        assert_eq!(
            parsed,
            Lens::predicate(
                "between",
                [
                    Lens::reference(ref_token("AAYQAZ46gMbJfZKr0qOpgknFfA")),
                    Lens::reference(ref_token("AAYQAZ47MrcTcuGpm-evDKpj7Q")),
                ]
            )
        );
    }

    #[test]
    fn parses_union_of_bare_symbols() {
        let parsed = Lens::parse("governor.process | wisdom.process").expect("parses");
        assert_eq!(
            parsed,
            Lens::union(
                Lens::symbol("governor.process"),
                Lens::symbol("wisdom.process"),
            )
        );
    }

    #[test]
    fn parses_intersection() {
        let parsed = Lens::parse("texture(observation) & agent(governor.process)").expect("parses");
        assert_eq!(
            parsed,
            Lens::intersection(
                Lens::predicate("texture", [Lens::symbol("observation")]),
                Lens::predicate("agent", [Lens::symbol("governor.process")]),
            )
        );
    }

    #[test]
    fn parses_difference() {
        let parsed = Lens::parse("texture(observation) ~ texture(handoff)").expect("parses");
        assert_eq!(
            parsed,
            Lens::difference(
                Lens::predicate("texture", [Lens::symbol("observation")]),
                Lens::predicate("texture", [Lens::symbol("handoff")]),
            )
        );
    }

    #[test]
    fn intersection_binds_tighter_than_union() {
        let parsed = Lens::parse("a() | b() & c()").expect("parses");
        assert_eq!(
            parsed,
            Lens::union(
                Lens::predicate("a", []),
                Lens::intersection(Lens::predicate("b", []), Lens::predicate("c", [])),
            )
        );
    }

    #[test]
    fn difference_is_left_associative_at_union_level() {
        let parsed = Lens::parse("a() ~ b() ~ c()").expect("parses");
        assert_eq!(
            parsed,
            Lens::difference(
                Lens::difference(Lens::predicate("a", []), Lens::predicate("b", [])),
                Lens::predicate("c", []),
            )
        );
    }

    #[test]
    fn parens_override_precedence() {
        let parsed = Lens::parse("(a() | b()) & c()").expect("parses");
        assert_eq!(
            parsed,
            Lens::intersection(
                Lens::union(Lens::predicate("a", []), Lens::predicate("b", [])),
                Lens::predicate("c", []),
            )
        );
    }

    #[test]
    fn whitespace_is_ignored() {
        let parsed = Lens::parse("  agent(  governor.process  )   ").expect("parses");
        assert_eq!(
            parsed,
            Lens::predicate("agent", [Lens::symbol("governor.process")])
        );
    }

    #[test]
    fn display_round_trips_through_parse() {
        let inputs = [
            "governor.process",
            "ref:AAYQAZ46gMbJfZKr0qOpgknFfA",
            "agent(governor.process)",
            r#"search("naming")"#,
            "mentions(ref:AAYQAZ46gMbJfZKr0qOpgknFfA)",
            "mentions(level(working))",
            "emitted_by(texture(observation) | texture(handoff))",
            "a() | b() & c()",
            "(a() | b()) & c()",
            "a() ~ b() ~ c()",
        ];
        for input in inputs {
            let parsed = Lens::parse(input).expect("parses");
            let rendered = parsed.to_string();
            let reparsed = Lens::parse(&rendered).expect("rendered form parses");
            assert_eq!(parsed, reparsed, "round-trip failed for {input:?}");
        }
    }

    #[test]
    fn error_on_missing_closing_paren_carries_span() {
        let error = Lens::parse("agent(governor.process").expect_err("must fail");
        assert!(matches!(error, LensParseError::UnexpectedEof { .. }));
    }

    #[test]
    fn error_on_unterminated_string_carries_span() {
        let error = Lens::parse(r#"search("oops"#).expect_err("must fail");
        assert!(matches!(error, LensParseError::UnterminatedString { .. }));
    }

    #[test]
    fn error_on_trailing_input_carries_span() {
        let error = Lens::parse("agent(a) garbage").expect_err("must fail");
        assert!(matches!(error, LensParseError::TrailingInput { .. }));
    }

    #[test]
    fn error_on_empty_input() {
        let error = Lens::parse("").expect_err("must fail");
        assert!(matches!(error, LensParseError::UnexpectedEof { .. }));
    }

    #[test]
    fn parses_integer_literal_as_lens() {
        let parsed = Lens::parse("12").expect("parses");
        assert_eq!(parsed, Lens::integer(12_i64));
    }

    #[test]
    fn parses_integer_with_underscore_separators() {
        let parsed = Lens::parse("1_000_000").expect("parses");
        assert_eq!(parsed, Lens::integer(1_000_000_i64));
    }

    #[test]
    fn parses_integer_as_predicate_argument() {
        let parsed = Lens::parse("latest(everything(), 25)").expect("parses");
        assert_eq!(
            parsed,
            Lens::predicate(
                "latest",
                [Lens::predicate("everything", []), Lens::integer(25_i64)]
            )
        );
    }

    #[test]
    fn integer_round_trips_through_display() {
        let inputs = ["0", "12", "1000", "9223372036854775807"];
        for input in inputs {
            let parsed = Lens::parse(input).expect("parses");
            let rendered = parsed.to_string();
            let reparsed = Lens::parse(&rendered).expect("rendered parses");
            assert_eq!(parsed, reparsed, "round-trip failed for {input:?}");
        }
    }

    #[test]
    fn error_on_integer_overflow_carries_span() {
        let error = Lens::parse("99999999999999999999999").expect_err("must fail");
        assert!(matches!(error, LensParseError::InvalidInteger { .. }));
    }

    #[test]
    fn error_on_missing_argument_between_commas() {
        let error = Lens::parse("between(a,)").expect_err("must fail");
        assert!(matches!(error, LensParseError::MissingArgument { .. }));
    }

    // ---------------------------------------------------------------
    // Paper exercise: express the dream's selection as lens expressions.
    //
    // The dream's actual selection (per kind):
    //   - "recent-for-agent" UNION "graph-reachable-from-seed up to depth N"
    //   - Memories: globally filtered by level threshold, capped by size
    //
    // These tests confirm the shapes PARSE with today's grammar. Integer
    // literals and context anchors aren't in the grammar yet, so we stand
    // them in with symbols (e.g. `cognition_size`, `current_agent`).
    // The validator stage will be the one to reject those — for the
    // parser, predicate names and symbols are open.
    // ---------------------------------------------------------------

    #[test]
    fn dream_recent_cognitions_sub_query_parses() {
        // Closer to target shape now — integer literal works; anchor still
        // stubbed as `current_agent` until `@agent` lands.
        let parsed =
            Lens::parse("recent(kind(cognition) & agent(current_agent), 12)").expect("parses");
        let Lens::Predicate(predicate) = parsed else {
            panic!("expected top-level predicate");
        };
        assert_eq!(predicate.name.as_str(), "recent");
        assert_eq!(predicate.args.len(), 2);
        assert!(matches!(predicate.args[1], Lens::Integer(_)));
    }

    #[test]
    fn dream_memories_sub_query_parses() {
        // Target shape: recent(kind(memory) & level_at_least(working), 25)
        let parsed = Lens::parse(
            "recent(kind(memory) & level_at_least(recollection_level), recollection_size)",
        )
        .expect("parses");
        let Lens::Predicate(predicate) = parsed else {
            panic!("expected top-level predicate");
        };
        assert_eq!(predicate.name.as_str(), "recent");
    }

    #[test]
    fn dream_graph_traversal_sub_query_parses() {
        // Target shape: kind(cognition) & reachable(experiences(@agent), 3)
        let parsed =
            Lens::parse("kind(cognition) & reachable(experiences(current_agent), dream_depth)")
                .expect("parses");
        assert!(matches!(parsed, Lens::Intersection(_, _)));
    }

    #[test]
    fn dream_full_cognitions_sub_query_parses() {
        // Full per-kind selection: recent ∪ graph-reachable.
        let parsed = Lens::parse(
            "recent(kind(cognition) & agent(current_agent), recent_window) \
             | (kind(cognition) & reachable(experiences(current_agent), dream_depth))",
        )
        .expect("parses");
        assert!(matches!(parsed, Lens::Union(_, _)));
    }
}
