use crate::api::ApiRequest;
use crate::api_info;
use crate::error::ApiResult;
use crate::Context;

use super::ApiRequestBuilder;

/// Live related neXt generation APIs collection
pub struct Xlive {
    ctx: Context,
}

pub enum PAreaID {
    SingleGame,
    Vup,
}

impl PAreaID {
    pub fn as_i32(&self) -> i32 {
        match self {
            Self::SingleGame => 6,
            Self::Vup => 9,
        }
    }
}

pub enum AreaID {
    All,
    HaloInfinite,
    ConsoleGame,
    OtherSingleGame,
}

impl AreaID {
    pub fn as_i32(&self) -> i32 {
        match self {
            Self::All => 0,
            Self::OtherSingleGame => 235,
            Self::ConsoleGame => 236,
            Self::HaloInfinite => 559,
        }
    }
}

pub enum ListSortType {
    Entropy,
    Latest,
}

impl ListSortType {
    pub fn as_str(&self) -> String {
        String::from(match self {
            Self::Entropy => "sort_type_291",
            Self::Latest => "live_time",
        })
    }
}

impl Xlive {
    pub fn new(n: &Context) -> Self {
        Self { ctx: n.clone() }
    }

    fn rb(&self) -> ApiRequestBuilder {
        self.ctx.req_build().api(api_info::xlive::get)
    }

    pub fn get_list(
        &self,
        p_area: PAreaID,
        area: AreaID,
        stype: ListSortType,
        page: i32,
    ) -> ApiResult<ApiRequest> {
        let pn = page.to_string();
        let pid = p_area.as_i32().to_string();
        let aid = area.as_i32().to_string();
        let sty = stype.as_str();
        self.rb()
            .path("info/get_list")
            .query(&[("platform", "web")])
            .query(&[
                ("sort_type", &sty),
                ("parent_area_id", &pid),
                ("area_id", &aid),
                ("page", &pn),
            ])
            .nobuffer()
    }
}
