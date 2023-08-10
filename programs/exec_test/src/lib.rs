use anchor_lang::prelude::*;
use ark_bn254::Config;
use ark_ff::{BigInteger, One};
use ark_ff::{BigInteger256, MontConfig};
use solana_program::poseidon::{hash, hashv};
declare_id!("Esqmi51Gc2EGie1b3uGqZyqY86rakzCLk1gv4D9x2Lwp");

pub struct PublicInput {
    pub input1: [[u8; 32]; 3],
}
use ark_bn254::{Fr, FrConfig};
use ark_ff::PrimeField;
#[program]
pub mod exec_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world");
        // execute_poseidon()?;
        // update_merkle_tree()?;
        let value = [vec![0u8; 31], vec![1u8]].concat();
        let hash_invalid_input = solana_program::poseidon::hash([1u8; 128].as_slice());
        let hash1 = solana_program::poseidon::hash([1u8; 1].as_slice());

        let hash32 = solana_program::poseidon::hash([1u8; 32].as_slice());

        msg!("here");
        let greater_than_field_size = Fr::from(FrConfig::MODULUS) + Fr::one();
        msg!("greater_than_field_size");

        let greater_than_field_size = greater_than_field_size.into_bigint().to_bytes_be();
        msg!(
            "greater_than_field_size bytes {:?}",
            greater_than_field_size
        );

        // msg!("hash_invalid_input {:?}", hash_invalid_input.unwrap().0);
        // msg!("hash1 {:?}", hash1.unwrap().0);
        // msg!("hash32 {:?}", hash32.unwrap().0);
        let hash = hash(greater_than_field_size.as_slice());

        // assert!(hash_invalid_input.is_err());
        panic!();
        // let hash = hash([1; 1].as_slice());
        // assert!(hash.is_err());
        // let hash = hashv(&[[&[0u8]]]).unwrap();

        Ok(())
    }
}

fn execute_poseidon() -> Result<()> {
    msg!("execute_poseidon");
    for _ in 0..200 {
        let mut inputs = Vec::new();
        let value = [vec![0u8; 31], vec![1u8]].concat();
        // inputs.push(value.as_slice());
        // Fr::from_be_bytes_mod_order(inputs[0]);
        // let hash = hashv(&[&value[..]]).unwrap();
        // msg!("here");
        for i in 2..3 {
            // msg!("i {}", i);
            inputs.push(value.as_slice());
            let hash = hashv(inputs.as_slice()).unwrap();
            // assert_eq!(TEST_CASES[i - 1], hash.to_bytes());
        }
        // inputs.push(value.as_slice());
        // let hash = hashv(inputs.as_slice());
        // // msg!("{:?}", hash.0);
        // assert!(hash.is_err());
    }

    Ok(())
}

//
// use light_merkle_tree::{Hash, Hasher};

// #[derive(Clone, Copy)] // To allow using with zero copy Solana accounts.
// pub struct Poseidon;

// impl Hasher for Poseidon {
//     fn hash(val: &[u8]) -> Hash {
//         unimplemented!()
//     }

//     fn hashv(vals: &[&[u8]]) -> Hash {
//         hashv(vals).to_bytes()
//     }
// }
// pub(crate) struct Sha256MerkleTreeConfig;
// impl config::MerkleTreeConfig for Sha256MerkleTreeConfig {
//     const ZERO_BYTES: constants::ZeroBytes = constants::poseidon::ZERO_BYTES;
//     const PROGRAM_ID: Pubkey = Pubkey::new_from_array([0u8; 32]);
// }
// fn update_merkle_tree() -> Result<()> {

//         let mut merkle_tree = MerkleTree::<
//             light_merkle_tree::HashFunction::Poseidon,
//             Sha256MerkleTreeConfig,
//         >::new(18);
//         merkle_tree.init(3, light_merkle_tree::HashFunction::Poseidon);
//         merkle_tree

//     // merkle_tree.init(18, HashFunction::Poseidon);
//     // merkle_tree.insert(leaf_left, leaf_right);
//     Ok(())
// }
#[derive(Accounts)]
pub struct Initialize {
    // /// CHECK:` doc comment explaining why no checks through types are necessary.
    // signer: AccountInfo<'info>,
    // /// CHECK:` doc comment explaining why no checks through types are necessary.
    // payer: AccountInfo<'info>,
}

