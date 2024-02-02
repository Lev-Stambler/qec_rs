use nalgebra_sparse::{csr::CsrMatrix};

pub struct LDPCCode {
    pub N: usize,
    pub M: usize,
    pub H: CsrMatrix<u8>,
    pub parity_nghbs: Vec<Vec<usize>>,
    pub bit_nghbs: Vec<Vec<usize>>,
    pub dc: usize,
    pub dv: usize,
}

impl LDPCCode {
    fn from_alist(alist_path: &str) -> Result<Self, std::io::Error> {
        // TODO: proper error handling
        let alist = std::fs::read_to_string(alist_path).unwrap();
        let mut lines = alist.lines();
        let mut header = lines.next().unwrap().split_whitespace();
        let N = header.next().unwrap().parse().unwrap();
        let M = header.next().unwrap().parse().unwrap();
        let mut degs = lines.next().unwrap().split_whitespace();
        let dv: usize = degs.next().unwrap().parse().unwrap();
        let dc: usize = degs.next().unwrap().parse().unwrap();

        let _pc_of_v = lines.next().unwrap();
        let _pc_of_c = degs.next().unwrap();

        let parity_checks: Vec<Vec<usize>> = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap() - 1)
                    .collect::<Vec<usize>>()
            })
            .collect();
        let dc = parity_checks.len();
        let dv = parity_checks[0].len();
        Ok(Self {
            N,
            M,
            parity_nghbs: parity_checks,
            bit_nghbs: todo!(),
            H: todo!(),
            dc,
            dv,
        })
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_from_alist() {
        todo!("implement test")
    }
}