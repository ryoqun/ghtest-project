//! ghtest_project
//! # Document
//! 自動でドキュメントを作成して公開するテストです。yay push ayayayay test from forks1 jjjkkk

#[cfg_attr(doc, aquamarine::aquamarine)]
/// ```mermaid
/// graph LR
///     s([Source]) --> a[[aquamarine]]
///     r[[rustdoc]] --> f([Docs w/ Mermaid!])
///     subgraph rustc[Rust Compiler]
///     a -. inject mermaid.js .-> r
///     end
///     x([X]) --> y([Y])
/// ```
pub fn add(a: u64, b: u64) -> u64 {
    a + b
}

/// a - b を計算します
/// # Panics
/// a < b の時、オーバーフローします
/// # Examples
/// ```
/// use crate::ghtest_project::sub;
/// let (a, b) = (3, 2);
/// assert_eq!(sub(a, b), 1);
/// ```
/// 👻
pub fn sub(a: u64, b: u64) -> u64 {
    a - b
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_test() {
        let (a, b) = (3, 2);
        assert_eq!(add(a, b), 5);
    }

    #[test]
    fn sub_test() {
        let (a, b) = (3, 2);
        assert_eq!(sub(a, b), 1);
    }
}
