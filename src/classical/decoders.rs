use nalgebra::{DMatrix, DVector};
use nalgebra_sparse::csr::CsrMatrix;
use nalgebra_sparse::ops::{serial::spmm_csr_dense, Op};
use rayon::prelude::*;
use std::{f32::consts::E, intrinsics::log2f64, ops::Mul};

use super::code::LDPCCode;

enum DecodeErrors {
    ImproperParams(String),
}

/// A list of 0 and 1s that represent the bit flips that need to be made to correct the error per index
type FlipErrors = Vec<u8>;

trait DecoderInternal {
    fn _decode(
        &self,
        code: LDPCCode,
        bit_probs: Vec<f64>,
        syndrome: Vec<bool>,
    ) -> Result<FlipErrors, DecodeErrors>;
}

pub trait Decoder: DecoderInternal {
    fn decode(
        &self,
        code: LDPCCode,
        bit_probs: Vec<f64>,
        syndrome: Vec<bool>,
    ) -> Result<FlipErrors, DecodeErrors> {
        if code.N != bit_probs.len() {
            return Err(DecodeErrors::ImproperParams(
                "bit_probs length does not match code length".to_string(),
            ));
        }
        if code.M != syndrome.len() {
            return Err(DecodeErrors::ImproperParams(
                "syndrome length does not match code length".to_string(),
            ));
        }
        self._decode(code, bit_probs, syndrome)
    }
}

/// Min-Sum Belief Propagation Decoder
/// Here we follow the pseudo-code primarily from page 14 of [this QEC paper](https://arxiv.org/pdf/2005.07016.pdf).
struct DecoderMinSumBP {
    max_iter: u32,
    // damping: f64,
}

impl DecoderInternal for DecoderMinSumBP {
    fn _decode(
        &self,
        code: LDPCCode,
        bit_probs: Vec<f64>,
        syndrome: Vec<bool>,
    ) -> Result<FlipErrors, DecodeErrors> {
        // TODO: ln?
        let syndrome_dvec = DVector::from_vec(syndrome.iter().map(|x| *x as u8).collect());
        let mut channel_llr: Vec<f64> = bit_probs
            .par_iter()
            .map(|p| f64::log2(1.0 - p) - p.log2())
            .collect();
        let mut data_to_parity = code
            .bit_nghbs
            .par_iter()
            .enumerate()
            .map(|(i, nghbs)| vec![channel_llr[i]; nghbs.len()])
            .collect::<Vec<Vec<f64>>>();

        let mut parity_to_data = code
            .parity_nghbs
            .par_iter()
            .enumerate()
            .map(|(i, nghbs)| vec![0.0; nghbs.len()])
            .collect::<Vec<Vec<f64>>>();

        for iter in 0..self.max_iter {
            // Scaling factor
            let scaling = 1.0 - f64::powi(2.0, -1 * iter as i32);

            // Parity to Data Msgs
            code.bit_nghbs
                .par_iter()
                .enumerate()
                .for_each(|(u_i, nghbs)| {
                    // We will not use par_iter bellow as we are assuming LDPC codes
                    // TODO: func doc for this
                    for (j, v) in nghbs.iter().enumerate() {
                        let mut min_not_j = f64::INFINITY;
                        for (k, w) in nghbs.iter().enumerate() {
                            if k != j {
                                min_not_j = f64::min(min_not_j, data_to_parity[*v][k].abs());
                            }
                        }
                        let sign_prods = data_to_parity[*v]
                            .iter()
                            .enumerate()
                            .map(|(i, x)| {
                                if x.is_sign_positive() || i == j {
                                    // the || i == j condition filters out for v_j
                                    1.0
                                } else {
                                    -1.0
                                }
                            })
                            .collect::<Vec<f64>>();
                        let syndrom_mult = if syndrome[u_i] { -1.0 } else { 1.0 };
                        parity_to_data[u_i][j] =
                            syndrom_mult * scaling * min_not_j * sign_prods.iter().product::<f64>();
                    }
                });
            // Data to Parity Msgs
            code.parity_nghbs
                .par_iter()
                .enumerate()
                .for_each(|(v_i, nghbs)| {
                    // TODO: we can build up the sum bellow outside of the bellow iter and then just subtract
                    for (j, u) in nghbs.iter().enumerate() {
                        let mut sum = 0.0;
                        for (k, u_prime) in nghbs.iter().enumerate() {
                            if k != j {
                                // TODO: we should build this lookup table once and use it here
                                let backwards_idx = code.bit_nghbs[*u_prime]
                                    .iter()
                                    .position(|&x| x == v_i)
                                    .unwrap();
                                sum += parity_to_data[*u_prime][backwards_idx];
                            }
                        }
                        // TODO: is the bellow right?
                        data_to_parity[v_i][j] = data_to_parity[v_i][j] + sum;
                    }
                });

            // Perform the hard decision
            let e_bp = code
                .bit_nghbs
                .par_iter()
                .enumerate()
                .map(|(j, nghbs)| {
                    let mut sum = channel_llr[j];
                    for (i, u) in nghbs.iter().enumerate() {
                        sum += parity_to_data[*u][i];
                    }
                    // TODO: are these signs correct? -1 is 1 and 1 is 0
                    if sum.is_sign_positive() {
                        1
                    } else {
                        0
                    }
                })
                .collect::<Vec<u8>>();
            // TODO: into vector thing and sparse vector
            let dvec_e = DMatrix::from_vec(1, e_bp.len(), e_bp);
            let mut y: DVector<_> = DVector::zeros(syndrome.len());
            // Perform the mat mult
            spmm_csr_dense(0, &mut y, 1, Op::NoOp(&code.H), Op::NoOp(&dvec_e));
            y = y.map(|x| x % 2);
            // Check if y == syndrome, if so we are done
            // TODO: what
            if y == syndrome_dvec {
                return Ok(e_bp);
            }
        }
        // Check for termination

        // let mut messages
        // Initialize the messages

        todo!()
    }
}
