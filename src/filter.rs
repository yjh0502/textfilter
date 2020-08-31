use fst::automaton::*;
use fst::{IntoStreamer, Set};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub struct Substring<'a> {
    subseq: &'a [u8],
    pub ignore_whitespace: bool,
    pub case_insensitive: bool,
}

impl<'a> Substring<'a> {
    /// Constructs automaton that matches input containing the
    /// specified subsequence.
    #[inline]
    pub fn new(subsequence: &'a str) -> Self {
        Self {
            subseq: subsequence.as_bytes(),
            ignore_whitespace: false,
            case_insensitive: false,
        }
    }
}

#[derive(Clone)]
pub enum SubstringState {
    Index(usize),
    NotMatched,
    Matched(usize),
}

impl<'a> Automaton for Substring<'a> {
    type State = SubstringState;

    #[inline]
    fn start(&self) -> Self::State {
        SubstringState::Index(0)
    }

    #[inline]
    fn is_match(&self, state: &Self::State) -> bool {
        if let SubstringState::Matched(_) = state {
            true
        } else {
            false
        }
    }

    #[inline]
    fn can_match(&self, state: &Self::State) -> bool {
        match state {
            SubstringState::Index(_) => true,
            SubstringState::Matched(_) => true,
            SubstringState::NotMatched => false,
        }
    }

    #[inline]
    fn will_always_match(&self, state: &Self::State) -> bool {
        match state {
            SubstringState::Index(_) => false,
            SubstringState::Matched(_) => true,
            SubstringState::NotMatched => false,
        }
    }

    #[inline]
    fn accept(&self, state: &Self::State, byte: u8) -> Self::State {
        match state {
            SubstringState::Index(idx) => {
                let mut idx = *idx;
                if self.ignore_whitespace {
                    while idx < self.subseq.len() && self.subseq[idx].is_ascii_whitespace() {
                        idx += 1;
                    }
                }
                if idx == self.subseq.len() {
                    SubstringState::Index(idx)
                } else {
                    let matched = if self.case_insensitive {
                        byte.to_ascii_lowercase() == self.subseq[idx].to_ascii_lowercase()
                    } else {
                        byte == self.subseq[idx]
                    };

                    if matched {
                        SubstringState::Index(idx + 1)
                    } else {
                        SubstringState::NotMatched
                    }
                }
            }
            other => other.clone(),
        }
    }

    #[inline]
    fn accept_eof(&self, state: &Self::State) -> Option<Self::State> {
        match state {
            SubstringState::Index(idx) => Some(SubstringState::Matched(*idx)),
            SubstringState::Matched(idx) => Some(SubstringState::Matched(*idx)),
            SubstringState::NotMatched => None,
        }
    }
}

#[derive(serde_derive::Serialize)]
pub struct FilterResult {
    result: String,
    keywords: Vec<String>,
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

    pub fn filter(&self, s: &str) -> FilterResult {
        self.filter_opts(s, false, false)
    }

    pub fn filter_opts(
        &self,
        s: &str,
        ignore_whitespace: bool,
        case_insensitive: bool,
    ) -> FilterResult {
        use fst::Streamer;

        let mut out = String::new();
        let mut last_idx = 0;
        let mut keywords = Vec::new();

        for (idx, ch) in s.char_indices() {
            if last_idx > idx {
                continue;
            }
            if ch.is_ascii_whitespace() {
                out.push(ch);
                continue;
            }

            let mut subseq = Substring::new(&s[idx..]);
            subseq.ignore_whitespace = ignore_whitespace;
            subseq.case_insensitive = case_insensitive;

            let mut keyword = None;
            let mut stream = self.inner.search_with_state(subseq).into_stream();
            while let Some((_m, state)) = stream.next() {
                if let SubstringState::Index(end_idx) = state {
                    let next_idx = idx + end_idx;
                    if next_idx > last_idx {
                        last_idx = next_idx;
                        keyword = std::str::from_utf8(_m).ok().map(|s| s.to_string());
                    }
                }
            }

            if last_idx > idx {
                if let Some(keyword) = keyword {
                    for _ in keyword.char_indices() {
                        out.push('*');
                    }
                    keywords.push(keyword);
                }
            } else {
                out.push(ch);
            }
        }

        FilterResult {
            result: out,
            keywords,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let keys = &["foo", "bar", "foofo", "한글"];

        let filter = Filter::new(keys);

        // longest match
        assert_eq!(filter.filter("foofo bazbaz").result, "***** bazbaz");
        // exact string
        assert_eq!(filter.filter("foo").result, "***");
        // exact string/unicode
        assert_eq!(filter.filter("한글").result, "**");

        // multiple matches
        assert_eq!(
            filter.filter("foo bazbaz bar foof bar").result,
            "*** bazbaz *** ***f ***"
        );
    }

    #[test]
    fn whitespace() {
        let keys = &["foo", "bar", "foofo", "한글"];

        let filter = Filter::new(keys);

        // longest match
        assert_eq!(
            filter.filter_opts("foo  B a\tr", true, true).result,
            "***  ***"
        );
    }
}
