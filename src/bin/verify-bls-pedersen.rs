use bls_pedersen::bls::verify;
use bls_pedersen::data::puzzle_data;
use bls_pedersen::PUZZLE_DESCRIPTION;
use bls_pedersen::solver::solve;
use prompt::{puzzle, welcome};

use ark_bls12_381::{G1Affine};
use ark_serialize::{CanonicalDeserialize};
use std::io::Cursor;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (pk, _ms, _sigs) = puzzle_data();
   //  for (m, sig) in ms.iter().zip(sigs.iter()) {
   //      verify(pk, m, *sig);
   //  }

	 let username = "Derked".as_bytes();
	 let sig_forged = solve();
	 let final_forged = G1Affine::deserialize(&mut Cursor::new(sig_forged)).unwrap();
	 println!("{:?}", final_forged);

	 verify(pk, &username, final_forged);
	 println!("SUCCESSFULLY FORGED SIG");

 }
