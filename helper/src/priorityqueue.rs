#[derive(Debug, Clone, Copy)]
pub struct MinPQEntry<T> {
    pub data: T,
    pub priority: isize,
}

impl<T> MinPQEntry<T> {
    pub fn new(priority: isize, data: T) -> Self {
        Self { priority, data }
    }
}

impl<T> Ord for MinPQEntry<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<T> PartialOrd for MinPQEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl<T> PartialEq for MinPQEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T> Eq for MinPQEntry<T> {}

#[derive(Debug, Clone, Copy)]
pub struct MaxPQEntry<T, C>
where
    C: Ord + Eq,
{
    pub data: T,
    pub priority: C,
}

impl<T, C> MaxPQEntry<T, C>
where
    C: Ord + Eq,
{
    pub fn new(priority: C, data: T) -> Self {
        Self { priority, data }
    }
}

impl<T, C> Ord for MaxPQEntry<T, C>
where
    C: Ord + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<T, C> PartialOrd for MaxPQEntry<T, C>
where
    C: Ord + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T, C> PartialEq for MaxPQEntry<T, C>
where
    C: Ord + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T, C> Eq for MaxPQEntry<T, C> where C: Ord + Eq {}

#[test]
fn test_reference() {
    let a = MaxPQEntry::<&str, isize>::new(1_isize, "Hello");
    let b = MaxPQEntry::new(-1, "Zzzz");
    assert!(a < b);
}
