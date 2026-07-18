use emittio_crypto::id::Id;

use crate::{peer::PeerId, reply::Replyable};

/// Contains replies and ids of peers who sent them
pub type VerificationInput<R> = Vec<(PeerId, R)>;

/// Contains results of verifications on replies and the most accurate reply
pub type VerificationOutput<R> = (Vec<(PeerId, bool)>, Option<R>);

/// Verifies something `Replyable`
pub trait Verifier<R: Replyable> {
    /// Note: order of replies in `VerificationInput` is not necessarily will the same as in the `VerificationOutput`
    fn verify(&self, replies: VerificationInput<R>) -> VerificationOutput<R>;
}

/// It's useful when there is no way to verify the reply (is empty for example)
pub struct NoVerifier;

impl<R: Replyable> Verifier<R> for NoVerifier {
    fn verify(&self, replies: VerificationInput<R>) -> VerificationOutput<R> {
        let (ids, replies): (Vec<_>, Vec<_>) = replies.into_iter().unzip();
        (ids.iter().map(|id| (*id, true)).collect(), replies.into_iter().next())
    }
}

/// Verifies reply content comparing content's hash id to the internal id.
/// Useful in content-addressed systems like DHT
pub struct HashVerifier(pub Id);

impl<R: AsRef<[u8]> + Replyable> Verifier<R> for HashVerifier {
    fn verify(&self, replies: VerificationInput<R>) -> VerificationOutput<R> {
        let mut final_reply = None;
        let results: Vec<(Id, bool)> = replies.into_iter().map(|(id, reply)| {
            if Id::hash_bytes(reply.as_ref()) == self.0 {
                final_reply = Some(reply);
                (id, true)
            } else {
                (id, false)
            }
        }).collect();

        (results, final_reply)
    }
}

/// It's usually useful when the reply is a number and you need to select the middle value of the replies.
pub struct MedianVerifier {
    /// Allowable deviation as a fraction in range [0.0, 1.0]
    pub tolerance: f64,
}

impl Verifier<u64> for MedianVerifier {
    fn verify(&self, mut replies: VerificationInput<u64>) -> VerificationOutput<u64> {
        if replies.len() == 0 {
            return (Vec::new(), None);
        }

        replies.sort_by(|(_, a), (_, b)| a.cmp(b));

        let median = replies[replies.len() / 2].1;
        let results = replies.into_iter().map(|(id, reply)| {
            if median == 0 {
                (id, reply == 0)
            } else {
                let relative = reply.abs_diff(median) as f64 / median as f64;
                (id, relative <= self.tolerance)
            }
        }).collect();

        (results, Some(median))
    }
}

#[cfg(test)]
mod tests {

    use bytes::Bytes;

    use super::*;

    fn replies_from_array<R>(replies: Vec<R>) -> VerificationInput<R> {
        let mut out = Vec::new();

        for (i, reply) in replies.into_iter().enumerate() {
            out.push((Id::hash_bytes(&i.to_be_bytes()), reply));
        }
        
        out
    }

    #[test]
    fn test_hash_verifier() {
        let right = Bytes::from_static(b"right reply");
        let wrong = Bytes::from_static(b"foobarbaz");
        let verifier = HashVerifier(Id::hash_bytes(&right));

        // 1. Empty replies
        assert_eq!(verifier.verify(vec![]), (vec![], None::<Bytes>));
        
        // 2. Single right reply
        assert_eq!(verifier.verify(vec![(Id::default(), right.clone())]), (vec![(Id::default(), true)], Some(right.clone())));
        
        // 3. Single wrong reply
        assert_eq!(verifier.verify(vec![(Id::default(), wrong.clone())]), (vec![(Id::default(), false)], None));
        
        // 4. Multiple replies
        let mut replies: Vec<_> = (0..5)
            .map(|i| (Id::new([i; 32]), right.clone()))
            .collect();

        for i in 0..5 {
            let out = verifier.verify(replies.clone());

            assert!(out.0.into_iter().enumerate().all(|(j, (_, r))| if j >= i { r } else { !r }));
            assert_eq!(out.1, if i == 5 { None } else { Some(right.clone()) });

            if i == 5 { break; }
            replies[i].1 = wrong.clone();
        }
    }

    #[test]
    fn test_median_verifier() {
        let verifier = MedianVerifier { tolerance: 0.05 };

        // 1. Empty replies
        assert_eq!(verifier.verify(vec![]), (vec![], None::<u64>));
        
        // 2. Single reply
        assert_eq!(verifier.verify(vec![(Id::default(), 1)]), (vec![(Id::default(), true)], Some(1)));
        assert_eq!(verifier.verify(vec![(Id::default(), 0)]), (vec![(Id::default(), true)], Some(0)));
        
        // 3. Multiple replies
        let median = 100;
        let mut deviation = 0.00;

        loop {
            let lower = (median as f64 * (1.0 - deviation)) as u64;
            let upper = (median as f64 * (1.0 + deviation)) as u64;

            let out = verifier.verify(replies_from_array(vec![median, lower, upper, 200, 0]));
            
            for (id, result) in out.0 {
                match id {
                    // Median
                    id if id == Id::hash_bytes(&0u64.to_be_bytes()) => assert!(result),
                    // Lower
                    id if id == Id::hash_bytes(&1u64.to_be_bytes()) => {
                        if deviation <= 0.05 { assert!(result) } else { assert!(!result) }
                    }
                    // Upper
                    id if id == Id::hash_bytes(&2u64.to_be_bytes()) => {
                        if deviation <= 0.05 { assert!(result) } else { assert!(!result) }
                    }
                    // 200
                    id if id == Id::hash_bytes(&3u64.to_be_bytes()) => assert!(!result),
                    // 0
                    id if id == Id::hash_bytes(&4u64.to_be_bytes()) => assert!(!result),
                    _ => unreachable!()
                }
            }

            assert_eq!(out.1, Some(median));

            if deviation >= 0.10 { break; }
            deviation += 0.01;
        }
    }
}