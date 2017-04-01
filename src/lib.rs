fn decimalvalue(pcset: &PcSet) -> Option<i64> {
    match pcset.first() {
        None => None,
        Some(head) => {
            let decimalvalue = pcset
                .iter()
                .skip(1)
                .map(|x| (((x - head) % 12) + 12) % 12)
                .enumerate()
                .fold(0, |acc, (i, e)| acc + e as i64 * 10i64.pow(i as u32));
            Some(decimalvalue)
        }
    }
}

fn packed(x: PcSet, y: PcSet) -> PcSet {
    match (decimalvalue(&x), decimalvalue(&y)){
        (_, None) => x, // not sure what to do about (None, None)
        (None, _) => y,
        (Some(a), Some(b)) => {
            if a < b { x }
            else if a > b { y }
            else {
                if x.first() < y.first() { x }
                else { y }
            }
        },
    }
}

type PcSet = Vec<i8>;
type IVec = [i8; 6];
type CVec = [i8; 12];

trait Fundamentals {
    fn invert(&self) -> Self;
    fn transpose(&self, i8) -> Self;
    fn i(&self) -> Self;
    fn t(&self, i8) -> Self;
    fn tni(&self, i8) -> Self;
    fn ixy(&self, i8, i8) -> Self;
}

impl Fundamentals for PcSet {
    fn invert(&self) -> PcSet {
        self.iter().map(|x| (12 - x) % 12).collect()
    }
    fn transpose(&self, n: i8) -> PcSet {
        self.iter().map(|x| (((x + n) % 12) + 12) % 12).collect()
    }
    fn i(&self) -> PcSet {
        self.invert()
    }
    fn t(&self, n: i8) -> PcSet {
        self.transpose(n)
    }
    fn tni(&self, n: i8) -> PcSet {
        self.invert().transpose(n)
    }
    fn ixy(&self, x: i8, y: i8) -> PcSet {
        self.invert().transpose(x+y)
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
    fn prime(&self) -> Self;
}

impl SetOperations for PcSet {
    fn complement(&self) -> PcSet {
        (0..12).filter(|x| !self.contains(x)).collect()
    }
    fn reverse(&self) -> PcSet {
        self.iter().rev().cloned().collect()
    }
    /// Implementation of the insertion sort algorithm for performant sorting of
    /// (quite) small data sets.
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
        match self.first() {
            Some(head) => self.transpose(-head),
            None => self.clone(),
        }
    }
    fn normal(&self) -> PcSet {
        let mut sorted = self.sort();
        sorted.dedup();

        (0..self.len())
            .map(|x| sorted.shift(x))
            .fold(vec![], |x, y| packed(x, y))
    }
    fn reduced(&self) -> PcSet {
        self.normal().zero()
    }
    fn prime(&self) -> PcSet {
        let original = self.normal().zero();
        let inverted = self.invert().normal().zero();
        if decimalvalue(&original) < decimalvalue(&inverted) { original }
        else { inverted }
    }
}

trait SetAnalysis {
    fn ivec(&self) -> IVec;
    fn cvec(&self) -> CVec;
}

impl SetAnalysis for PcSet {
    fn ivec(&self) -> IVec {
        match self.split_first() {
            Some((head, tail)) =>
            {
                let is: Vec<i8> = tail.iter().map(|x| (x - head) % 6).collect();

                let mut temp = [0i8; 6];

                for i in 0..6 {
                    temp[i] = is
                                .iter()
                                .filter(|&x| *x == i as i8 + 1)
                                .count() as i8
                }
                temp
            },
            None => [1,2,3,4,5,6],
        }
    }
    /// Dummy data
    fn cvec(&self) -> CVec {
        [1,2,3,4,5,6,7,8,9,10,11,12]
    }
}
#[cfg(test)]
mod tests {
    use Fundamentals;
    use SetOperations;
    use SetAnalysis;
    use PcSet;
    use IVec;
    use CVec;

    #[test]
    fn invert() {
        let w: PcSet = vec![0, 2, 4, 8];
        assert_eq!(w.invert(), vec![0, 10, 8, 4]);
        let x: PcSet = vec![1, 2, 3];
        assert_eq!(x.invert(), vec![11, 10, 9]);
        let y: PcSet = vec![0, 4, 6, 8];
        assert_eq!(y.invert(), vec![0, 8, 6, 4]);
        let z: PcSet = vec![8, 0, 4, 6];
        assert_eq!(z.invert(), vec![4, 0, 8, 6]);
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
        let y: PcSet = vec![0, 1, 2];
        assert_eq!(y.zero(), vec![0, 1, 2]);
    }
    #[test]
    fn normal() {
        let x: PcSet = vec![8, 0, 4, 6];
        assert_eq!(x.normal(), vec![4, 6, 8, 0]);
        let y: PcSet = vec![2, 1, 3, 7, 6];
        assert_eq!(y.normal(), vec![1, 2, 3, 6, 7]);
        assert_eq!(x, vec![8, 0, 4, 6]);
        assert_eq!(y, vec![2, 1, 3, 7, 6]);
    }
    #[test]
    fn reduced() {
        let x: PcSet = vec![2, 1, 3, 7, 6];
        assert_eq!(x.reduced(), vec![0, 1, 2, 5, 6]);
    }
    #[test]
    fn prime() {
        let v: PcSet = vec![0, 4, 6, 8];
        assert_eq!(v.prime(), vec![0, 2, 4, 8]);
        let w: PcSet = vec![8, 0, 4, 6];
        assert_eq!(w.prime(), vec![0, 2, 4, 8]);
        let x: PcSet = vec![1, 5, 6, 7];
        assert_eq!(x.prime(), vec![0, 1, 2, 6]);
        let y: PcSet = vec![3, 4, 5];
        assert_eq!(y.prime(), vec![0, 1, 2]);
        let z: PcSet = vec![2, 4, 8, 9];
        assert_eq!(z.prime(), vec![0, 1, 5, 7]);
    }
    #[test]
    fn ivec() {
        let v: PcSet = vec![1,2,3];
        assert_eq!(v.ivec(), [0,4,5,6,3,5]);
    }
}
