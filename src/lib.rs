pub type PcSet = Vec<i8>;
pub type IcVec = [usize; 6];
pub type IVec = [usize; 12];

trait Fundamentals {
    fn invert(&self) -> Self;
    fn transpose(&self, i8) -> Self;
    fn i(&self) -> Self;
    fn t(&self, i8) -> Self;
    fn tni(&self, i8) -> Self;
    fn ixy(&self, i8, i8) -> Self;
    fn chroma(&self) -> u16;
}

impl Fundamentals for PcSet {
    /// Returns the inversion of the pitch-class set.
    fn invert(&self) -> PcSet {
        self.iter().map(|x| (12 - x) % 12).collect()
    }
    /// Returns the transposition of the pitch-class set by _n_ semitones.
    fn transpose(&self, n: i8) -> PcSet {
        self.iter().map(|x| ((x + n) % 12 + 12) % 12).collect()
    }
    /// Returns the inversion of the pitch-class set.
    fn i(&self) -> PcSet {
        self.invert()
    }
    /// Returns the transposition of the pitch-class set by _n_ semitones.
    fn t(&self, n: i8) -> PcSet {
        self.transpose(n)
    }
    /// Inverts the pitch-class set, then returns the transposition by _n_
    /// semitones.
    fn tni(&self, n: i8) -> PcSet {
        self.invert().transpose(n)
    }
    /// Returns the transposition of the pitch-class set by _y_ semitones around
    /// the axis _x_
    fn ixy(&self, x: i8, y: i8) -> PcSet {
        self.invert().transpose(x+y)
    }
    /// Returns the binary representation of the pitch-class chroma feature
    fn chroma(&self) -> u16 {
        (0..12)
            .map(|x| self.contains(&x))
            .rev()
            .fold(0 as u16, |acc, x| (acc << 1) | x as u16)
    }
}

trait SetOperations {
    fn complement(&self) -> Self;
    fn retrograde(&self) -> Self;
    fn sort(&self) -> Self;
    fn rotate(&self, usize) -> Self;
    fn zero(&self) -> Self;
    fn normal(&self) -> Self;
    fn reduced(&self) -> Self;
    fn prime(&self) -> Self;
    fn intervals(&self) -> Vec<i8>;
    fn transposition_number(&self, other: &Self) -> Option<i8>;
    fn index_number(&self, other: &Self) -> Option<i8>;
}

impl SetOperations for PcSet {
    /// Returns the complement of the pitch-class set
    fn complement(&self) -> PcSet {
        (0..12).filter(|x| !self.contains(x)).collect()
    }
    /// Returns the retrograde of the pitch-class set
    fn retrograde(&self) -> PcSet {
        self.iter().rev().cloned().collect()
    }
    /// Returns the sorted pitch-class set in ascending order
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
    /// Returns the rotation of the pitch-class set by _n_ semitones
    fn rotate(&self, n: usize) -> PcSet {
        self.iter().cycle().skip(n + 1).take(self.len()).cloned().collect()
    }
    fn zero(&self) -> PcSet {
        match self.first() {
            Some(head) => self.transpose(12 - head),
            None => self.clone(),
        }
    }
    /// Returns the normal form of the pitch-class set
    fn normal(&self) -> PcSet {
        let mut sorted = self.sort();
        sorted.dedup();

        (0..{self.len()})
            .map(|x| sorted.rotate(x))
            .fold(sorted.clone(), |x, y| {
                if x.intervals() > y.intervals() { y }
                else { x }
            })
    }
    fn reduced(&self) -> PcSet {
        self.normal().zero()
    }
    /// Returns the prime form of the pitch-class set
    fn prime(&self) -> PcSet {
        let a = self.normal().zero();
        let b = self.invert().normal().zero();
        if a.intervals() < b.intervals() { a }
        else { b }
    }
    /// Helper function that returns a vector containing the
    /// interval-classes for the first and n to last pitch-class in a
    /// pitch-class set
    fn intervals(&self) -> Vec<i8> {
        match self.first() {
            None =>vec![],
            Some(first) =>
                self.iter()
                    .rev()
                    .map(|x| ((x - first) % 12 + 12) % 12)
                    .collect(),
        }
    }
    /// Returns the transposition number of two pitch-class sets if they are
    /// related by transposition
    fn transposition_number(&self, other: &PcSet) -> Option<i8> {
        if self.len() != other.len() { return None }

        let differences: Vec<i8> =
            self.iter()
                .zip(other)
                .map(|(x, y)| ((x - y) % 12 + 12) % 12)
                .collect();

        match differences.first() {
            None => None,
            Some(&first) => {
                if differences.iter().all(|&x| x == first) { Some(first) }
                else { None }
            },
        }
    }
    /// Returns the index number of two pitch-class sets if they are
    /// related by inversion
    fn index_number(&self, other: &PcSet) -> Option<i8> {
        if self.len() != other.len() { return None }

        let sums: Vec<i8> =
            self.iter()
                .zip(other)
                .map(|(x, y)| ((x + y) % 12 + 12) % 12)
                .collect();

        match sums.first() {
            None => None,
            Some(&first) => {
                if sums.iter().all(|&x| x == first) { Some(first) }
                else { None }
            },
        }
    }
}

