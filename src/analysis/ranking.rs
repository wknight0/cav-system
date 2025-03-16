use std::cmp::Ordering;

use crate::schema::{cve::CVE, rank::RankedCVE};

//TEMPORARY RANKING FUNCTION TO JUST ADD C, I, and A VALUES TOGETHER FOR SCORE AND SORT BY SCORE
pub fn rank_cves(cves: Vec<CVE>) -> Vec<RankedCVE> {
    let mut ranked_cves: Vec<RankedCVE> = Vec::new();

    for cve in cves {
        let cia_score = cve.retrieve_values();
        let score = cia_score.c + cia_score.i + cia_score.a;
        ranked_cves.push(RankedCVE {
            cve: cve,
            score,
        });
    }

    let sort_function = | a: &RankedCVE, b: &RankedCVE | -> Ordering {
        if b.score > a.score {
            return Ordering::Greater;
        }

        if b.score < a.score {
            return Ordering::Less;
        }

        return Ordering::Equal;
    };

    ranked_cves.sort_by(sort_function);
    ranked_cves
}
