use fst::automaton::*;
use fst::{IntoStreamer, Set};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub struct Substring<'a> {
    subseq: &'a [u8],
}

impl<'a> Substring<'a> {
    /// Constructs automaton that matches input containing the
    /// specified subsequence.
    #[inline]
    pub fn new(subsequence: &'a str) -> Self {
        Self {
            subseq: subsequence.as_bytes(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SubstringState {
    Index(usize),
    NotMatched,
    Matched,
}

impl<'a> Automaton for Substring<'a> {
    type State = SubstringState;

    #[inline]
    fn start(&self) -> Self::State {
        SubstringState::Index(0)
    }

    #[inline]
    fn is_match(&self, state: &Self::State) -> bool {
        state == &SubstringState::Matched
    }

    #[inline]
    fn can_match(&self, state: &Self::State) -> bool {
        match state {
            SubstringState::Index(_) => true,
            SubstringState::Matched => true,
            SubstringState::NotMatched => false,
        }
    }

    #[inline]
    fn will_always_match(&self, state: &Self::State) -> bool {
        match state {
            SubstringState::Index(_) => false,
            SubstringState::Matched => true,
            SubstringState::NotMatched => false,
        }
    }

    #[inline]
    fn accept(&self, state: &Self::State, byte: u8) -> Self::State {
        match state {
            SubstringState::Index(idx) => {
                if byte == self.subseq[*idx] {
                    if *idx == self.subseq.len() - 1 {
                        SubstringState::Index(*idx)
                    } else {
                        SubstringState::Index(*idx + 1)
                    }
                } else {
                    SubstringState::NotMatched
                }
            }
            other => *other,
        }
    }

    #[inline]
    fn accept_eof(&self, state: &Self::State) -> Option<Self::State> {
        match state {
            SubstringState::Index(_) => Some(SubstringState::Matched),
            SubstringState::Matched => Some(SubstringState::Matched),
            SubstringState::NotMatched => None,
        }
    }
}

pub struct Filter {
    inner: fst::Set<Vec<u8>>,
}

impl Filter {
    pub fn from_js(strings: Box<[JsValue]>) -> Filter {
        let mut list = Vec::new();
        for word in strings.iter() {
            if let Some(s) = word.as_string() {
                list.push(s);
            }
        }
        list.sort();

        let set = Set::from_iter(list.into_iter()).unwrap();
        Filter { inner: set }
    }

    pub fn new(strings: &[&str]) -> Filter {
        let mut list = strings.iter().map(|v| v.to_string()).collect::<Vec<_>>();
        list.sort();

        let set = Set::from_iter(list.into_iter()).unwrap();
        Filter { inner: set }
    }

    pub fn filter(&self, s: &str) -> String {
        use fst::Streamer;

        let mut out = String::new();
        let mut last_idx = 0;
        for (idx, ch) in s.char_indices() {
            if last_idx > idx {
                continue;
            }

            let substr = &s[idx..];

            let subseq = Substring::new(substr);
            let mut stream = self.inner.search(subseq).into_stream();
            while let Some(m) = stream.next() {
                let next_idx = idx + m.len();
                if next_idx > last_idx {
                    last_idx = next_idx;
                }
            }

            if last_idx > idx {
                for _ in s[idx..last_idx].char_indices() {
                    out.push('*');
                }
            } else {
                out.push(ch);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let keys = &["foo", "bar", "foofo", "한글"];

        // longest match
        assert_eq!(Filter::new(keys).filter("foofo bazbaz"), "***** bazbaz");
        // exact string
        assert_eq!(Filter::new(keys).filter("foo"), "***");
        // exact string/unicode
        assert_eq!(Filter::new(keys).filter("한글"), "**");

        // multiple matches
        assert_eq!(
            Filter::new(keys).filter("foo bazbaz bar foof bar"),
            "*** bazbaz *** ***f ***"
        );
    }
}