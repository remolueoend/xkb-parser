use crate::xkb::Rule;
use derivative::Derivative;
use pest::Span;
use pest_ast::FromPest;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::file))]
pub struct File<'src> {
    pub definitions: Vec<Definition<'src>>,
    #[derivative(Debug = "ignore")]
    eoi: EOI,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::definition))]
pub struct Definition<'src> {
    pub what: Option<What<'src>>,
    pub symbols: Symbols<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::what))]
pub struct What<'src> {
    pub how: How,
    pub name: Vec<Ident<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::how))]
pub enum How {
    DefaultPartial(DefaultPartial),
    HiddenPartial(HiddenPartial),
    Partial(Partial),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::default_partial))]
pub struct DefaultPartial;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::hidden_partial))]
pub struct HiddenPartial;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::partial))]
pub struct Partial;

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::symbols))]
pub struct Symbols<'src> {
    pub name: StringContent<'src>,
    pub symbols: Vec<Symbol<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::symbol))]
pub enum Symbol<'src> {
    #[derivative(Debug = "transparent")]
    Include(Include<'src>),
    #[derivative(Debug = "transparent")]
    Name(Name<'src>),
    #[derivative(Debug = "transparent")]
    Key(Key<'src>),
    #[derivative(Debug = "transparent")]
    ModifierMap(ModifierMap<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::include))]
pub struct Include<'src> {
    pub name: StringContent<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::name))]
pub struct Name<'src> {
    pub group: Ident<'src>,
    pub name: StringContent<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::key))]
pub struct Key<'src> {
    pub id: Ident2<'src>,
    pub values: KeyValues<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_values))]
pub enum KeyValues<'src> {
    KeyNames(KeyNames<'src>),
    KeyDefs(KeyDefs<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_names))]
pub struct KeyNames<'src> {
    pub values: Vec<Ident<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_defs))]
pub struct KeyDefs<'src> {
    pub values: Vec<KeyDef<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::key_def))]
pub enum KeyDef<'src> {
    TypeDef(TypeDef<'src>),
    SymbolDef(SymbolDef<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::type_def))]
pub struct TypeDef<'src> {
    #[pest_ast(inner(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::symbol_def))]
pub struct SymbolDef<'src> {
    pub name: Ident<'src>,
    pub values: KeyNames<'src>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::modifier_map))]
pub struct ModifierMap<'src> {
    pub name: Ident<'src>,
    pub values: Vec<Modifier<'src>>,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::modifier))]
pub enum Modifier<'src> {
    KeyId(Ident2<'src>),
    Ident(Ident<'src>),
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::ident))]
pub struct Ident<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::ident2))]
pub struct Ident2<'src> {
    #[pest_ast(inner(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug = "transparent")]
#[pest_ast(rule(Rule::string_content))]
pub struct StringContent<'src> {
    #[pest_ast(outer(with(span_into_str)))]
    pub content: &'src str,
}

#[derive(Derivative, FromPest, Clone, PartialEq)]
#[derivative(Debug)]
#[pest_ast(rule(Rule::EOI))]
struct EOI;

