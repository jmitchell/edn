#![allow(dead_code)]

use std::collections::{LinkedList};

#[derive(Debug,PartialEq)]
pub enum BasicElement {
    Nil,
    Boolean(bool),
    String(String),
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
        assert_eq!(String("abc".to_string()), String("abc".to_string()));
        assert!(String("abc".to_string()) != String("ABC".to_string()));
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
    use super::BasicElement::*;
    use super::Edn;
    use super::Edn::*;

    use pc::{string};
    use pc::primitives::{Parser,State};
    use pc::combinator::{ParserExt};

    #[derive(Debug,PartialEq)]
    pub struct ReadError {
        // TODO: provide more information (parser-combinator's error
        // reporting is nice, but I don't want to expose any of its
        // API)
        pub message: &'static str
    }

    pub fn read_edn(input: &str) -> Result<Edn, ReadError> {
        let state = State::new(input);

        let nil = string("nil");
        let bool_true = string("true");
        let bool_false = string("false");

        let mut parser = nil.map(|_| Basic(Nil))
            .or(bool_true.map(|_| Basic(Boolean(true))))
            .or(bool_false.map(|_| Basic(Boolean(false))));

        match parser.parse_state(state) {
            Ok((edn,_)) => Ok(edn),
            Err(_) => Err(ReadError { message: "some kind of error!" }),
        }
    }

    #[cfg(test)]
    mod test {
        use super::{read_edn};
        use super::super::BasicElement::*;
        use super::super::Edn::*;

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
    }
}
