pub struct GroupByKey<'a, T, M> {
    slice: &'a [T],
    by: M,
}

impl<'a, T, K: PartialEq, M: Fn(&T) -> K> GroupByKey<'a, T, M> {
    pub fn new(slice: &'a [T], by: M) -> Self {
        Self { slice, by }
    }
}

impl<'a, T, K: PartialEq, M: Fn(&T) -> K> Iterator for GroupByKey<'a, T, M> {
    type Item = (K, &'a [T]);
    fn next(&mut self) -> Option<Self::Item> {
        let mut last_key = None;
        let mut index = 0;
        for item in self.slice.into_iter() {
            let next_key = (self.by)(item);
            match &last_key {
                Some(last_key) if last_key != &next_key => {
                    break;
                }
                None => {
                    last_key = Some(next_key);
                }
                _ => {}
            }
            index += 1;
        }
        if let Some(last_key) = last_key {
            let ret;
            (ret, self.slice) = self.slice.split_at(index);
            Some((last_key, ret))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        #[derive(Debug, PartialEq)]
        struct Foo(u8);
        let slice = [Foo(1), Foo(1), Foo(2), Foo(2), Foo(2), Foo(1), Foo(3)];
        let res: Vec<_> = GroupByKey::new(&slice, |x| x.0).collect();
        assert_eq!(
            res,
            &[
                (1, &[Foo(1), Foo(1)][..]),
                (2, &[Foo(2), Foo(2), Foo(2)][..]),
                (1, &[Foo(1)][..]),
                (3, &[Foo(3)][..])
            ]
        );

        let slice: [Foo; 0] = [];
        assert!(GroupByKey::new(&slice, |x| x.0).next().is_none());
    }
}
