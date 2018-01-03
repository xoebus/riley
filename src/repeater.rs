#[derive(Clone, Debug)]
pub struct Repeater<I: Iterator> {
    iter: I,
    dups: usize,
    curr: Option<I::Item>,
    done: usize,
}

impl<I> Repeater<I>
where
    I: Clone + Iterator,
{
    pub fn new(it: I, dups: usize) -> Self {
        Repeater {
            iter: it,
            dups: dups,
            curr: None,
            done: 0,
        }
    }
}

impl<I> Iterator for Repeater<I>
where
    I: Clone + Iterator,
    I::Item: Clone,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        if self.done == 0 {
            self.curr = self.iter.next()
        }

        if self.done < self.dups {
            self.done += 1;
            self.curr.clone()
        } else {
            self.done = 0;
            self.next()
        }
    }
}
