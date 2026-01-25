pub trait GenericId {
    type Distance;

    fn distance<T>(&self, to: &T) -> Self::Distance
    where
        T: AsRef<[u8; 32]>;

    fn bucket_for<T>(&self, other: &T) -> Option<usize>
    where
        T: AsRef<[u8; 32]>;
}

impl<T> GenericId for T
where T: AsRef<[u8; 32]> {
    type Distance = [u8; 32];

    fn distance<O>(&self, to: &O) -> [u8; 32]
    where O: AsRef<[u8; 32]> {
        let a = self.as_ref();
        let b = to.as_ref();
        let mut out = [0u8; 32];
        for i in 0..32 {
            out[i] = a[i] ^ b[i];
        }
        out
    }
    
    fn bucket_for<O>(&self, other: &O) -> Option<usize>
    where O: AsRef<[u8; 32]> {
        let dist = self.distance(other);
        for (i, byte) in dist.iter().enumerate() {
            if *byte != 0 {
                return Some((i * 8) + (7 - byte.leading_zeros() as usize));
            }
        }
        None
    }
}