// test cases were created with circomlibjs poseidon([1, ...]) for 1 to 16 inputs
const TEST_CASES: [[u8; 32]; 16] = [
    [
        41, 23, 97, 0, 234, 169, 98, 189, 193, 254, 108, 101, 77, 106, 60, 19, 14, 150, 164, 209,
        22, 139, 51, 132, 139, 137, 125, 197, 2, 130, 1, 51,
    ],
    [
        0, 122, 243, 70, 226, 211, 4, 39, 158, 121, 224, 169, 243, 2, 63, 119, 18, 148, 167, 138,
        203, 112, 231, 63, 144, 175, 226, 124, 173, 64, 30, 129,
    ],
    [
        2, 192, 6, 110, 16, 167, 42, 189, 43, 51, 195, 178, 20, 203, 62, 129, 188, 177, 182, 227,
        9, 97, 205, 35, 194, 2, 177, 134, 115, 191, 37, 67,
    ],
    [
        8, 44, 156, 55, 10, 13, 36, 244, 65, 111, 188, 65, 74, 55, 104, 31, 120, 68, 45, 39, 216,
        99, 133, 153, 28, 23, 214, 252, 12, 75, 125, 113,
    ],
    [
        16, 56, 150, 5, 174, 104, 141, 79, 20, 219, 133, 49, 34, 196, 125, 102, 168, 3, 199, 43,
        65, 88, 156, 177, 191, 134, 135, 65, 178, 6, 185, 187,
    ],
    [
        42, 115, 246, 121, 50, 140, 62, 171, 114, 74, 163, 229, 189, 191, 80, 179, 144, 53, 215,
        114, 159, 19, 91, 151, 9, 137, 15, 133, 197, 220, 94, 118,
    ],
    [
        34, 118, 49, 10, 167, 243, 52, 58, 40, 66, 20, 19, 157, 157, 169, 89, 190, 42, 49, 178,
        199, 8, 165, 248, 25, 84, 178, 101, 229, 58, 48, 184,
    ],
    [
        23, 126, 20, 83, 196, 70, 225, 176, 125, 43, 66, 51, 66, 81, 71, 9, 92, 79, 202, 187, 35,
        61, 35, 11, 109, 70, 162, 20, 217, 91, 40, 132,
    ],
    [
        14, 143, 238, 47, 228, 157, 163, 15, 222, 235, 72, 196, 46, 187, 68, 204, 110, 231, 5, 95,
        97, 251, 202, 94, 49, 59, 138, 95, 202, 131, 76, 71,
    ],
    [
        46, 196, 198, 94, 99, 120, 171, 140, 115, 48, 133, 79, 74, 112, 119, 193, 255, 146, 96,
        228, 72, 133, 196, 184, 29, 209, 49, 173, 58, 134, 205, 150,
    ],
    [
        0, 113, 61, 65, 236, 166, 53, 241, 23, 212, 236, 188, 235, 95, 58, 102, 220, 65, 66, 235,
        112, 181, 103, 101, 188, 53, 143, 27, 236, 64, 187, 155,
    ],
    [
        20, 57, 11, 224, 186, 239, 36, 155, 212, 124, 101, 221, 172, 101, 194, 229, 46, 133, 19,
        192, 129, 193, 205, 114, 201, 128, 6, 9, 142, 154, 143, 190,
    ],
    [
        46, 189, 128, 161, 169, 134, 85, 62, 67, 87, 243, 70, 211, 225, 145, 254, 148, 6, 253, 243,
        71, 34, 120, 31, 232, 83, 111, 99, 230, 198, 92, 108,
    ],
    [
        38, 42, 196, 73, 28, 210, 208, 129, 149, 54, 79, 247, 165, 12, 238, 56, 66, 117, 50, 113,
        188, 114, 50, 216, 85, 147, 150, 58, 127, 104, 233, 221,
    ],
    [
        27, 225, 209, 175, 237, 11, 90, 129, 139, 218, 5, 21, 20, 49, 65, 35, 218, 22, 99, 154,
        152, 243, 138, 136, 36, 39, 1, 149, 158, 199, 205, 61,
    ],
    [
        35, 235, 143, 111, 217, 66, 220, 208, 175, 10, 19, 232, 111, 181, 60, 252, 121, 177, 148,
        13, 210, 181, 154, 155, 141, 8, 85, 118, 43, 126, 211, 142,
    ],
];

#[test]
fn test_execute_poseidon() {
    execute_poseidon().unwrap();
}

#[test]
fn test_greater_than_field_size() {
    msg!("here");
    let mut greater_than_field_size = FrConfig::MODULUS; //  + Fr::one()
    msg!("greater_than_field_size");
    BigInteger::sub_with_borrow(&mut greater_than_field_size, &BigInteger256::from(1u64));

    msg!(
        "greater_than_field_size bytes {:?}",
        greater_than_field_size
    );
    let res = hash(greater_than_field_size.to_bytes_be().as_slice()).unwrap();
    BigInteger::add_with_carry(&mut greater_than_field_size, &BigInteger256::from(10u64));
    let hash = hash(greater_than_field_size.to_bytes_be().as_slice()).unwrap();

    // msg!("hash_invalid_input {:?}", hash_invalid_input.unwrap().0);
    // msg!("hash1 {:?}", hash1.unwrap().0);
    // msg!("hash32 {:?}", hash32.unwrap().0);
}

fn maybe_get_value() -> Option<i32> {
    None
}

fn another_function() -> Option<i32> {
    let val = maybe_get_value();
    val.unwrap(); // This will panic if `val` is `None`
    None
}

#[test]
fn unwrap_test() {
    another_function();
}