fn span_into_str(span: Span) -> &str {
    span.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xkb::{Rule, XkbParser};
    use from_pest::FromPest;
    use pest::Parser;
    use std::fmt::Debug;

    #[test]
    fn test_ast_how() {
        enable_logging();

        assert_parse(Rule::how, "default partial\n", How::DefaultPartial(DefaultPartial));
        assert_parse(Rule::how, "hidden partial\n", How::HiddenPartial(HiddenPartial));
        assert_parse(Rule::how, "partial\n", How::Partial(Partial));
    }

    #[test]
    fn test_ast_ident() {
        enable_logging();

        assert_parse(Rule::ident, "foobar\n", Ident { content: "foobar" });
    }

    #[test]
    fn test_ast_what() {
        enable_logging();

        assert_parse(
            Rule::what,
            "default partial alphanumeric_keys\n",
            What {
                how: How::DefaultPartial(DefaultPartial),
                name: vec![Ident { content: "alphanumeric_keys" }],
            },
        );

        assert_parse(
            Rule::what,
            "default partial alphanumeric_keys modifier_keys\n",
            What {
                how: How::DefaultPartial(DefaultPartial),
                name: vec![
                    Ident { content: "alphanumeric_keys" },
                    Ident { content: "modifier_keys" },
                ],
            },
        );
    }

    #[test]
    fn test_ast_symbol() {
        enable_logging();

        assert_parse(
            Rule::symbol,
            "key <ESC>  {	[ Escape		]	};",
            Symbol::Key(Key {
                id: Ident2 { content: "ESC" },
                values: KeyValues::KeyNames(KeyNames { values: vec![Ident { content: "Escape" }] }),
            }),
        );

        assert_parse(
            Rule::symbol,
            "key <LSGT> {	[ less, greater, bar, brokenbar ] };",
            Symbol::Key(Key {
                id: Ident2 { content: "LSGT" },
                values: KeyValues::KeyNames(KeyNames {
                    values: vec![
                        Ident { content: "less" },
                        Ident { content: "greater" },
                        Ident { content: "bar" },
                        Ident { content: "brokenbar" },
                    ],
                }),
            }),
        );

        assert_parse(
            Rule::symbol,
            "key <PRSC> {\n\ttype= \"PC_ALT_LEVEL2\",\n\tsymbols[Group1]= [ Print, Sys_Req ]\n    };",
            Symbol::Key(Key {
                id: Ident2 { content: "PRSC" },
                values: KeyValues::KeyDefs(KeyDefs {
                    values: vec![
                        KeyDef::TypeDef(TypeDef { content: "PC_ALT_LEVEL2" }),
                        KeyDef::SymbolDef(SymbolDef { name: Ident { content: "Group1" }, values: KeyNames { values: vec![
                            Ident { content: "Print" },
                            Ident { content: "Sys_Req" },
                        ] } } ),
                    ]
                })
            }),
        );

        assert_parse(
            Rule::symbol,
            r#"include "srvr_ctrl(fkey2vt)""#,
            Symbol::Include(Include { name: StringContent { content: "srvr_ctrl(fkey2vt)" } }),
        );

        assert_parse(
            Rule::symbol,
            "modifier_map Shift  { Shift_L, Shift_R };",
            Symbol::ModifierMap(ModifierMap {
                name: Ident { content: "Shift" },
                values: vec![
                    Modifier::Ident(Ident { content: "Shift_L" }),
                    Modifier::Ident(Ident { content: "Shift_R" }),
                ],
            }),
        );

        assert_parse(
            Rule::symbol,
            "modifier_map Mod4 { <META>, Meta_L, Meta_R };",
            Symbol::ModifierMap(ModifierMap {
                name: Ident { content: "Mod4" },
                values: vec![
                    Modifier::KeyId(Ident2 { content: "META" }),
                    Modifier::Ident(Ident { content: "Meta_L" }),
                    Modifier::Ident(Ident { content: "Meta_R" }),
                ],
            }),
        );
    }

    fn enable_logging() {
        let _ = env_logger::builder()
            .filter(None, log::LevelFilter::Trace)
            .default_format_timestamp(false)
            .is_test(true)
            .try_init();
    }

    fn assert_parse<'i, T>(r: Rule, input: &'i str, expected: T)
    where
        T: FromPest<'i, Rule = Rule> + PartialEq + Debug,
        <T as FromPest<'i>>::FatalError: Debug,
    {
        let mut parse_tree = match XkbParser::parse(r, input) {
            Ok(parse_tree) => {
                println!("parse tree = {:#?}", parse_tree);
                parse_tree
            }
            Err(e) => {
                panic!("Failed to parse `{}` as {:?}: `{}`", input, r, e);
            }
        };

        let syntax_tree = T::from_pest(&mut parse_tree).expect("infallible");
        println!("syntax tree = {:#?}", syntax_tree);

        assert_eq!(syntax_tree, expected);
    }
}
