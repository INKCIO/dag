use base32;
use base64;
use bit_vec::BitVec;
use error::Result;
use obj_ser::to_string;
use ripemd160::Ripemd160;
use serde::ser::Serialize;
use sha2::{Digest, Sha256};

pub fn get_base64_hash<T>(object: &T) -> Result<String>
where
    T: Serialize,
{
    Ok(base64::encode(
        &Sha256::digest(&to_string(object)?.as_bytes()),
    ))
}

pub fn get_chash<T>(object: &T) -> Result<String>
where
    T: Serialize,
{
    let hash = Ripemd160::digest(&to_string(object)?.as_bytes());
    let truncate_hash = &hash[4..];

    let sha256 = Sha256::digest(truncate_hash);
    let checksum = [sha256[5], sha256[13], sha256[21], sha256[29]];

    //This is generated as a mix index from PI, see chash.js for details
    let index_for_mix = [
        1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2, 7, 9,
        5, /*0,*/ 2, 8, 8, 4, 1, 9, 7, 1, 6, 9, 3, 9, 9, 3, 7, 5, 1 /*0,*/,
    ];

    let mut mixed = BitVec::from_elem(160, false);
    let truncate_hash = BitVec::from_bytes(&truncate_hash);
    let checksum = BitVec::from_bytes(&checksum);

    //println!("truncate_hash {:?}", truncate_hash);
    //println!("checksum {:?}", checksum);

    let mut index = 0;
    let mut hash_index = 0;
    let mut mix_index = 0;

    for bit in checksum.iter() {
        let mut n = if mix_index == 0 {
            //The head is specially treated
            index_for_mix[mix_index]
        } else {
            index_for_mix[mix_index] - 1
        };

        while n > 0 {
            //println!("index {} mix_index {}  hash_index {}", index, mix_index, hash_index);
            mixed.set(index, truncate_hash[hash_index]);
            index = index + 1;
            hash_index = hash_index + 1;
            n = n - 1;
        }

        mixed.set(index, bit);
        //println!("index {} mix_index {}  hash_index {}", index, mix_index, hash_index);
        //println!("mixed {:?}", mixed);
        index = index + 1;

        mix_index = mix_index + 1;
    }

    //Append the tail
    while index < mixed.len() {
        //println!("index {} mix_index {}  hash_index {}", index, mix_index, hash_index);
        mixed.set(index, truncate_hash[hash_index]);
        index = index + 1;
        hash_index = hash_index + 1;
    }

    //println!("mixed {:?}", mixed);
    Ok(base32::encode(
        base32::Alphabet::RFC4648 { padding: true },
        &mixed.to_bytes(),
    ))
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_payload() {
    use serde_json;
    use spec;

    //Copied from the Unit json string
    let json = r#"{
                "outputs": [
                    {
                        "address": "7JXBJQPQC3466UPK7C6ABA6VVU6YFYAI",
                        "amount": 10000
                    },
                    {
                        "address": "JERTY5XNENMHYQW7NVBXUB5CU3IDODA3",
                        "amount": 99989412
                    }
                ],
                "inputs": [
                    {
                        "unit": "lQCxxsMslXLzQKybX2KArOGho8XuNf1Lpds2abdf8O4=",
                        "message_index": 0,
                        "output_index": 1
                    }
                ]
            }"#;
    let payload: spec::Payload = serde_json::from_str(json).unwrap();
    let expected = "5CYeTTa4VQxgF4b1Tn33NBlKilJadddwBMLvtp1HIus=";

    //println!("{:?}", to_base64_hash(&payload));
    assert_eq!(get_base64_hash(&payload).unwrap(), expected);
}

#[test]
fn test_chash160() {
    let data = "A0mQdZvy+bGpIu/yBSNt7eB4mTZUQiM173bIQTOQRz3U";
    let expected = "YFAR4AK2RSRTAWZ3ILRFZOMN7M7QJTJ2";

    assert_eq!(get_chash(&data).unwrap(), expected);
}
