type PcSet = Vec<i8>;

trait Fundamentals {
    fn invert(&self) -> Self;
    fn transpose(&self, i8) -> Self;
}

impl Fundamentals for PcSet {
    fn invert(&self) -> PcSet {
        self.iter().map(|x| (12 - x) % 12).collect()
    }
    fn transpose(&self, n: i8) -> PcSet {
        self.iter()
            .map(|x| {
                match (x + n) % 12 {
                    r if r >= 0 => r,
                    r => r + 12,
                }
            })
            .collect()
    }
}

trait SetOperations {
    fn complement(&self) -> Self;
    fn reverse(&self) -> Self;
    fn sort(&self) -> Self;
    // fn shift(&self, i8) -> Self;
    fn zero(&self) -> Self;
    // fn normal(&self) -> Self;
    // fn reduced(&self) -> Self;
    // fn prime(&self) -> Self;
}

impl SetOperations for PcSet {
    fn complement(&self) -> PcSet {
        (0..12).filter(|x| !self.contains(x)).collect()
    }
    fn reverse(&self) -> PcSet {
        self.iter().fold(vec![] as PcSet, |acc, &x| [vec![x], acc].concat())
    }
    fn sort(&self) -> PcSet {
        let mut clone = self.clone();
        let mut temp: i8;
        for i in 0..clone.len() {
            temp = clone[i];
            let mut j = i;
            while j > 0 && clone[j - 1] > temp {
                clone[j] = clone[j - 1];
                j = j - 1;
            }
            clone[j] = temp;
        }
        clone
    }
    fn zero(&self) -> PcSet {
        match self.iter().min() {
            Some(min) => self.iter().map(|x| x - min).collect(),
            None => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use Fundamentals;
    use SetOperations;
    use PcSet;

    #[test]
    fn invert() {
        let x: PcSet = vec![1, 2, 3];
        assert_eq!(x.invert(), vec![11, 10, 9]);
    }

    #[test]
    fn transpose() {
        let x: PcSet = vec![1, 2, 3];
        assert_eq!(x.transpose(-14), vec![11, 0, 1]);
    }

    #[test]
    fn complement() {
        let x: PcSet = vec![0, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(x.complement(), vec![8, 9, 10, 11]);
    }

    #[test]
    fn reverse() {
        let x: PcSet = vec![0, 1, 2];
        assert_eq!(x.reverse(), vec![2, 1, 0]);
    }

    #[test]
    fn sort() {
        let x: PcSet = vec![3, 1, 2];
        assert_eq!(x.sort(), vec![1, 2, 3]);
    }

    #[test]
    fn zero() {
        let x: PcSet = vec![1, 2, 3];
        assert_eq!(x.zero(), vec![0, 1, 2]);
    }
}
