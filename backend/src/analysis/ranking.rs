use std::cmp::Ordering;

use crate::schema::{cve::CVE, asset::Asset, rank::RankedCVE};

//TEMPORARY RANKING FUNCTION TO JUST ADD C, I, and A VALUES TOGETHER FOR SCORE AND SORT BY SCORE
pub fn rank_cves(cves: Vec<CVE>, _assets: Vec<Asset>) -> Vec<RankedCVE> {
    let mut ranked_cves: Vec<RankedCVE> = Vec::new();
    let max_score = 1.98; // Maximum possible score for C, I, and A (0.66 + 0.66 + 0.66)

    for cve in cves {
        let cia_score = cve.retrieve_values();
        let raw_score = (cia_score.c + cia_score.i + cia_score.a) / max_score;
        let score = raw_score * 100.0;

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
