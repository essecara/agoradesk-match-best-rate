use reqwest::blocking::Client;
use serde_json::Value;
use std::io::Read;
use std::{thread, time::Duration, collections::HashMap};
use std::env;

fn get_my_offer_price(client: &Client, id: &str) -> Result<f64, Box<dyn std::error::Error>> {

    let mut url = "https://agoradesk.com/api/v1/ad-get/".to_owned();
    url.push_str(id);

    let response = client.get(url)
        .send()?;

    let json: Value = serde_json::from_str(response.text().unwrap().as_str())?;
    let tmp_price = json["data"]["ad_list"][0]["data"]["temp_price"].as_str().unwrap();    
    let price: f64 = tmp_price.parse::<f64>()?;

    return Ok(price);
}

fn get_offers(client: &Client, currency: &str, method: &str, skip: Vec<Value>) -> Result<Vec<f64>, Box<dyn std::error::Error>> {

    let mut url: String = "https://agoradesk.com/api/v1/buy-monero-online/".to_owned();
    url.push_str(currency);
    url.push_str("/");
    url.push_str(method);

    let response = client.get(url)
        .send()?;

    let json: Value = serde_json::from_str(response.text().unwrap().as_str())?;
    let mut prices: Vec<f64> = Vec::new();  


    if let Some(ads) = json["data"]["ad_list"].as_array() {
        
        for ad in ads {

            let index = skip.iter().position(|p| p.as_str().unwrap() == ad["data"]["ad_id"]); 
            if index.is_some() {
                continue;
            }

            let temp_price = ad["data"]["temp_price"].as_str().unwrap();
            prices.push(temp_price.parse::<f64>()?);
        }
    }

    return Ok(prices);

}


fn get_kraken_rate(currency: &str) -> Result<f64, Box<dyn std::error::Error>> {
    
    let client: Client = reqwest::blocking::Client::builder()
        .build()?;

    let mut url = "https://api.kraken.com/0/public/Ticker?pair=XMR".to_owned();
    url.push_str(currency);

    let mut key: String =  "XXMRZ".to_owned();
    key.push_str(currency);


    let response = client.get(url)
        .send()?;


    let json: Value = serde_json::from_str(response.text().unwrap().as_str())?;

    let rate: f64 = json["result"][key]["c"][0].as_str().unwrap().parse::<f64>()?;

    Ok(rate)
}


fn change_price(client: &Client, ad: &str, price: f64) -> Result<(), Box<dyn std::error::Error>> {

    let mut url = "https://agoradesk.com/api/v1/ad-equation/".to_string();
    url.push_str(ad);

    let mut body = HashMap::new();
    body.insert("price_equation", price.to_string());


    match client.post(url)
        .json(&body)
        .send() {

            Ok(res) => {
                println!("{}", res.text()?);
                return Ok(());
            },
            Err(err) => return Err(Box::new(err))
    };
}


fn main() -> Result<(), Box<dyn std::error::Error>> {


    let args:Vec<String> = env::args().collect();
    let pth = &args[1];

    let mut file = std::fs::File::open(pth).expect("Non existing configuration!");
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let cnf: Value = serde_json::from_str(&data).unwrap_or_else(|error| {
        panic!("Your configuration files is wrong: {}", error);
    });

    let apikey = &cnf["apikey"].as_str().unwrap();
    let currency = &cnf["currency"].as_str().unwrap();    
    let method = &cnf["method"].as_str().unwrap();
    let ad = &cnf["ad"].as_str().unwrap();
    let margin = &cnf["margin"].as_f64().unwrap();
    let cnf_limit: &f64 = &cnf["limit"].as_f64().unwrap();
    let skip = cnf["skip_ads"].as_array().unwrap();
    let mut limit: f64;   
    let substract: f64 = 0.01;

    let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", apikey.parse().unwrap());
    headers.insert("User-Agent", "PostmanRuntime/7.32.2".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let lm_client: Client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();


    loop {

        if 0f64 > *cnf_limit {
            limit = (get_kraken_rate(currency).unwrap() * margin).ceil();
        } else {
            limit = *cnf_limit;
        }


        let prices = get_offers(&lm_client, currency, method, skip.clone()).unwrap();
        let myprice = get_my_offer_price(&lm_client, ad).unwrap();
        let mut newprice: f64 = myprice;
        let mut low: f64 = 100000.99;

        for price in prices {

            if price < low && price > limit {
                low = price;
            }

            if price < newprice && price > limit {
                newprice = price - substract;
            }
        }

        if low > myprice {
            newprice = low - substract;
        }
            
        if newprice != myprice {


            match change_price(&lm_client, ad, newprice) {
                Ok(_) => println!("Price change to: {}", newprice),
                Err(err) => println!("{:?}", err)
            }
        } 

        thread::sleep(Duration::from_secs(2 * 60));
    }
    
    Ok(())
}
