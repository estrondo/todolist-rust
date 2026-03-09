use arbitrary::Arbitrary;
use time::{Duration, UtcDateTime};

use super::super::TodoDueDate;

impl<'a> Arbitrary<'a> for TodoDueDate {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let now = UtcDateTime::now().truncate_to_second();
        let days = u.int_in_range(-10..=10)?;

        if let Ok(1) = u.choose_index(1) {
            let new_date = now.date() + Duration::days(days);
            Ok(TodoDueDate::WholeDay(new_date))
        } else {
            let hours = u.int_in_range(0..=12)?;
            Ok(TodoDueDate::Period(
                now + Duration::days(days),
                Duration::hours(hours),
            ))
        }
    }
}
