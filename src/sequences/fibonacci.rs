pub trait Fibonacci: Sized {
    /// The exact type of the `Iterator<Item = Self>` returned by
    /// `fibonacci_iter`.
    type Iter: Iterator<Item = Self>;

    /// Returns an `Iterator<Item = Self>` implementation that goes through all
    /// Fibonacci numbers from zero until just before overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathematical::sequences::Fibonacci;
    ///
    /// let mut iter = i8::fibonacci_iter();
    /// assert_eq!(iter.next(), Some(0));
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(3));
    /// assert_eq!(iter.next(), Some(5));
    /// assert_eq!(iter.nth(5), Some(89));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn fibonacci_iter() -> Self::Iter;

    /// Returns an `Option` containing either the n<sup>th</sup> Fibonacci
    /// number or `None` if that would cause overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathematical::sequences::Fibonacci;
    ///
    /// assert_eq!(i32::nth_fibonacci(&10), Some(55));
    /// assert_eq!(i32::nth_fibonacci(&50), None);
    /// ```
    fn nth_fibonacci(n: &Self) -> Option<Self>;
}

macro_rules! fibonacci_trait_from_signed_array {
    ($type:ty, $array:expr) => {
        impl self::Fibonacci for $type {
            type Iter = ::core::iter::Copied<::core::slice::Iter<'static, $type>>;

            fn fibonacci_iter() -> Self::Iter {
                ($array).iter().copied()
            }

            fn nth_fibonacci(n: &Self) -> Option<Self> {
                let array = ($array);

                if *n < 0 {
                    let element = array.get(-*n as usize).copied();
                    if *n & 1 == 0 {
                        element.map(<$type as ::core::ops::Neg>::neg)
                    } else {
                        element
                    }
                } else {
                    array.get(*n as usize).copied()
                }
            }
        }
    };
}

macro_rules! fibonacci_trait_from_unsigned_array {
    ($type:ty, $array:expr) => {
        impl $crate::sequences::fibonacci::Fibonacci for $type {
            type Iter = ::core::iter::Copied<::core::slice::Iter<'static, $type>>;

            fn fibonacci_iter() -> Self::Iter {
                ($array).iter().copied()
            }

            fn nth_fibonacci(n: &Self) -> Option<Self> {
                ($array).get(*n as usize).copied()
            }
        }
    };
}

fibonacci_trait_from_signed_array!(
    ::core::primitive::i8,
    [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]
);

fibonacci_trait_from_unsigned_array!(
    ::core::primitive::u8,
    [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233]
);

fibonacci_trait_from_signed_array!(
    ::core::primitive::i16,
    [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711, 28657
    ]
);

fibonacci_trait_from_unsigned_array!(
    ::core::primitive::u16,
    [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711, 28657, 46368
    ]
);

fibonacci_trait_from_signed_array!(
    ::core::primitive::i32,
    [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
        2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
        165580141, 267914296, 433494437, 701408733, 1134903170, 1836311903
    ]
);

fibonacci_trait_from_unsigned_array!(
    ::core::primitive::u32,
    [
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269,
        2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
        165580141, 267914296, 433494437, 701408733, 1134903170, 1836311903, 2971215073
    ]
);

fibonacci_trait_from_signed_array!(
    ::core::primitive::i64,
    [
        0,
        1,
        1,
        2,
        3,
        5,
        8,
        13,
        21,
        34,
        55,
        89,
        144,
        233,
        377,
        610,
        987,
        1597,
        2584,
        4181,
        6765,
        10946,
        17711,
        28657,
        46368,
        75025,
        121393,
        196418,
        317811,
        514229,
        832040,
        1346269,
        2178309,
        3524578,
        5702887,
        9227465,
        14930352,
        24157817,
        39088169,
        63245986,
        102334155,
        165580141,
        267914296,
        433494437,
        701408733,
        1134903170,
        1836311903,
        2971215073,
        4807526976,
        7778742049,
        12586269025,
        20365011074,
        32951280099,
        53316291173,
        86267571272,
        139583862445,
        225851433717,
        365435296162,
        591286729879,
        956722026041,
        1548008755920,
        2504730781961,
        4052739537881,
        6557470319842,
        10610209857723,
        17167680177565,
        27777890035288,
        44945570212853,
        72723460248141,
        117669030460994,
        190392490709135,
        308061521170129,
        498454011879264,
        806515533049393,
        1304969544928657,
        2111485077978050,
        3416454622906707,
        5527939700884757,
        8944394323791464,
        14472334024676221,
        23416728348467685,
        37889062373143906,
        61305790721611591,
        99194853094755497,
        160500643816367088,
        259695496911122585,
        420196140727489673,
        679891637638612258,
        1100087778366101931,
        1779979416004714189,
        2880067194370816120,
        4660046610375530309,
        7540113804746346429
    ]
);

