use std::ops;
use serde::{Deserialize, Serialize};

use crate::schema::cve::CVE;

// CIAScore struct which holds confidentiality, integrity, and availability values for each asset affected in the network
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CIAScore {
    pub c: f32,
    pub i: f32,
    pub a: f32,
}

impl CIAScore {
    // Constructor for CIAScore struct
    pub fn new(c: f32, i: f32, a: f32) -> CIAScore {
        CIAScore { c, i, a }
    }

    // Returns maximum of the two numbers for CIA values
    pub fn max(&self, rhs: CIAScore) -> CIAScore {
        CIAScore {
            c: self.c.max(rhs.c),
            i: self.i.max(rhs.i),
            a: self.a.max(rhs.a),
        }
    }
}

// Multiplication implementation for CIAScore
impl ops::Mul<CIAScore> for CIAScore {
    type Output = CIAScore;

    fn mul(self, rhs: CIAScore) -> Self::Output {
        CIAScore::new(self.c * rhs.c, self.i * rhs.i, self.a * rhs.a)
    }
}

// Addition implementation for CIAScore
impl ops::Add<CIAScore> for CIAScore {
    type Output = CIAScore;

    fn add(self, rhs: CIAScore) -> Self::Output {
        CIAScore::new(self.c + rhs.c, self.i + rhs.i, self.a + rhs.a)
    }
}

// Adding assignment implementation for CIAScore
impl ops::AddAssign<CIAScore> for CIAScore {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            c: self.c + other.c,
            i: self.i + other.i,
            a: self.a + other.a,
        };
    }
}

// Partial equivalence implementation for CIAScore
impl PartialEq<CIAScore> for CIAScore {
    fn eq(&self, other: &CIAScore) -> bool {
        self.c == other.c && self.i == other.i && self.a == other.a
    }
}

// RankedCVE struct of CVE and score based on context of CVE and Asset context in network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedCVE {
    pub score: f32,
    pub cve: CVE,
}

impl RankedCVE {
    pub fn new(score: f32, cve: CVE) -> RankedCVE {
        RankedCVE { score, cve }
    }
}
