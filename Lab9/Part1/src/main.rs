pub trait VecExt<T: PartialOrd> {
    fn selection_sort(&mut self);
}

impl<T: PartialOrd> VecExt<T> for Vec<T> {
    fn selection_sort(&mut self) {
        let n = self.len();

        for i in 0..n - 1 {
            let mut current_min_index = i;

            for j in (i + 1)..n {
                if self[j] < self[current_min_index] {
                    current_min_index = j;
                }
            }

            self.swap(i, current_min_index);
        }
    }
}

fn main() {
    let mut testvec = Vec::from([8, 11, 4, 6, 9]);
    testvec.selection_sort();
    println!("{:?}",testvec);
}