fibonacci_trait_from_unsigned_array!(
    ::core::primitive::u64,
    [
        0,
        1,
        1,
        2,
        3,
        5,
        8,
        13,
        21,
        34,
        55,
        89,
        144,
        233,
        377,
        610,
        987,
        1597,
        2584,
        4181,
        6765,
        10946,
        17711,
        28657,
        46368,
        75025,
        121393,
        196418,
        317811,
        514229,
        832040,
        1346269,
        2178309,
        3524578,
        5702887,
        9227465,
        14930352,
        24157817,
        39088169,
        63245986,
        102334155,
        165580141,
        267914296,
        433494437,
        701408733,
        1134903170,
        1836311903,
        2971215073,
        4807526976,
        7778742049,
        12586269025,
        20365011074,
        32951280099,
        53316291173,
        86267571272,
        139583862445,
        225851433717,
        365435296162,
        591286729879,
        956722026041,
        1548008755920,
        2504730781961,
        4052739537881,
        6557470319842,
        10610209857723,
        17167680177565,
        27777890035288,
        44945570212853,
        72723460248141,
        117669030460994,
        190392490709135,
        308061521170129,
        498454011879264,
        806515533049393,
        1304969544928657,
        2111485077978050,
        3416454622906707,
        5527939700884757,
        8944394323791464,
        14472334024676221,
        23416728348467685,
        37889062373143906,
        61305790721611591,
        99194853094755497,
        160500643816367088,
        259695496911122585,
        420196140727489673,
        679891637638612258,
        1100087778366101931,
        1779979416004714189,
        2880067194370816120,
        4660046610375530309,
        7540113804746346429,
        12200160415121876738
    ]
);

