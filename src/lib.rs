extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::Read;

#[derive(Debug, Default)]
pub struct Coin {
    pub symbol: String,
    pub price: f64,
    pub high: f64,
    pub low: f64,
    pub cny: f64,
    pub rate: f64,
    pub delay: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Json {
    code: i64,
    message: String,
    data: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    name: String,
    symbol: String,
    price: f64,
    high: f64,
    low: f64,
    timestamps: i64,
    volume: f64,
    change_hourly: f64,
    change_daily: f64,
    change_weekly: f64,
    change_monthly: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct RateJson {
    code: i64,
    message: String,
    data: RateData,
}

#[derive(Debug, Serialize, Deserialize)]
struct RateData {
    base: String,
    date: String,
    rates: Rates,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rates {
    #[serde(rename = "BITCNY")]
    bitcny: f64,
    #[serde(rename = "USDT")]
    usdt: f64,
    #[serde(rename = "CNY")]
    cny: f64,
    #[serde(rename = "HKD")]
    hkd: f64,
    #[serde(rename = "KRW")]
    krw: f64,
    #[serde(rename = "JPY")]
    jpy: f64,
    #[serde(rename = "EUR")]
    eur: f64,
    #[serde(rename = "AUD")]
    aud: f64,
    #[serde(rename = "BRL")]
    brl: f64,
    #[serde(rename = "CAD")]
    cad: f64,
    #[serde(rename = "CHF")]
    chf: f64,
    #[serde(rename = "CLP")]
    clp: f64,
    #[serde(rename = "CZK")]
    czk: f64,
    #[serde(rename = "DKK")]
    dkk: f64,
    #[serde(rename = "GBP")]
    gbp: f64,
    #[serde(rename = "HUF")]
    huf: f64,
    #[serde(rename = "IDR")]
    idr: f64,
    #[serde(rename = "ILS")]
    ils: f64,
    #[serde(rename = "INR")]
    inr: f64,
    #[serde(rename = "MXN")]
    mxn: f64,
    #[serde(rename = "MYR")]
    myr: f64,
    #[serde(rename = "NOK")]
    nok: f64,
    #[serde(rename = "NZD")]
    nzd: f64,
    #[serde(rename = "PHP")]
    php: f64,
    #[serde(rename = "CNH")]
    cnh: f64,
    #[serde(rename = "PKR")]
    pkr: f64,
    #[serde(rename = "PLN")]
    pln: f64,
    #[serde(rename = "RUB")]
    rub: f64,
    #[serde(rename = "SEK")]
    sek: f64,
    #[serde(rename = "SGD")]
    sgd: f64,
    #[serde(rename = "THB")]
    thb: f64,
    #[serde(rename = "TRY")]
    try: f64,
    #[serde(rename = "TWD")]
    twd: f64,
    #[serde(rename = "ZAR")]
    zar: f64,
    #[serde(rename = "BGN")]
    bgn: f64,
    #[serde(rename = "HRK")]
    hrk: f64,
    #[serde(rename = "ISK")]
    isk: f64,
    #[serde(rename = "RON")]
    ron: f64,
    #[serde(rename = "BTC")]
    btc: f64,
    #[serde(rename = "XBT")]
    xbt: f64,
    #[serde(rename = "ETH")]
    eth: f64,
    #[serde(rename = "XMR")]
    xmr: f64,
    #[serde(rename = "ETP")]
    etp: f64,
    #[serde(rename = "QTUM")]
    qtum: f64,
    #[serde(rename = "ETC")]
    etc: f64,
    #[serde(rename = "LTC")]
    ltc: f64,
    #[serde(rename = "ZEC")]
    zec: f64,
    #[serde(rename = "BCH")]
    bch: f64,
    #[serde(rename = "BNB")]
    bnb: f64,
    #[serde(rename = "DOGE")]
    doge: f64,
    #[serde(rename = "NZDT")]
    nzdt: f64,
    #[serde(rename = "EOS")]
    eos: f64,
    #[serde(rename = "HSR")]
    hsr: f64,
    #[serde(rename = "QASH")]
    qash: f64,
    #[serde(rename = "DAI")]
    dai: f64,
    #[serde(rename = "FBT")]
    fbt: f64,
    #[serde(rename = "CNYT")]
    cnyt: f64,
    #[serde(rename = "QC")]
    qc: f64,
    #[serde(rename = "DC")]
    dc: f64,
    #[serde(rename = "UT")]
    ut: f64,
    #[serde(rename = "C2C")]
    c2_c: f64,
    #[serde(rename = "BNC")]
    bnc: f64,
    #[serde(rename = "FCNY")]
    fcny: f64,
    #[serde(rename = "CNT")]
    cnt: f64,
    #[serde(rename = "CKUSD")]
    ckusd: i64,
    #[serde(rename = "BNT")]
    bnt: f64,
    #[serde(rename = "NEO")]
    neo: f64,
    #[serde(rename = "GAS")]
    gas: f64,
    #[serde(rename = "SWH")]
    swh: f64,
    #[serde(rename = "FRGC")]
    frgc: f64,
    #[serde(rename = "XLM")]
    xlm: f64,
    #[serde(rename = "NANO")]
    nano: f64,
    #[serde(rename = "BTCP")]
    btcp: f64,
    #[serde(rename = "DASH")]
    dash: f64,
    #[serde(rename = "WETH")]
    weth: f64,
    #[serde(rename = "CNET")]
    cnet: f64,
    #[serde(rename = "OKB")]
    okb: f64,
}

impl Coin {
    pub fn new(symbol: &str, delay: u64) -> Self {
        Coin {
            symbol: symbol.to_string(),
            delay,
            ..Default::default()
        }
    }

    pub fn get_symbol_rate(&mut self, name: &str) {
        let api_url = format!("https://data.block.cc/api/v1/exrate?base=USD&symbol={}", name);
        let mut resp = reqwest::get(&api_url).unwrap();
        let mut content = String::new();
        resp.read_to_string(&mut content).expect("Could not read body");

        let json: RateJson = serde_json::from_str(content.as_str()).unwrap();
        self.rate = json.data.rates.bitcny
    }

    pub fn get_symbol_price(&mut self) {
        let api_url = format!("https://data.block.cc/api/v1/price?symbol={}", self.symbol);
        let mut resp = reqwest::get(&api_url).unwrap();
        let mut content = String::new();
        resp.read_to_string(&mut content).expect("Could not read body");

        let json: Json = serde_json::from_str(content.as_str()).unwrap();
        self.symbol = json.data[0].symbol.clone();
        self.price = json.data[0].price.clone();

        if (self.price > self.high ||
            self.price < self.low) && self.high > 0.0 {
            let api_url = "https://sc.ftqq.com/SCU7858Tfbad8a804df5762202060d87265f49f458ff85975bcae.send";
            reqwest::Client::new()
                .post(api_url)
                .form(&[("desp", self.cny.to_string()), ("text", self.symbol.to_string())])
                .send()
                .expect("Cloud not post url");
        }

        self.high = json.data[0].high.clone();
        self.low = json.data[0].low.clone()
    }

    pub fn calculate_cny_price(&mut self) {
        self.cny = (self.price * self.rate * 10000.0).round() / 10000.0
    }
}
