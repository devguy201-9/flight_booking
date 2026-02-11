#[inline]
pub fn optimistic_ok(rows_affected: u64) -> bool {
    rows_affected > 0
}