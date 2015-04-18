#![allow(dead_code)]

use std::collections::{LinkedList};

#[derive(Debug,PartialEq)]
pub enum BasicElement {
    Nil,
    Boolean(bool),
    EString(String),
    Character(char),
    Symbol(String),
    Keyword(String),
    Integer(i64),
    // TODO: arbitrary precision integer
    // TODO: 64-bit signed floating point
    // TODO: "exact precision" decimal
}

#[derive(Debug,PartialEq)]
pub enum TaggedElement {
    // TODO: arbitrary tagged elements
    // TODO: #inst
    // TODO: #uuid
}

// TODO: move BasicSet into a separate module
#[derive(Debug)]
pub struct BasicSet<T> {
    // TODO: confirm Set is public, but elements aren't.
    elements: Vec<T>
}

impl<T> BasicSet<T> {
    // TODO: provide constructor and methods for adding/removing
    // elements, querying for membership, and iterating.
}

impl<T: PartialEq> PartialEq for BasicSet<T> {
    fn eq(&self, other: &BasicSet<T>) -> bool {
        if self.elements.len() != other.elements.len() {
            false
        } else {
            for i in self.elements.iter() {
                let mut found_match = false;
                for j in other.elements.iter() {
                    if &i == &j {
                        found_match = true;
                    }
                }
                if !found_match {
                    return false
                }
            }
            true
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum Edn {
    Basic(BasicElement),
    Tagged(TaggedElement),
    List(LinkedList<Edn>),
    Vector(Vec<Edn>),
    Map(BasicSet<(Edn,Edn)>),
    Set(BasicSet<Edn>),
}

#[cfg(test)]
mod test {
    use super::BasicElement::*;
    use super::BasicSet;
    use super::Edn::*;
    use std::collections::{LinkedList};

    #[test]
    fn build_nested_heterogeneous_vector() {
        Vector(vec![
            Vector(vec![Basic(Integer(1)),
                        Basic(Integer(2)),
                        Basic(Character('c'))]),
            Basic(Boolean(true)),
            Vector(vec![Basic(Keyword(":test".to_string())),
                        Basic(Boolean(false))])]);
    }

    #[test]
    fn nil_equals_nil() {
        assert_eq!(Nil, Nil)
    }

    #[test]
    fn bool_equality() {
        assert_eq!(Boolean(true), Boolean(true));
        assert_eq!(Boolean(false), Boolean(false));
        assert!(Boolean(false) != Boolean(true));
        assert!(Boolean(true) != Boolean(false));
    }

    #[test]
    fn string_equality() {
        assert_eq!(EString("abc".to_string()), EString("abc".to_string()));
        assert!(EString("abc".to_string()) != EString("ABC".to_string()));
    }

    #[test]
    fn character_equality() {
        assert_eq!(Character('x'), Character('x'));
        assert!(Character('x') != Character('y'));
    }

    #[test]
    fn symbol_equality() {
        assert_eq!(Symbol("+".to_string()), Symbol("+".to_string()));
        assert!(Symbol("+".to_string()) != Symbol("math/add".to_string()));
    }

    #[test]
    fn keyword_equality() {
        assert_eq!(Keyword(":bob".to_string()), Keyword(":bob".to_string()));
        assert!(Keyword(":bob".to_string()) != Keyword(":alice".to_string()));
    }

    #[test]
    fn integer_equality() {
        assert_eq!(Integer(1), Integer(1));
        assert_eq!(Integer(-1), Integer(-1));
        assert_eq!(Integer(0), Integer(-0));
        assert!(Integer(0) != Integer(1));
    }

    #[test]
    fn empty_list_equals_empty_list() {
        assert_eq!(List(LinkedList::new()), List(LinkedList::new()));
    }

    #[test]
    fn list_123_equals_list_123() {
        let mut x = LinkedList::new();
        x.push_back(Basic(Integer(1)));
        x.push_back(Basic(Integer(2)));
        x.push_back(Basic(Integer(3)));

        let mut y = LinkedList::new();
        y.push_back(Basic(Integer(1)));
        y.push_back(Basic(Integer(2)));
        y.push_back(Basic(Integer(3)));

        assert_eq!(List(x), List(y));
    }

    #[test]
    fn vec_123_equals_vec_123() {
        let x = vec![Basic(Integer(1)), Basic(Integer(2)), Basic(Integer(3))];
        let y = vec![Basic(Integer(1)), Basic(Integer(2)), Basic(Integer(3))];
        assert_eq!(Vector(x), Vector(y));
    }

    #[test]
    fn map_12_ab_equals_map_ab_12() {
        let x = BasicSet {
            elements: vec![
                (Basic(Integer(1)), Basic(Integer(2))),
                (Basic(Character('a')), Basic(Character('b')))]
        };
        let y = BasicSet {
            elements: vec![
                (Basic(Character('a')), Basic(Character('b'))),
                (Basic(Integer(1)), Basic(Integer(2)))]
        };
        assert_eq!(Map(x), Map(y));
    }

    #[test]
    fn set_123_equals_set_321() {
        let x = BasicSet {
            elements: vec![
                Basic(Integer(1)),
                Basic(Integer(2)),
                Basic(Integer(3))]
        };
        let y = BasicSet {
            elements: vec![
                Basic(Integer(1)),
                Basic(Integer(2)),
                Basic(Integer(3))]
        };
        assert_eq!(Set(x), Set(y));
    }
}






mod reader {
    extern crate parser_combinators as pc;

    use super::BasicElement::*;
    use super::BasicSet;
    use super::Edn;
    use super::Edn::*;

    use self::pc::{many1,digit,spaces,string,parser,sep_by,between,ParseResult};
    use self::pc::primitives::{Parser,State};
    use self::pc::combinator::{ParserExt};

    // TODO: element delimiters (whitespace, other than within strings, and commas)

    fn boolean(input: State<&str>) -> ParseResult<bool, &str> {
        string("true").map(|_| true)
            .or(string("false").map(|_| false))
            .parse_state(input)
    }

    fn integer(input: State<&str>) -> ParseResult<i64, &str> {
        // TODO: optional '+' or '-' prefix
        // TODO: constrain first digit to non-zero when multiple digits or prefix
        // TODO: arbitrary precision 'N' suffix

        many1(digit())
            .map(|digits: String|
                 match digits.parse::<i64>() {
                     Ok(n) => n,
                     Err(_) => panic!("falls outside i64's range!"),
                 })
            .parse_state(input)
    }

    // TODO: strings
    // TODO: characters
    // TODO: symbols
    // TODO: keywords

    // TODO: floats

    fn list(input: State<&str>) -> ParseResult<Edn, &str> {
        between(string("("),
                string(")"),
                sep_by(parser(parse_edn), spaces()))
            .map(|xs| List(xs))
            .parse_state(input)
    }

    fn vector(input: State<&str>) -> ParseResult<Edn, &str> {
        between(string("["),
                string("]"),
                sep_by(parser(parse_edn), spaces()))
            .map(|xs| Vector(xs))
            .parse_state(input)
    }

    // TODO: maps

    fn set(input: State<&str>) -> ParseResult<Edn, &str> {
        between(string("#{"),
                string("}"),
                sep_by(parser(parse_edn), spaces()))
            .map(|xs| Set(BasicSet { elements: xs }))
            .parse_state(input)
    }


    // TODO: #inst
    // TODO: #uuid
    // TODO: generic tagged element

    // TODO: comments
    // TODO: discard sequence

    fn parse_edn(input: State<&str>) -> ParseResult<Edn, &str> {
        string("nil").map(|_| Basic(Nil))
            .or(parser(boolean).map(|b| Basic(Boolean(b))))
            .or(parser(integer).map(|n| Basic(Integer(n))))
            .or(parser(list))
            .or(parser(vector))
            .or(parser(set))
            .parse_state(input)
    }

    pub fn read_edn(input: &str) -> Result<Edn, &str> {
        let state = State::new(input);

        match parse_edn(state) {
            Ok((edn, _)) => Ok(edn),
            Err(_) => Err("some kind of error!"),
        }
    }

    #[cfg(test)]
    mod test {
        use super::{read_edn};
        use super::super::BasicElement::*;
        use super::super::BasicSet;
        use super::super::Edn::*;

        use std::collections::{LinkedList};

        #[test]
        fn parses_nil() {
            let result = read_edn("nil");
            assert_eq!(result, Ok(Basic(Nil)));
        }

        #[test]
        fn failed_parse_nil() {
            let result = read_edn("not_nil");
            assert!(result.is_err());
        }

        #[test]
        fn parses_true() {
            let result = read_edn("true");
            assert_eq!(result, Ok(Basic(Boolean(true))));
        }

        #[test]
        fn parses_false() {
            let result = read_edn("false");
            assert_eq!(result, Ok(Basic(Boolean(false))));
        }

        #[test]
        fn parse_zero_int() {
            assert_eq!(read_edn("0"), Ok(Basic(Integer(0))));
        }

        #[test]
        fn parse_max_64_bit_signed_int() {
            assert_eq!(read_edn("9223372036854775807"), Ok(Basic(Integer(9223372036854775807))));
        }

        #[test]
        #[should_panic]
        #[allow(unused)]
        fn parse_overflowed_int() {
            // TODO: Reassess API.
            //
            // Maybe the right thing to do is auto-promote to arbitrary precision.
            let _ = read_edn("9223372036854775808");
        }

        #[test]
        fn parse_single_element_list() {
            let mut list = LinkedList::new();
            list.push_back(Basic(Nil));

            assert_eq!(read_edn("(nil)"), Ok(List(list)));
        }

        #[test]
        fn parse_empty_list() {
            assert_eq!(read_edn("()"), Ok(List(LinkedList::new())));
        }

        #[test]
        fn parse_nested_lists() {
            let empty = List(LinkedList::new());

            let mut false_list = LinkedList::new();
            false_list.push_back(Basic(Boolean(false)));

            let mut one_and_false_list = LinkedList::new();
            one_and_false_list.push_back(Basic(Integer(1)));
            one_and_false_list.push_back(List(false_list));

            let mut outer_list = LinkedList::new();
            outer_list.push_back(empty);
            outer_list.push_back(List(one_and_false_list));

            assert_eq!(read_edn("(() (1 (false)))"), Ok(List(outer_list)));
        }

        #[test]
        fn parse_single_element_vector() {
            assert_eq!(read_edn("[nil]"), Ok(Vector(vec![Basic(Nil)])));
        }

        #[test]
        fn parse_empty_vector() {
            assert_eq!(read_edn("[]"), Ok(Vector(vec![])));
        }

        #[test]
        fn parse_three_element_vector() {
            assert_eq!(read_edn("[nil nil nil]"), Ok(Vector(vec![Basic(Nil), Basic(Nil), Basic(Nil)])));
        }

        #[test]
        fn parse_nested_vectors() {
            assert_eq!(read_edn("[[] [1 [false]]]"), Ok(
                Vector(vec![
                    Vector(vec![]),
                    Vector(vec![
                        Basic(Integer(1)),
                        Vector(vec![
                            Basic(Boolean(false))])])])))
        }

        #[test]
        fn parse_empty_set() {
            assert_eq!(read_edn("#{}"), Ok(Set(BasicSet { elements: vec![] })));
        }

        #[test]
        fn parse_mixed_set() {
            assert_eq!(read_edn("#{1 [2 3] #{4}}"),
                       // arbitrary order
                       Ok(Set(BasicSet { elements: vec![
                           Set(BasicSet { elements: vec![
                               Basic(Integer(4))]}),
                           Basic(Integer(1)),
                           Vector(vec![
                               Basic(Integer(2)),
                               Basic(Integer(3))])]})));
        }
    }
}
