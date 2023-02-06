use std::collections::BTreeMap;

struct MyCalendar {
    /// key: start, val: end
    scheduled: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            scheduled: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let schduled = &mut self.scheduled;

        let time = schduled.range(..end).next_back();
        let can_book = if let Some((&_start, &_end)) = time {
            if _end <= start {
                true
            } else {
                false
            }
        } else {
            true
        };
        if can_book {
            schduled.insert(start, end);
        }

        can_book
    }
}

fn main() {
    // println!("{:?}", );
}
