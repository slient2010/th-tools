#!/usr/bin/env zx

let response = await fetch('https://i.meituan.com/awp/hfe/fep/1a368b1d945fd175cc40c859d172350d.json')
if (response.ok) {
  let data = await response.json()
  let city_id = data['data'].map(x => ( x.id +"_" + x.name))

  for (let i = 0;  i < city_id.length; i++) {
    let value = await $`echo ${city_id[i]} | awk -F"_" '{print $1}'`
    let key = await $`echo ${city_id[i]} | awk -F"_" '{print $2}'`
    let store_key = "meituan_" + key.stdout.split('\n')[0]
    let store_value = value.stdout.split('\n')[0]
    // await $`redis-cli -h hotel-spider-redis.teyixing.com -a Vp8fICsV3P -c set ${store_key} ${store_value}`
  }
}