fibonacci_trait_from_signed_array!(
    ::core::primitive::i128,
    [
        0,
        1,
        1,
        2,
        3,
        5,
        8,
        13,
        21,
        34,
        55,
        89,
        144,
        233,
        377,
        610,
        987,
        1597,
        2584,
        4181,
        6765,
        10946,
        17711,
        28657,
        46368,
        75025,
        121393,
        196418,
        317811,
        514229,
        832040,
        1346269,
        2178309,
        3524578,
        5702887,
        9227465,
        14930352,
        24157817,
        39088169,
        63245986,
        102334155,
        165580141,
        267914296,
        433494437,
        701408733,
        1134903170,
        1836311903,
        2971215073,
        4807526976,
        7778742049,
        12586269025,
        20365011074,
        32951280099,
        53316291173,
        86267571272,
        139583862445,
        225851433717,
        365435296162,
        591286729879,
        956722026041,
        1548008755920,
        2504730781961,
        4052739537881,
        6557470319842,
        10610209857723,
        17167680177565,
        27777890035288,
        44945570212853,
        72723460248141,
        117669030460994,
        190392490709135,
        308061521170129,
        498454011879264,
        806515533049393,
        1304969544928657,
        2111485077978050,
        3416454622906707,
        5527939700884757,
        8944394323791464,
        14472334024676221,
        23416728348467685,
        37889062373143906,
        61305790721611591,
        99194853094755497,
        160500643816367088,
        259695496911122585,
        420196140727489673,
        679891637638612258,
        1100087778366101931,
        1779979416004714189,
        2880067194370816120,
        4660046610375530309,
        7540113804746346429,
        12200160415121876738,
        19740274219868223167,
        31940434634990099905,
        51680708854858323072,
        83621143489848422977,
        135301852344706746049,
        218922995834555169026,
        354224848179261915075,
        573147844013817084101,
        927372692193078999176,
        1500520536206896083277,
        2427893228399975082453,
        3928413764606871165730,
        6356306993006846248183,
        10284720757613717413913,
        16641027750620563662096,
        26925748508234281076009,
        43566776258854844738105,
        70492524767089125814114,
        114059301025943970552219,
        184551825793033096366333,
        298611126818977066918552,
        483162952612010163284885,
        781774079430987230203437,
        1264937032042997393488322,
        2046711111473984623691759,
        3311648143516982017180081,
        5358359254990966640871840,
        8670007398507948658051921,
        14028366653498915298923761,
        22698374052006863956975682,
        36726740705505779255899443,
        59425114757512643212875125,
        96151855463018422468774568,
        155576970220531065681649693,
        251728825683549488150424261,
        407305795904080553832073954,
        659034621587630041982498215,
        1066340417491710595814572169,
        1725375039079340637797070384,
        2791715456571051233611642553,
        4517090495650391871408712937,
        7308805952221443105020355490,
        11825896447871834976429068427,
        19134702400093278081449423917,
        30960598847965113057878492344,
        50095301248058391139327916261,
        81055900096023504197206408605,
        131151201344081895336534324866,
        212207101440105399533740733471,
        343358302784187294870275058337,
        555565404224292694404015791808,
        898923707008479989274290850145,
        1454489111232772683678306641953,
        2353412818241252672952597492098,
        3807901929474025356630904134051,
        6161314747715278029583501626149,
        9969216677189303386214405760200,
        16130531424904581415797907386349,
        26099748102093884802012313146549,
        42230279526998466217810220532898,
        68330027629092351019822533679447,
        110560307156090817237632754212345,
        178890334785183168257455287891792,
        289450641941273985495088042104137,
        468340976726457153752543329995929,
        757791618667731139247631372100066,
        1226132595394188293000174702095995,
        1983924214061919432247806074196061,
        3210056809456107725247980776292056,
        5193981023518027157495786850488117,
        8404037832974134882743767626780173,
        13598018856492162040239554477268290,
        22002056689466296922983322104048463,
        35600075545958458963222876581316753,
        57602132235424755886206198685365216,
        93202207781383214849429075266681969,
        150804340016807970735635273952047185,
        244006547798191185585064349218729154,
        394810887814999156320699623170776339,
        638817435613190341905763972389505493,
        1033628323428189498226463595560281832,
        1672445759041379840132227567949787325,
        2706074082469569338358691163510069157,
        4378519841510949178490918731459856482,
        7084593923980518516849609894969925639,
        11463113765491467695340528626429782121,
        18547707689471986212190138521399707760,
        30010821454963453907530667147829489881,
        48558529144435440119720805669229197641,
        78569350599398894027251472817058687522,
        127127879743834334146972278486287885163,
    ]
);

fibonacci_trait_from_unsigned_array!(
    ::core::primitive::u128,
    [
        0,
        1,
        1,
        2,
        3,
        5,
        8,
        13,
        21,
        34,
        55,
        89,
        144,
        233,
        377,
        610,
        987,
        1597,
        2584,
        4181,
        6765,
        10946,
        17711,
        28657,
        46368,
        75025,
        121393,
        196418,
        317811,
        514229,
        832040,
        1346269,
        2178309,
        3524578,
        5702887,
        9227465,
        14930352,
        24157817,
        39088169,
        63245986,
        102334155,
        165580141,
        267914296,
        433494437,
        701408733,
        1134903170,
        1836311903,
        2971215073,
        4807526976,
        7778742049,
        12586269025,
        20365011074,
        32951280099,
        53316291173,
        86267571272,
        139583862445,
        225851433717,
        365435296162,
        591286729879,
        956722026041,
        1548008755920,
        2504730781961,
        4052739537881,
        6557470319842,
        10610209857723,
        17167680177565,
        27777890035288,
        44945570212853,
        72723460248141,
        117669030460994,
        190392490709135,
        308061521170129,
        498454011879264,
        806515533049393,
        1304969544928657,
        2111485077978050,
        3416454622906707,
        5527939700884757,
        8944394323791464,
        14472334024676221,
        23416728348467685,
        37889062373143906,
        61305790721611591,
        99194853094755497,
        160500643816367088,
        259695496911122585,
        420196140727489673,
        679891637638612258,
        1100087778366101931,
        1779979416004714189,
        2880067194370816120,
        4660046610375530309,
        7540113804746346429,
        12200160415121876738,
        19740274219868223167,
        31940434634990099905,
        51680708854858323072,
        83621143489848422977,
        135301852344706746049,
        218922995834555169026,
        354224848179261915075,
        573147844013817084101,
        927372692193078999176,
        1500520536206896083277,
        2427893228399975082453,
        3928413764606871165730,
        6356306993006846248183,
        10284720757613717413913,
        16641027750620563662096,
        26925748508234281076009,
        43566776258854844738105,
        70492524767089125814114,
        114059301025943970552219,
        184551825793033096366333,
        298611126818977066918552,
        483162952612010163284885,
        781774079430987230203437,
        1264937032042997393488322,
        2046711111473984623691759,
        3311648143516982017180081,
        5358359254990966640871840,
        8670007398507948658051921,
        14028366653498915298923761,
        22698374052006863956975682,
        36726740705505779255899443,
        59425114757512643212875125,
        96151855463018422468774568,
        155576970220531065681649693,
        251728825683549488150424261,
        407305795904080553832073954,
        659034621587630041982498215,
        1066340417491710595814572169,
        1725375039079340637797070384,
        2791715456571051233611642553,
        4517090495650391871408712937,
        7308805952221443105020355490,
        11825896447871834976429068427,
        19134702400093278081449423917,
        30960598847965113057878492344,
        50095301248058391139327916261,
        81055900096023504197206408605,
        131151201344081895336534324866,
        212207101440105399533740733471,
        343358302784187294870275058337,
        555565404224292694404015791808,
        898923707008479989274290850145,
        1454489111232772683678306641953,
        2353412818241252672952597492098,
        3807901929474025356630904134051,
        6161314747715278029583501626149,
        9969216677189303386214405760200,
        16130531424904581415797907386349,
        26099748102093884802012313146549,
        42230279526998466217810220532898,
        68330027629092351019822533679447,
        110560307156090817237632754212345,
        178890334785183168257455287891792,
        289450641941273985495088042104137,
        468340976726457153752543329995929,
        757791618667731139247631372100066,
        1226132595394188293000174702095995,
        1983924214061919432247806074196061,
        3210056809456107725247980776292056,
        5193981023518027157495786850488117,
        8404037832974134882743767626780173,
        13598018856492162040239554477268290,
        22002056689466296922983322104048463,
        35600075545958458963222876581316753,
        57602132235424755886206198685365216,
        93202207781383214849429075266681969,
        150804340016807970735635273952047185,
        244006547798191185585064349218729154,
        394810887814999156320699623170776339,
        638817435613190341905763972389505493,
        1033628323428189498226463595560281832,
        1672445759041379840132227567949787325,
        2706074082469569338358691163510069157,
        4378519841510949178490918731459856482,
        7084593923980518516849609894969925639,
        11463113765491467695340528626429782121,
        18547707689471986212190138521399707760,
        30010821454963453907530667147829489881,
        48558529144435440119720805669229197641,
        78569350599398894027251472817058687522,
        127127879743834334146972278486287885163,
        205697230343233228174223751303346572685,
        332825110087067562321196029789634457848
    ]
);

fibonacci_trait_from_signed_array!(::core::primitive::isize, {
    const ARRAY_SIZE: usize = {
        let mut result = 0usize;
        let mut a = Some(0isize);
        let mut b = Some(1isize);

        while a.is_some() {
            let temp = b;
            b = match (a, b) {
                (Some(a), Some(b)) => a.checked_add(b),
                _ => None,
            };
            a = temp;

            result += 1;
        }

        result
    };

    const ARRAY: [isize; ARRAY_SIZE] = {
        let mut result = [0isize; ARRAY_SIZE];
        let mut a = Some(0isize);
        let mut b = Some(1isize);

        let mut i = 0;
        while a.is_some() {
            result[i] = match a {
                Some(a) => a,
                _ => loop {},
            };

            let temp = b;
            b = match (a, b) {
                (Some(a), Some(b)) => a.checked_add(b),
                _ => None,
            };
            a = temp;

            i += 1;
        }

        result
    };

    ARRAY
});

fibonacci_trait_from_unsigned_array!(::core::primitive::usize, {
    const ARRAY_SIZE: usize = {
        let mut result = 0usize;
        let mut a = Some(0usize);
        let mut b = Some(1usize);

        while a.is_some() {
            let temp = b;
            b = match (a, b) {
                (Some(a), Some(b)) => a.checked_add(b),
                _ => None,
            };
            a = temp;

            result += 1;
        }

        result
    };

    const ARRAY: [usize; ARRAY_SIZE] = {
        let mut result = [0usize; ARRAY_SIZE];
        let mut a = Some(0usize);
        let mut b = Some(1usize);

        let mut i = 0;
        while a.is_some() {
            result[i] = match a {
                Some(a) => a,
                _ => loop {},
            };

            let temp = b;
            b = match (a, b) {
                (Some(a), Some(b)) => a.checked_add(b),
                _ => None,
            };
            a = temp;

            i += 1;
        }

        result
    };

    ARRAY
});

#[cfg(any(feature = "rug", doc, test))]
#[doc(cfg(feature = "rug"))]
impl Fibonacci for rug::Integer {
    type Iter = RugIter;

    fn fibonacci_iter() -> Self::Iter {
        RugIter::new()
    }

    fn nth_fibonacci(n: &Self) -> Option<Self> {
        if *n < rug::Integer::new() {
            (-n.clone()).to_usize().and_then(|n| {
                Self::fibonacci_iter().nth(n).map(
                    |result| {
                        if n & 1 == 1 {
                            -result
                        } else {
                            result
                        }
                    },
                )
            })
        } else {
            n.to_usize().and_then(|n| Self::fibonacci_iter().nth(n))
        }
    }
}

#[cfg(any(feature = "rug", doc, test))]
#[doc(cfg(feature = "rug"))]
pub struct RugIter {
    a: rug::Integer,
    b: rug::Integer,
    a_next: bool,
}

#[cfg(any(feature = "rug", doc, test))]
#[doc(cfg(feature = "rug"))]
impl RugIter {
    fn new() -> Self {
        Self {
            a: rug::Integer::new(),
            b: rug::Integer::new() + 1u8,
            a_next: true,
        }
    }
}

#[cfg(any(feature = "rug", doc, test))]
#[doc(cfg(feature = "rug"))]
impl Iterator for RugIter {
    type Item = rug::Integer;

