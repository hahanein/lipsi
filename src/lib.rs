// TODO make less ugly or maybe throw an error on None
fn decimalvalue(pcset: &PcSet) -> i64 {
    match pcset.first() {
        None => 0,
        Some(head) =>
            pcset
                .iter()
                .skip(1)
                .map(|&x| match (x - head) % 12 {
                    r if r >= 0 => r,
                    r => r + 12,
                })
                .enumerate()
                .fold(0, |acc, (i, e)| acc + e as i64 * 10i64.pow(i as u32))
    }
}



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
    fn shift(&self, usize) -> Self;
    fn zero(&self) -> Self;
    fn normal(&self) -> Self;
    fn reduced(&self) -> Self;
    // fn prime(&self) -> Self;
}

impl SetOperations for PcSet {
    fn complement(&self) -> PcSet {
        (0..12).filter(|x| !self.contains(x)).collect()
    }
    fn reverse(&self) -> PcSet {
        self.iter().fold(vec![], |acc, &x| [vec![x], acc].concat())
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
        return clone;
    }
    fn shift(&self, n: usize) -> PcSet {
        self.iter().cycle().skip(n + 1).take(self.len()).cloned().collect()
    }
    fn zero(&self) -> PcSet {
        match self.iter().min() {
            Some(min) => self.iter().map(|x| x - min).collect(),
            None => self.clone(),
        }
    }
    fn normal(&self) -> PcSet {
        fn packed(x: PcSet, y: PcSet) -> PcSet {
            let dec_x = decimalvalue(&x);
            let dec_y = decimalvalue(&y);

            if dec_x < dec_y { x }
            else if dec_x > dec_y { y }
            else {
                if x.first() < y.first() { x }
                else { y }
            }
        }

        let sorted = self.sort();
        let normal = (0..self.len())
            .map(|x| sorted.shift(x))
            .fold(None, |x, y| {
                match x {
                    None => Some(y),
                    Some(x) => Some(packed(x, y)),
                }
            });

        match normal {
            None => self.clone(),
            Some(x) => x,
        }
    }
    fn reduced(&self) -> PcSet {
        self.normal().zero()
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
    fn shift() {
        let x: PcSet = vec![1, 2, 3];
        assert_eq!(x.shift(1), vec![3, 1, 2]);
        assert_eq!(x, vec![1, 2, 3]);
    }

    #[test]
    fn zero() {
        let x: PcSet = vec![1, 2, 3];
        assert_eq!(x.zero(), vec![0, 1, 2]);
    }

    #[test]
    fn normal() {
        let x: PcSet = vec![8, 0, 4, 6];
        assert_eq!(x.normal(), vec![4, 6, 8, 0]);
        let y: PcSet = vec![2, 1, 3, 7, 6];
        assert_eq!(y.normal(), vec![1, 2, 3, 6, 7]);
    }

    #[test]
    fn reduced() {
        let x: PcSet = vec![2, 1, 3, 7, 6];
        assert_eq!(x.reduced(), vec![0, 1, 2, 5, 6]);
    }
}
