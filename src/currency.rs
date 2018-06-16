use bimap::BiMap;
use std::fmt;
use std::str::FromStr;
use std::string::ToString;

lazy_static! {
    static ref GLOBAL_CURRENCY: BiMap<u32, &'static str> = {
        let mut m = BiMap::new();
        m.insert(784, "AED");
        m.insert(971, "AFN");
        m.insert(008, "ALL");
        m
    };
}

static NONE_CURRENCY: &'static str = "NONE";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Currency {
    code: u32,
}

impl Currency {
    pub fn name(&self) -> Option<&&str> {
        GLOBAL_CURRENCY.get_by_left(&self.code)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCurrencyError {
    kind: CurrencyErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CurrencyErrorKind {
    //    Empty,
    InvalidCurrency,
}

impl ParseCurrencyError {
    #[doc(hidden)]
    pub fn __description(&self) -> &str {
        match self.kind {
            //        CurrencyErrorKind::Empty => "cannot parse currency from empty string",
            CurrencyErrorKind::InvalidCurrency => "invalid currency name",
        }
    }
}

impl fmt::Display for ParseCurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.__description().fmt(f)
    }
}

impl FromStr for Currency {
    type Err = ParseCurrencyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = GLOBAL_CURRENCY.get_by_right(&s);
        match c {
            Some(currency_code) => Ok(Currency {
                code: *currency_code,
            }),
            None => Err(ParseCurrencyError {
                kind: CurrencyErrorKind::InvalidCurrency,
            }),
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = self.name();
        let s = match v {
            Some(s) => s.to_string(),
            None => NONE_CURRENCY.to_string(),
        };
        s.to_string().fmt(f)
    }
}

#[test]
fn test_to_and_from() {
    let aed = Currency { code: 784 };
    assert_eq!(aed.to_string(), "AED");

    let all = Currency::from_str("ALL");
    assert!(all.is_ok());
    assert_eq!(all.unwrap().code, 8);

    let usd = Currency::from_str("USD");
    assert!(usd.is_err());
}
