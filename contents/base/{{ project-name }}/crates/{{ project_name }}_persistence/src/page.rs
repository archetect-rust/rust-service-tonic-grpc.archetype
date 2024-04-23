pub struct Page<T> {
    pub records: Vec<T>,
    pub index: usize,
    pub next: usize,
    pub has_next: bool,
    pub previous: usize,
    pub has_previous: bool,
    pub total_pages: usize,
    pub total_records: usize,
}

impl<T> Page<T> {
    pub fn new(records: Vec<T>, index: usize, page_size: usize, total_records: usize) -> Page<T> {
        let total_pages = (total_records as f32 / page_size as f32).ceil() as usize;
        let index = index.min(if total_pages == 0 { 0 } else { total_pages - 1 });
        let next = (index + 1).min(if total_pages == 0 { 0 } else { total_pages - 1 });
        let has_next = next != index;
        let previous = if index == 0 { 0 } else { index - 1 };
        let has_previous = previous != index;
        Page {
            records,
            index,
            next,
            has_next,
            previous,
            has_previous,
            total_pages,
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
        assert_eq!(page.total_records, 100);
        assert_eq!(page.total_pages, 10);
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
        assert_eq!(page.total_records, 100);
        assert_eq!(page.total_pages, 10);
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
        assert_eq!(page.total_records, 100);
        assert_eq!(page.total_pages, 10);
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
        assert_eq!(page.total_records, 0);
        assert_eq!(page.total_pages, 0);
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
        assert_eq!(page.total_records, 5);
        assert_eq!(page.total_pages, 1);
    }
}