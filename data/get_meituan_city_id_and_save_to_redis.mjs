#!/usr/bin/env zx

let cities = await $`cat cities.json`
let cities_obj = JSON.parse(cities)

for (let i = 0; i < cities_obj['provinces'].length; i++) {
  for (let j = 0; j < cities_obj['provinces'][i]['citys'].length; j++){
    let city = cities_obj['provinces'][i]['citys'][j]['citysName']
    let request_url = 'https://ihotel.meituan.com/group/v1/area/search/shenzhen'
    let get_city_info_response = await fetch('https://ihotel.meituan.com/group/v1/area/search/' + encodeURI(String(city)))
    if (get_city_info_response.ok) {
      // console.log('xxxx', await get_city_info_response.json())
      let city_info = await get_city_info_response.json()
      for (let x = 0; x < city_info['data'].length; x++ ) {
        let store_key = 'meituan_' + city_info['data'][x]['cityName']
        let store_value = city_info['data'][x]['cityId']
        await $`redis-cli -h hotel-spider-redis.teyixing.com -a Vp8fICsV3P -c set ${store_key} ${store_value}`
      }
    }
  }
}
