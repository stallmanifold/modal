use num::{One, Integer, BigInt, Num};
use extended_gcd::ExtendedGcd;


struct Test {
    data: Vec<TestCase>,
}

struct TestCase {
    x:      BigInt,
    y:      BigInt,
    gcd_xy: BigInt,
    coef_x: BigInt,
    coef_y: BigInt,
}

fn gcd_test_cases() -> Test {
    Test {
        data: vec![
            // Example 14.62 from 'Handbook of Applied Cryptography'
            TestCase {
                x:      BigInt::from(693),
                y:      BigInt::from(609),
                gcd_xy: BigInt::from(21),
                coef_x: BigInt::from(-181),
                coef_y: BigInt::from(206),
            },
            TestCase {
                x:      <BigInt as Num>::from_str_radix("294248851906335666255475965306356\
                                                         33692994051181214434796327203075", 10).unwrap(),
                y:      <BigInt as Num>::from_str_radix("919087970205406919189208074679995\
                                                         123273961", 10).unwrap(),
                gcd_xy: <BigInt as Num>::from_str_radix("3", 10).unwrap(),
                coef_x: <BigInt as Num>::from_str_radix("-13780862337125063622355860403035\
                                                          4666602474", 10).unwrap(),
                coef_y: <BigInt as Num>::from_str_radix("441198563405422365116499908277872\
                                                         3845569132647537334425498184473", 10).unwrap(),
            },
            TestCase {
                x:      BigInt::from(613769282),
                y:      BigInt::from(716888961),
                gcd_xy: BigInt::from(29),
                coef_x: BigInt::from(2233618),
                coef_y: BigInt::from(-1912327),
            },
            TestCase {
                x:      <BigInt as Num>::from_str_radix("240007433584325356921800502290759\
                                                         31889728948", 10).unwrap(),
                y:      <BigInt as Num>::from_str_radix("476057445640162830420892551794736\
                                                         263384096274661520", 10).unwrap(),
                gcd_xy: <BigInt as Num>::from_str_radix("4", 10).unwrap(),
                coef_x: <BigInt as Num>::from_str_radix("57452030383374486115869206982212629264831027293013", 10).unwrap(),
                coef_y: <BigInt as Num>::from_str_radix("-2896481189991724314184721919272517544310316", 10).unwrap(), 
            },
            TestCase {
                x:      <BigInt as Num>::from_str_radix("100000000000000000000000000000000000000000\
                                                         000000000000000000000000000000000000000000\
                                                         000000000000000000000000000000000000000000\
                                                         000000000000000000000000000000000000000000", 10).unwrap(),
                y:      <BigInt as Num>::from_str_radix("100000000000000000000000000000000000000000\
                                                         000000000000000000000000000000000000000000\
                                                         000000000000000000000000000000000000000000", 10).unwrap(),
                gcd_xy: <BigInt as Num>::from_str_radix("100000000000000000000000000000000000000000\
                                                         000000000000000000000000000000000000000000\
                                                         000000000000000000000000000000000000000000", 10).unwrap(),
                coef_x: <BigInt as Num>::from_str_radix("0", 10).unwrap(),
                coef_y: <BigInt as Num>::from_str_radix("1", 10).unwrap(),
            }
        ]
    }
}

fn run_gcd_test(test: &Test) {
    for test_case in test.data.iter() {
        let result = <BigInt as ExtendedGcd<&_>>::extended_gcd(&test_case.x, &test_case.y).unwrap();
        assert_eq!(result.gcd_xy, test_case.gcd_xy);
        assert!(<BigInt as ExtendedGcd<&_>>::valid_solution(&test_case.x, &test_case.y, 
                                          &result.coef_x, &result.coef_y, &test_case.gcd_xy));
        // Show that coefficients returned by the extended_gcd algorithm are not unique.
        assert!(<BigInt as ExtendedGcd<&_>>::valid_solution(&test_case.x, &test_case.y, 
                                    &test_case.coef_x, &test_case.coef_y, &test_case.gcd_xy));
    }
}

#[test]
fn test_extended_gcd() {
    run_gcd_test(&gcd_test_cases());
}