pub trait VecExt<T: PartialOrd> {
    fn selection_sort(&mut self);
}

impl<T: PartialOrd + Ord> VecExt<T> for Vec<T> {
    fn selection_sort(&mut self) {
        let n = self.len();

        (0..n - 1).for_each(|bound| {
            if let Some(current_min_pos) = (bound + 1..n).min_by_key(|&j| &self[j])
            {
                if self[current_min_pos] < self[bound] {
                    self.swap(bound, current_min_pos);
                }
            }
        });
    }
}

fn main() {
    let mut testvec = Vec::from([8, 11, 4, 6, 9]);
    testvec.selection_sort();
    println!("{:?}",testvec);
}
