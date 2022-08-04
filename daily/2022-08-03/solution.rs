use std::collections::btree_map::BTreeMap;

struct MyCalendar {
    bookings: BTreeMap<i32, i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        Self {
            bookings: BTreeMap::new()
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((_, &next_end)) = self.bookings.range(..end).next_back() {
            if start < next_end {
                return false;
            }
        }
        self.bookings.insert(start, end);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */