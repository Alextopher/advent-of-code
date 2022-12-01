use std::cmp::Ordering;

impl<T: ?Sized> IterJunk for T where T: Iterator {}

pub trait IterJunk: Iterator {
    /// Uses repeated selection sort to return the K largest elements
    fn selection_sorted(self) -> Selection<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        Selection { v: self.collect() }
    }
}

pub struct Selection<Item>
where
    Item: Ord,
{
    v: Vec<Item>,
}

impl<Item: Ord> Iterator for Selection<Item> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        // Move the smallest element to the end of the array
        let len = self.v.len();
        for i in 0..len {
            match self.v[i].cmp(&self.v[len - 1]) {
                Ordering::Less => self.v.swap(i, len - 1),
                _ => {}
            }
        }

        // Remove the smallest element
        self.v.pop()
    }
}

impl<Item: Ord> DoubleEndedIterator for Selection<Item> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // Move the largest element to the end of the array
        let len = self.v.len();
        for i in 0..len {
            match self.v[i].cmp(&self.v[len - 1]) {
                Ordering::Greater => self.v.swap(i, len - 1),
                _ => {}
            }
        }

        // Remove the largest element
        self.v.pop()
    }
}
