use crate::exponentiation::binary;
use rug::{rand::RandState, Integer};
use std::ops::AddAssign;

fn n_factors(n: &Integer) -> Integer {
    if n % Integer::from(2u64) == 1 {
        return Integer::from(0u64);
    }
    let mut exp = Integer::from(1u64);
    let mut n_clone = n.clone();
    while &n_clone % Integer::from(2u64) == 0 {
        n_clone /= Integer::from(2u64);
        exp += Integer::from(1u64);
    }

    exp - 1
}

pub fn miller_rabin(n: &Integer) -> bool {
    if *n == 2 {
        return true;
    }
    if n % Integer::from(2) == 0 || *n == 1 {
        return false;
    }

    let n_minus_one = n - Integer::from(1);
    let k = n_factors(&n_minus_one);
    let mut exp = binary(&Integer::from(2), &k, None);
    let r = &n_minus_one / exp.clone();

    let mut rng = RandState::new();
    let a = n - Integer::from(2);
    let mut a = a.random_below(&mut rng);
    a.add_assign(Integer::from(1));

    while exp > 0 {
        let expp = r.clone() * exp.clone();
        let res = binary(&a, &expp, Some(n));

        if res == 1 {
            exp >>= 1;
            continue;
        };

        // Checking for n - 1, which is -1
        if res == n_minus_one {
            break;
        }

        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::primality::miller_rabin;
    use rug::Integer;

    #[test]
    fn test() {
        let primes = [
            "2",
            "3",
            "5",
            "7",
            "543165915552570104247678780070604672279011289359839209727583364406691873327606394089689340382741755070311804128765079618964382566208339179079459485225742676430588575604511729456129994320325905584854716158199748699700993344792840061111419166698246900964457709548045917326301847809524228386681597635267",
            "691826793068458536074208355133049291478531419048941848702313338608164245322895819651978896147719733508565736653903956362824739385732131890969671002559232884926169408928219198718725581288282235796285353558468100964394244945998514012276023216327756247926340827782709941975233386336751116416100943416897",
            "720714523785889173191354177371045760242297569261140563779852769239494236607878516643196470647354242607091192426841274197616849531050433864775643197014575443597601363224005309228633349876258651710793439200905812200126442288004063350129579058857449170104638055254079167970988227634912890827343610996771",
            "654192561342273737540525596287228522458866748209355344196805430932937858733575088437483439928216182954804138665208507999277928799546663114534323365043147989308493599740693163912846408255526501434571396592122908458907213628524101013797685469295239805265510171803657372870466979387633939198951173246927",
            "983389828069245665884547151859586238260145204546001744760748176701110103071142463342218818934867896489832127332118010158891753427831379000978255113915046040935791699717694614182190446666573340390308631860645247204614492426734524967861717830533351482447223474314657448327342722551224532940073042873021"
        ];

        let non_primes = [
            "1",
            "9",
            "516",
            "798726597823569276598027",
            "19374971848167",
        ];

        for prime in primes {
            println!("Testing {}", prime);
            assert!(miller_rabin(&Integer::from_str(&prime).unwrap()))
        }

        for non_prime in non_primes {
            println!("Testing {}", non_prime);
            assert!(!miller_rabin(&Integer::from_str(&non_prime).unwrap()))
        }
    }
}
