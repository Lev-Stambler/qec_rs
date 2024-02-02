pub struct LDPCCode {
    N: usize,
    M: usize,
    parity_checks: Vec<usize>,
    dc: usize,
    dv: usize,
}

impl LDPCCode {
    fn from_alist(alist_path: &str) -> Result<Self, std::io::Error> {
        // TODO: proper error handling
        let alist = std::fs::read_to_string(alist_path).unwrap();
        let mut lines = alist.lines();
        let mut header = lines.next().unwrap().split_whitespace();
        let N = header.next().unwrap().parse().unwrap();
        let M = header.next().unwrap().parse().unwrap();
        let degs = lines.next().unwrap().parse().unwrap().split_whitespace();
        let dv = degs.next().unwrap().parse().unwrap();
        let dc = degs.next().unwrap().parse().unwrap();

        let _pc_of_v = lines.next().unwrap();
        let _pc_of_c = degs.next().unwrap();

        let parity_checks = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse().unwrap() - 1)
                    .collect::<Vec<usize>>()
            })
            .collect();
        let dc = parity_checks.len();
        let dv = parity_checks[0].len();
        Ok(Self {
            N,
            M,
            parity_checks,
            dc,
            dv,
        })
    }
}
