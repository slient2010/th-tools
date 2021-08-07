use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::fs;
use tokio::time::{sleep, Duration};
// use std::collections::HashMap;
extern crate redis;
use redis::Commands;

#[derive(Debug, Serialize, Deserialize)]
struct searchDetails {
    searchResults: Vec<CityInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct idValue {
    Id: String,
    Value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct resStatus {
    Timestamp: String,
    Ack: String,
    Errors: Vec<String>,
    Extension: Vec<idValue>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseResult {
    Result: bool,
    ErrorCode: i32,
    Response: searchDetails,
    ResponseStatus: resStatus,
}

#[derive(Debug, Serialize, Deserialize)]
// #[serde(tag = "type", rename_all = "camelCase")]
struct CityInfo {
    id: String,
    // #[serde(rename="type")]
    // Type: String,
    r#type: String,
    word: String,
    cityId: i32,
    cityName: String,
    provinceId: i32,
    provinceName: String,
    countryId: i32,
    countryName: String,
    lat: f32,
    lon: f32,
    gLat: f32,
    gLon: f32,
    gdLat: f32,
    gdLon: f32,
    bdLat: f32,
    bdLon: f32,
    domestic: bool,
    mainland: bool,
    distance: i32,
    cityEName: String,
    countryEName: String,
    timeOffset: i32,
    extendinfo: String,
    issamecity: bool,
    tokens: Vec<String>,
    content: String,
    source: String,
    displayName: String,
    displayType: String,
    eName: String,
    parentName: String,
    matchType: i32,
    resultsType: i32,
    cStar: i32,
    commentScore: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct City {
    citysName: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Province {
    citys: Vec<City>,
    provinceName: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Provinces {
    provinces: Vec<Province>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Req {
    url: String,
    status: u16,
    headers: HashMap<String, String>,
    body: Option<serde_json::Value>,
}

#[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
async fn main() {
    let contents = fs::read_to_string("data/cities.json").expect("读取文件失败。");
    let p: Provinces = serde_json::from_str(&contents).expect("解析数据失败。");
    // 获取国内城市名称
    for province in p.provinces.iter() {
        for pro in province.citys.iter() {
            let city_name = &pro.citysName;
            println!("{}", city_name);
            block_on(get_city_id(city_name));
            sleep(Duration::from_millis(100)).await;
        }
    }
}

async fn get_city_id(city: &String) -> i32 {
    let client = reqwest::Client::new();
    let url = "https://m.ctrip.com/restapi/soa2/21881/json/gaHotelSearchEngine";
    let city_name = &*city;
    let city: &str = &city_name[..];
    let mut data_raw = "{\"keyword\": \"".to_owned();
    data_raw.push_str(city);
    let data_1: &str = "\",\"label\":\"\",\"searchType\":\"D\",\
        \"cityId\":0, \"district\":0,\"province\":0,\"webpSupport\":false,\
        \"platform\":\"online\", \
        \"pageID\":\"102002\",\"head\":{\"Version\":\"\",\"userRegion\":\"CN\",\
        \"Locale\":\"zh-CN\", \
        \"LocaleController\":\"zh-CN\",\"TimeZone\":\"8\",\"Currency\":\"CNY\",\
        \"PageId\":\"102002\", \
        \"webpSupport\":false,\"userIP\":\"\",\"P\":\"76316441318\",\
        \"ticket\":\"\",\"clientID\":\"\", \
        \"group\":\"ctrip\",\"Frontend\":{\"vid\":\"\",\"sessionID\":2,\"pvid\":75},\
        \"Union\":{\"AllianceID\":\"\",\"SID\":\"\",\"Ouid\":\"\"},\
        \"HotelExtension\":{\"group\":\"CTRIP\", \"hasAidInUrl\":false,\
        \"Qid\":\"304674774242\",\"WebpSupport\":false}}} ";
    data_raw.push_str(data_1);
    println!("xxx{}", data_raw);
    let res = client.post(url).body(data_raw).send().await.expect("error");

    // let result_json_raw = format!("{}", res.text_with_charset("utf-8").await.expect("mmm"));
    // println!("{}", result_json_raw);
    let result_json = res.json::<ResponseResult>().await.expect("xxxyyy");

    // println!("{:#?}", res.json::<ResponseResult>().await.expect("xxxyyy"));
    let city_id = result_json.Response.searchResults[0].cityId;

    let _ = save_to_redis(city_name, city_id);
    sleep(Duration::from_millis(100)).await;
    return 1;
}

fn save_to_redis(city: &String, city_id: i32) -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://:Vp8fICsV3P@hotel-spider-redis.teyixing.com/2")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set(city, city_id)?;
    con.get(city)
}
