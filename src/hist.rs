use ahash::AHasher;
use core::hash::BuildHasherDefault;
use dashmap::DashMap;
use spec_math::Gamma;

type PhraseMap = DashMap<pfp::hash::HT, u64, BuildHasherDefault<AHasher>>;

pub struct PhraseFreqMap {
    pub phrase_map: PhraseMap,
    pub tot_refs: usize,
}

impl PhraseFreqMap {
    pub fn new() -> Self {
        Self {
            phrase_map: PhraseMap::with_capacity_and_hasher(
                1_000_000,
                BuildHasherDefault::<AHasher>::default(),
            ),
            tot_refs: 0,
        }
    }

    pub fn add_parse(&mut self, p: &[pfp::hash::HT]) {
        // keep just the distinct elements from this parse
        let mut seen_phrases =
            hashbrown::HashSet::<pfp::hash::HT, BuildHasherDefault<AHasher>>::with_hasher(
                BuildHasherDefault::<AHasher>::default(),
            );
        for phrase in p {
            if seen_phrases.insert(*phrase) {
                *self.phrase_map.entry(*phrase).or_insert(0) += 1;
            }
        }
        self.tot_refs += 1;
    }

    pub fn get_hist(&self) -> PhraseHist {
        let mut freqs = vec![0u64; self.tot_refs + 1];
        println!("{}", self.phrase_map.len());
        for kv in self.phrase_map.iter() {
            freqs[*kv.value() as usize] += 1;
        }
        PhraseHist {
            freqs,
            tot_refs: self.tot_refs,
        }
    }
}

#[derive(Debug)]
pub struct PhraseHist {
    pub freqs: Vec<u64>,
    pub tot_refs: usize,
}

pub fn falling_fact(n: u64, i: u64, m: u64) -> f64 {
    if m > n {
        0f64
    } else if i == n {
        1f64
    } else {
        let x = n - i;
        let num = (((x) + 1) as f64).lgamma();
        let denom = ((x - m + 1) as f64).lgamma();
        (num - denom).exp()
    }
}

impl PhraseHist {
    fn ftot(&self, m: u64) -> f64 {
        let mut ftot_m = 0f64;
        for i in 1..self.tot_refs {
            let n = self.tot_refs as u64;
            ftot_m += (self.freqs[i] as f64)
                * (1. - falling_fact(n, i as u64, m) / falling_fact(n, 0, m));
        }
        ftot_m
    }

    pub fn compute_fnew_vec(&self) -> Vec<f64> {
        let mut fnew = vec![0f64; self.freqs.len()];
        let mut ftot_m1 = 0f64;
        for (m, n) in self.freqs.iter().enumerate().skip(1) {
            let ftot_m = self.ftot(m as u64);
            fnew[m] = ftot_m - ftot_m1;
            ftot_m1 = ftot_m;
        }
        fnew
    }
}
