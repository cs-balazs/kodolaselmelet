use kodolaselmelet::rsa::{decrypt, encrypt, get_keys};
use rug::Integer;
use std::str::FromStr;

fn main() {
    let keys = get_keys(
        &Integer::from_str("691826793068458536074208355133049291478531419048941848702313338608164245322895819651978896147719733508565736653903956362824739385732131890969671002559232884926169408928219198718725581288282235796285353558468100964394244945998514012276023216327756247926340827782709941975233386336751116416100943416897").unwrap(),
        &Integer::from_str("720714523785889173191354177371045760242297569261140563779852769239494236607878516643196470647354242607091192426841274197616849531050433864775643197014575443597601363224005309228633349876258651710793439200905812200126442288004063350129579058857449170104638055254079167970988227634912890827343610996771").unwrap(),
    );

    dbg!(&keys);

    let m = Integer::from(123456789);
    dbg!(&m);

    let encrypted = encrypt(&m, &keys.pubkey);
    dbg!(&encrypted);

    let decrypted = decrypt(&encrypted, &keys);
    dbg!(&decrypted);
}
