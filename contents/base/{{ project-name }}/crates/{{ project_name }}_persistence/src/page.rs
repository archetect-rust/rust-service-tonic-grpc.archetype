pub struct Page<T> {
    pub records: Vec<T>,
    pub index: u32,
    pub next: u32,
    pub has_next: bool,
    pub previous: u32,
    pub has_previous: bool,
    pub total: u32,
    pub total_records: u64,
}

impl<T> Page<T> {
    pub fn new(records: Vec<T>, index: u32, page_size: u32, total_records: u64) -> Page<T> {
        let total = (total_records as f32 / page_size as f32).ceil() as u32;
        let index = index.min(if total == 0 { 0 } else { total - 1 });
        let next = (index + 1).min(if total == 0 { 0 } else { total - 1 });
        let has_next = next != index;
        let previous = if index == 0 { 0 } else { index - 1 } as u32;
        let has_previous = previous != index;
        Page {
            records,
            index,
            next,
            has_next,
            previous,
            has_previous,
            total,
            total_records,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page() {
        let records: Vec<String> = vec![];
        let page = Page::new(records, 0, 10, 100);
        assert_eq!(page.index, 0);
        assert_eq!(page.next, 1);
        assert_eq!(page.has_next, true);
        assert_eq!(page.previous, 0);
        assert_eq!(page.has_previous, false);
        assert_eq!(page.total, 10);
        assert_eq!(page.total_records, 100);
    }


    #[test]
    fn test_page_middle_page() {
        let records: Vec<String> = vec![];
        let page = Page::new(records, 5, 10, 100);
        assert_eq!(page.index, 5);
        assert_eq!(page.next, 6);
        assert_eq!(page.has_next, true);
        assert_eq!(page.previous, 4);
        assert_eq!(page.has_previous, true);
        assert_eq!(page.total, 10);
        assert_eq!(page.total_records, 100);
    }

    #[test]
    fn test_page_last_page() {
        let records: Vec<String> = vec![];
        let page = Page::new(records, 9, 10, 100);
        assert_eq!(page.index, 9);
        assert_eq!(page.next, 9);
        assert_eq!(page.has_next, false);
        assert_eq!(page.previous, 8);
        assert_eq!(page.has_previous, true);
        assert_eq!(page.total, 10);
        assert_eq!(page.total_records, 100);
    }

    #[test]
    fn test_page_no_results_start_0() {
        let records: Vec<String> = vec![];
        let page = Page::new(records, 0, 10, 0);
        assert_eq!(page.index, 0);
        assert_eq!(page.next, 0);
        assert_eq!(page.has_next, false);
        assert_eq!(page.previous, 0);
        assert_eq!(page.has_previous, false);
        assert_eq!(page.total, 0);
        assert_eq!(page.total_records, 0);
    }

    #[test]
    fn test_page_start_exceeds_total() {
        let records: Vec<String> = vec![];
        let page = Page::new(records, 10, 10, 5);
        assert_eq!(page.index, 0);
        assert_eq!(page.next, 0);
        assert_eq!(page.has_next, false);
        assert_eq!(page.previous, 0);
        assert_eq!(page.has_previous, false);
        assert_eq!(page.total, 1);
        assert_eq!(page.total_records, 5);
    }
}