    fn next(&mut self) -> Option<Self::Item> {
        Some(if self.a_next {
            self.a_next = false;
            let result = self.a.clone();
            self.a += &self.b;
            result
        } else {
            self.a_next = true;
            let result = self.b.clone();
            self.b += &self.a;
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_signed_bounded_nth {
        ($type:ty, $test_name:ident) => {
            #[test]
            fn $test_name() {
                let mut a = ::core::option::Option::Some(0);
                let mut b = ::core::option::Option::Some(1);

                for i in 0.. {
                    match (
                        a,
                        b,
                        <$type>::nth_fibonacci(&-i),
                        <$type>::nth_fibonacci(&i),
                    ) {
                        (
                            ::core::option::Option::Some(a),
                            _,
                            ::core::option::Option::Some(m),
                            ::core::option::Option::Some(n),
                        ) => {
                            ::core::assert_eq!(if i & 1 == 0 { -a } else { a }, m);
                            ::core::assert_eq!(a, n);
                        }
                        (
                            ::core::option::Option::None,
                            ::core::option::Option::None,
                            ::core::option::Option::None,
                            ::core::option::Option::None,
                        ) => break,
                        (::core::option::Option::None, ::core::option::Option::None, _, _) => {
                            ::core::panic!("tested results produce too many elements")
                        }
                        (_, ::core::option::Option::None, _, _) => {
                            ::core::panic!("tested results produce too few elements")
                        }
                        _ => ::core::unreachable!(),
                    }
                    let temp = b;
                    b = a.and_then(|a| b.and_then(|b| <$type>::checked_add(a, b)));
                    a = temp;
                }
            }
        };
    }

    macro_rules! test_unsigned_bounded_nth {
        ($type:ty, $test_name:ident) => {
            #[test]
            fn $test_name() {
                let mut a = ::core::option::Option::Some(0);
                let mut b = ::core::option::Option::Some(1);

                for i in 0.. {
                    match (a, b, <$type>::nth_fibonacci(&i)) {
                        (::core::option::Option::Some(a), _, ::core::option::Option::Some(n)) => {
                            ::core::assert_eq!(a, n);
                        }
                        (
                            ::core::option::Option::None,
                            ::core::option::Option::None,
                            ::core::option::Option::None,
                        ) => break,
                        (::core::option::Option::None, ::core::option::Option::None, _) => {
                            ::core::panic!("tested results produce too many elements")
                        }
                        (_, ::core::option::Option::None, ::core::option::Option::None) => {
                            ::core::panic!("tested results produce too few elements")
                        }
                        _ => ::core::unreachable!(),
                    }
                    let temp = b;
                    b = a.and_then(|a| b.and_then(|b| <$type>::checked_add(a, b)));
                    a = temp;
                }
            }
        };
    }

    macro_rules! test_bounded_iter {
        ($type:ty, $test_name:ident) => {
            #[test]
            fn $test_name() {
                let mut a = ::core::option::Option::Some(0);
                let mut b = ::core::option::Option::Some(1);

                let mut iter = <$type>::fibonacci_iter();
                loop {
                    match (a, b, iter.next()) {
                        (::core::option::Option::Some(a), _, ::core::option::Option::Some(n)) => {
                            ::core::assert_eq!(a, n);
                        }
                        (
                            ::core::option::Option::None,
                            ::core::option::Option::None,
                            ::core::option::Option::None,
                        ) => break,
                        (::core::option::Option::None, ::core::option::Option::None, _) => {
                            ::core::panic!("tested results produce too many elements")
                        }
                        (_, ::core::option::Option::None, ::core::option::Option::None) => {
                            ::core::panic!("tested results produce too few elements")
                        }
                        _ => ::core::unreachable!(),
                    }
                    let temp = b;
                    b = a.and_then(|a| b.and_then(|b| <$type>::checked_add(a, b)));
                    a = temp;
                }
            }
        };
    }

    test_signed_bounded_nth!(::core::primitive::i8, i8_nth);
    test_unsigned_bounded_nth!(::core::primitive::u8, u8_nth);
    test_signed_bounded_nth!(::core::primitive::i16, i16_nth);
    test_unsigned_bounded_nth!(::core::primitive::u16, u16_nth);
    test_signed_bounded_nth!(::core::primitive::i32, i32_nth);
    test_unsigned_bounded_nth!(::core::primitive::u32, u32_nth);
    test_signed_bounded_nth!(::core::primitive::i64, i64_nth);
    test_unsigned_bounded_nth!(::core::primitive::u64, u64_nth);
    test_signed_bounded_nth!(::core::primitive::i128, i128_nth);
    test_unsigned_bounded_nth!(::core::primitive::u128, u128_nth);
    test_signed_bounded_nth!(::core::primitive::isize, isize_nth);
    test_unsigned_bounded_nth!(::core::primitive::usize, usize_nth);
    test_bounded_iter!(::core::primitive::i8, i8_iter);
    test_bounded_iter!(::core::primitive::u8, u8_iter);
    test_bounded_iter!(::core::primitive::i16, i16_iter);
    test_bounded_iter!(::core::primitive::u16, u16_iter);
    test_bounded_iter!(::core::primitive::i32, i32_iter);
    test_bounded_iter!(::core::primitive::u32, u32_iter);
    test_bounded_iter!(::core::primitive::i64, i64_iter);
    test_bounded_iter!(::core::primitive::u64, u64_iter);
    test_bounded_iter!(::core::primitive::i128, i128_iter);
    test_bounded_iter!(::core::primitive::u128, u128_iter);
    test_bounded_iter!(::core::primitive::isize, isize_iter);
    test_bounded_iter!(::core::primitive::usize, usize_iter);
}
