//! ghtest_project
//! # Document
//! 自動でドキュメントを作成して公開するテストです。

/// a - b を計算します
/// # Panics
/// a < b の時、オーバーフローします
/// # Examples
/// ```
/// use crate::ghtest_project::sub;
/// let (a, b) = (3, 2);
/// assert_eq!(sub(a, b), 1);
/// ```
pub fn sub(a: u64, b: u64) -> u64 {
    a - b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
