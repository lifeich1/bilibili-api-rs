#!/usr/bin/env bash

# good api
curl -v -A "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36" \
'https://api.live.bilibili.com/xlive/web-room/v1/index/getInfoByRoom?room_id=6655'

#curl -v -A "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36" \
#  'https://search.bilibili.com/live?keyword=ywwuyi&from_source=webtop_search&spm_id_from=333.999&search_source=5'

#curl -v -A "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36" \
  #'https://live.bilibili.com/22505271'

#curl -v -A "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36" \
#  'https://live.bilibili.com/22505271?broadcast_type=0&is_room_feed=0&spm_id_from=333.999.to_liveroom.0.click&live_from=86002'

#curl -v -A "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36" \
#  https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id=6655

#curl -v -A "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36" \
#  https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo?room_id=22505271&protocol=0,1&format=0,1,2&codec=0,1,2&qn=0&platform=web&ptype=8&dolby=5&panorama=1&hdr_type=0,1

#curl -v -A "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36" \
  #'https://api.bilibili.com/x/web-interface/card?mid=1472906636&photo=1'


ttt() {
  #-H 'accept-encoding: gzip, deflate, br, zstd' \
curl -v -A "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36" \
  -H "Referer: https://space.bilibili.com/1343403" \
  -H "Origin: https://space.bilibili.com" \
  -H 'accept-language: zh,en-US;q=0.9,en;q=0.8,zh-CN;q=0.7' \
  -H 'priority: u=1, i' \
  -H 'sec-ch-ua: "Chromium";v="128", "Not;A=Brand";v="24", "Google Chrome";v="128"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'sec-ch-ua-platform: "Linux"' \
  -H 'sec-fetch-dest: empty' \
  -H 'sec-fetch-mode: cors' \
  -H 'sec-fetch-site: same-site' \
  --cookie "SESSDATA=" \
  --cookie "buvid3=B3ECA569-DA09-E82F-5F37-10D23EBD69B095632infoc" \
  --cookie "b_nut=1715502795" \
  --cookie "_uuid=3A21031019-48CB-810610-1C61-E8556185241A96229infoc" \
  --cookie "buvid4=C247ED4F-9CB7-ACD7-A2A4-F383AB72CA6A96236-024051208-8Dw5L6xrmba7x3ZS2k1tUw%3D%3D" \
  --cookie "LIVE_BUVID=AUTO7217156142463358" \
  --cookie "buvid_fp_plain=undefined" \
  --cookie "bili_ticket=eyJhbGciOiJIUzI1NiIsImtpZCI6InMwMyIsInR5cCI6IkpXVCJ9.eyJleHAiOjE3MjgzNjk1MzgsImlhdCI6MTcyODExMDI3OCwicGx0IjotMX0.ecSaJEh7-gsbE2oWRzqkJHPcCCYz7foNp2Co5ZJovNw" \
  --cookie "bili_ticket_expires=1728369478" \
  --cookie "buvid_fp=d1d39f7ed0ba33364e5e431a2e783c63" \
  --cookie "b_lsid=72EF22C9_192677142DE" \
  --cookie "sid=6t74c9bu" \
  --cookie "PVID=4" \
  --cookie "blackside_state=1" \
  --cookie "CURRENT_BLACKGAP=1" \
  --cookie "CURRENT_FNVAL=4048" \
  --cookie "rpdid=|(J|)J)~|klJ0J'u~ull|k~mY" \
  --cookie "hit-dyn-v2=1" \
  --cookie "enable_web_push=ENABLE" \
  --cookie "iflogin_when_web_push=0" \
  --cookie "home_feed_column=5" \
  --cookie "header_theme_version=CLOSE" \
  --cookie "is-2022-channel=1" \
  --cookie "browser_resolution=1920-923" \
  --cookie "fingerprint=d1d39f7ed0ba33364e5e431a2e783c63" \
  --cookie "opus-goback=1" \
  --cookie "bsource=search_baidu" \
  --url-query "mid=1343403" \
  --url-query "token=" \
  --url-query "platform=web" \
  --url-query "web_location=1550101" \
  --url-query "dm_img_list=[]" \
  --url-query "dm_img_str=V2ViR0wgMS4wIChPcGVuR0wgRVMgMi4wIENocm9taXVtKQ" \
  --url-query "dm_cover_img_str=QU5HTEUgKEFNRCwgQU1EIFJhZGVvbiA3ODBNIChyYWRlb25zaSBnZngxMTAzX3IxIExMVk0gMTguMS44KSwgT3BlbkdMIDQuNilHb29nbGUgSW5jLiAoQU1EKQ" \
  --url-query "dm_img_inter=%7B%22ds%22:[],%22wh%22:[4434,3943,62],%22of%22:[89,178,89]%7D" \
  --url-query "w_webid=eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzcG1faWQiOiIwLjAiLCJidXZpZCI6IkIzRUNBNTY5LURBMDktRTgyRi01RjM3LTEwRDIzRUJENjlCMDk1NjMyaW5mb2MiLCJ1c2VyX2FnZW50IjoiTW96aWxsYS81LjAgKFgxMTsgTGludXggeDg2XzY0KSBBcHBsZVdlYktpdC81MzcuMzYgKEtIVE1MLCBsaWtlIEdlY2tvKSBDaHJvbWUvMTI4LjAuMC4wIFNhZmFyaS81MzcuMzYiLCJidXZpZF9mcCI6ImQxZDM5ZjdlZDBiYTMzMzY0ZTVlNDMxYTJlNzgzYzYzIiwiY3JlYXRlZF9hdCI6MTcyODMxNTk5NiwidHRsIjo4NjQwMCwidXJsIjoiLzEzNDM0MDMiLCJyZXN1bHQiOiJub3JtYWwiLCJpc3MiOiJnYWlhIiwiaWF0IjoxNzI4MzE1OTk2fQ.hXEl4MSOa-9xiliHn7dAtdMrrGjMwat9XILYuXVMSM-yCvMjA67jNN0SaK79riDOPXQ35J0upXmFeHkKzNLKWxAk92re1cdqO-O4Cfa_5e6FmpxC1JkGcnquk1HaIlpYn-PiL_e-GdN6LbUUmBFh2ow-UtrjixKZWNkqbwksiKpanR4TMNVBq-YA3gg2mGevNx0bLa1CK0BySZp401hDVGK3_PFD9OUgfkSfpm6Cf62Io8dkE4ngclZA8InmSe_ehKllwjhucgqVTtc0cby8ACBdUpx5M8Qet8N2PJPjGO23LHja5jA2Ml1AH1nSvmIYLS2HI8mWGPKp7HETA5KlIQ" \
  --url-query "w_rid=60bdc4d57d8c74be79dd6a10faf10280&wts=1728315997" \
  --get 'https://api.bilibili.com/x/space/wbi/acc/info'
}