trait SetAnalysis {
    /// Returns the Interval-Class Vector
    fn icvec(&self) -> IcVec;
    /// Returns the Index Vector
    fn ivec(&self) -> IVec;
}

impl SetAnalysis for PcSet {
    /// Returns the interval-class vector of the pitch-class set
    fn icvec(&self) -> IcVec {
        fn f(pcset: &PcSet) -> Vec<i8> {
            match pcset.split_first() {
                Some((head, tail)) =>
                    tail
                    .iter()
                    .map(|x| match (x - head) % 12 {
                            n if n > 6 => (12 - n) % 12,
                            n => n,
                    })
                    .chain(f(&tail.to_vec()).iter().cloned())
                    .collect(),
            None => vec![],
            }
        }

        let intervals = f(self);

        [ intervals.iter().filter(|&x| *x == 1 ).count()
        , intervals.iter().filter(|&x| *x == 2 ).count()
        , intervals.iter().filter(|&x| *x == 3 ).count()
        , intervals.iter().filter(|&x| *x == 4 ).count()
        , intervals.iter().filter(|&x| *x == 5 ).count()
        , intervals.iter().filter(|&x| *x == 6 ).count()
        ]
    }
    /// Returns the interval vector of the pitch-class set
    fn ivec(&self) -> IVec {
        let intervals: Vec<i8> =
            self
            .iter()
            .flat_map(|x| self.iter().map(move |y| (y + x) % 12))
            .collect();

        [ intervals.iter().filter(|&x| *x == 0 ).count()
        , intervals.iter().filter(|&x| *x == 1 ).count()
        , intervals.iter().filter(|&x| *x == 2 ).count()
        , intervals.iter().filter(|&x| *x == 3 ).count()
        , intervals.iter().filter(|&x| *x == 4 ).count()
        , intervals.iter().filter(|&x| *x == 5 ).count()
        , intervals.iter().filter(|&x| *x == 6 ).count()
        , intervals.iter().filter(|&x| *x == 7 ).count()
        , intervals.iter().filter(|&x| *x == 8 ).count()
        , intervals.iter().filter(|&x| *x == 9 ).count()
        , intervals.iter().filter(|&x| *x == 10 ).count()
        , intervals.iter().filter(|&x| *x == 11 ).count()
        ]
    }
}

#[cfg(test)]
mod tests {
    use Fundamentals;
    use SetOperations;
    use SetAnalysis;
    use PcSet;

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
    fn chroma() {
        let x: PcSet = vec![0, 2, 4];
        assert_eq!(x.chroma(), 21);
        let y: PcSet = vec![0];
        assert_eq!(y.chroma(), 1);
    }
    #[test]
    fn complement() {
        let x: PcSet = vec![0, 1, 2, 3, 4, 5, 6, 7];
        assert_eq!(x.complement(), vec![8, 9, 10, 11]);
    }
    #[test]
    fn retrograde() {
        let x: PcSet = vec![0, 1, 2];
        assert_eq!(x.retrograde(), vec![2, 1, 0]);
    }
    #[test]
    fn sort() {
        let x: PcSet = vec![3, 1, 2];
        assert_eq!(x.sort(), vec![1, 2, 3]);
    }
    #[test]
    fn rotate() {
        let x: PcSet = vec![1, 2, 3];
        assert_eq!(x.rotate(1), vec![3, 1, 2]);
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
    fn icvec() {
        let v: PcSet = vec![0, 2, 4, 5, 7, 9, 11];
        assert_eq!(v.icvec(), [2,5,4,3,6,1]);
    }
    #[test]
    fn ivec() {
        let x: PcSet = vec![8, 9, 0];
        assert_eq!(x.ivec(), [1,0,0,0,1,2,1,0,2,2,0,0]);
        let y: PcSet = vec![0, 3, 4];
        assert_eq!(y.ivec(), [1,0,0,2,2,0,1,2,1,0,0,0]);
    }
    #[test]
    fn intervals() {
        let x: PcSet = vec![4, 9, 3];
        assert_eq!(x.intervals(), vec![11, 5, 0]);
    }
    #[test]
    fn index_number() {
        let x: PcSet = vec![7, 8, 11];
        let y: PcSet = vec![5, 4, 1];
        assert_eq!(x.index_number(&y), Some(0));
        assert_eq!(x.tni(0), y); //
        let z: PcSet = vec![11, 10, 7];
        assert_eq!(x.index_number(&z), Some(6));
        assert_eq!(x.tni(6), z);
    }
    #[test]
    fn transposition_number() {
        let x: PcSet = vec![1, 3, 4, 7];
        let y: PcSet = vec![5, 7, 8, 11];
        assert_eq!(x.transposition_number(&y), Some(8));
        assert_eq!(x.transpose(8), y);
    }
}
