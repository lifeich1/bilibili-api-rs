use crate::api_info::GetFromPath;
use lazy_static::lazy_static;
use serde_json::json;

lazy_static! {
    // Copy from https://github.com/Passkou/bilibili-api
    static ref DATA: serde_json::Value = json!({
        "info": {
            "stat": {
                "url": "https://api.bilibili.com/x/web-interface/archive/stat",
                "method": "GET",
                "verify": false,
                "params": {
                    "aid": "av号"
                },
                "comment": "视频数据"
            },
            "detail": {
                "url": "https://api.bilibili.com/x/web-interface/view",
                "method": "GET",
                "verify": false,
                "params": {
                    "aid": "av号"
                },
                "comment": "视频详细信息"
            },
            "tags": {
                "url": "https://api.bilibili.com/x/tag/archive/tags",
                "method": "GET",
                "verify": true,
                "params": {
                    "aid": "av号"
                },
                "comment": "视频标签信息"
            },
            "chargers": {
                "url": "https://api.bilibili.com/x/web-interface/elec/show",
                "method": "GET",
                "verify": false,
                "params": {
                    "aid": "av号",
                    "mid": "用户UID"
                },
                "comment": "视频充电信息"
            },
            "pages": {
                "url": "https://api.bilibili.com/x/player/pagelist",
                "method": "GET",
                "verify": false,
                "params": {
                    "aid": "av号"
                },
                "comment": "分P列表"
            },
            "playurl": {
                "url": "https://api.bilibili.com/x/player/playurl",
                "method": "GET",
                "verify": false,
                "params": {
                    "avid": "av号",
                    "cid": "分P编号",
                    "qn": "视频质量编号，最高120",
                    "otype": "json",
                    "fnval": 16
                },
                "comment": "视频下载的信息，下载链接需要提供headers伪装浏览器请求（Referer和User-Agent）"
            },
            "related": {
                "url": "https://api.bilibili.com/x/web-interface/archive/related",
                "method": "GET",
                "verify": false,
                "params": {
                    "aid": "av号"
                },
                "comment": "获取关联视频"
            },
            "has_liked": {
                "url": "https://api.bilibili.com/x/web-interface/archive/has/like",
                "method": "GET",
                "verify": true,
                "params": {
                    "aid": "av号"
                },
                "comment": "是否已点赞"
            },
            "get_pay_coins": {
                "url": "https://api.bilibili.com/x/web-interface/archive/coins",
                "method": "GET",
                "verify": true,
                "params": {
                    "aid": "av号"
                },
                "comment": "是否已投币 "
            },
            "has_favoured": {
                "url": "https://api.bilibili.com/x/v2/fav/video/favoured",
                "method": "GET",
                "verify": true,
                "params": {
                    "aid": "av号"
                },
                "comment": "是否已收藏"
            },
            "media_list": {
                "url": "https://api.bilibili.com/x/v3/fav/folder/created/list-all",
                "method": "GET",
                "verify": true,
                "params": {
                    "rid": "av号",
                    "up_mid": "up的uid",
                    "type": 2
                },
                "comment": "获取收藏夹列表信息，用于收藏操作"
            }
        },
        "operate": {
            "like": {
                "url": "https://api.bilibili.com/x/web-interface/archive/like",
                "method": "POST",
                "verify": true,
                "data": {
                    "aid": "av号",
                    "like": "1是点赞，2是取消点赞"
                },
                "comment": "给视频点赞/取消点赞 "
            },
            "coin": {
                "url": "https://api.bilibili.com/x/web-interface/coin/add",
                "method": "POST",
                "verify": true,
                "data": {
                    "aid": "av号",
                    "multiply": "几个币",
                    "select_like": "同时点赞（1是0否）"
                },
                "comment": "给视频投币"
            },
            "add_tag": {
                "url": "https://api.bilibili.com/x/tag/archive/add",
                "method": "POST",
                "verify": true,
                "data": {
                    "aid": "av号",
                    "tag_name": "标签名"
                },
                "comment": "添加标签"
            },
            "del_tag": {
                "url": "https://api.bilibili.com/x/tag/archive/del",
                "method": "POST",
                "verify": true,
                "data": {
                    "aid": "av号",
                    "tag_id": "标签id"
                },
                "comment": "删除标签"
            },
            "subscribe_tag": {
                "url": "https://api.bilibili.com/x/tag/subscribe/add",
                "method": "POST",
                "verify": true,
                "data": {
                    "tag_id": "标签id"
                },
                "comment": "订阅标签"
            },
            "unsubscribe_tag": {
                "url": "https://api.bilibili.com/x/tag/subscribe/cancel",
                "method": "POST",
                "verify": true,
                "data": {
                    "tag_id": "标签id"
                },
                "comment": "取消订阅标签"
            },
            "favorite": {
                "url": "https://api.bilibili.com/x/v3/fav/resource/deal",
                "method": "POST",
                "verify": true,
                "data": {
                    "rid": "int: av 号。",
                    "type": "int: 类型。目前为 2。",
                    "add_media_ids": "commaList[int]: 要添加到的收藏夹 ID。",
                    "del_media_ids": "commaList[int]: 要移出的收藏夹 ID。"
                },
                "comment": "设置视频收藏状态"
            }
        },
        "danmaku": {
            "get_danmaku": {
                "url": "https://api.bilibili.com/x/v2/dm/web/seg.so",
                "method": "GET",
                "verify": false,
                "params": {
                    "oid": "video_info中的cid，即分P的编号",
                    "type": 1,
                    "segment_index": "分片序号",
                    "pid": "av号"
                },
                "comment": "获取弹幕列表"
            },
            "get_history_danmaku": {
                "url": "https://api.bilibili.com/x/v2/dm/web/history/seg.so",
                "method": "GET",
                "verify": true,
                "params": {
                    "oid": "video_info中的cid，即分P的编号",
                    "type": 1,
                    "date": "历史弹幕日期，格式：YYYY-MM-DD"
                },
                "comment": "获取历史弹幕列表"
            },
            "view": {
                "url": "https://api.bilibili.com/x/v2/dm/web/view",
                "method": "GET",
                "verify": false,
                "params": {
                    "type": 1,
                    "oid": "分Pid",
                    "pid": "av号"
                },
                "comment": "获取弹幕设置、特殊弹幕"
            },
            "get_history_danmaku_index": {
                "url": "https://api.bilibili.com/x/v2/dm/history/index",
                "method": "GET",
                "verify": true,
                "params": {
                    "oid": "分P的编号",
                    "type": "1",
                    "month": "年月 (yyyy-mm)"
                },
                "comment": "存在历史弹幕的日期"
            },
            "has_liked_danmaku": {
                "url": "https://api.bilibili.com/x/v2/dm/thumbup/stats",
                "method": "GET",
                "verify": true,
                "params": {
                    "oid": "video_info中的cid，即分P的编号",
                    "ids": "弹幕id，多个以逗号分隔"
                },
                "comment": "是否已点赞弹幕"
            },
            "send_danmaku": {
                "url": "https://api.bilibili.com/x/v2/dm/post",
                "method": "POST",
                "verify": true,
                "data": {
                    "type": "1",
                    "oid": "分P编号",
                    "msg": "弹幕内容",
                    "bvid": "bvid",
                    "progress": "发送时间（毫秒）",
                    "color": "颜色（十六进制转十进制）",
                    "fontsize": "字体大小（小18普通25大36）",
                    "pool": "字幕弹幕（1是0否）",
                    "mode": "模式（滚动1顶部5底部4）",
                    "plat": "1"
                },
                "comment": "发送弹幕"
            },
            "like_danmaku": {
                "url": "https://api.bilibili.com/x/v2/dm/thumbup/add",
                "method": "POST",
                "verify": true,
                "data": {
                    "dmid": "弹幕ID",
                    "oid": "分P编号",
                    "op": "1点赞2取消点赞",
                    "platform": "web_player"
                },
                "comment": "点赞弹幕"
            }
        }
    });
}

pub fn get(path: &str) -> &serde_json::Value {
    DATA.get_from_path(path)
}
