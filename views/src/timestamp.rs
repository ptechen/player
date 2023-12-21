use chrono::prelude::*;
use yew::prelude::*;
use once_cell::sync::Lazy;
use components::a::A;

use components::input::{Input, OnEvent};
use components::select::Select;
use components::timestamp_interval::TimestampInterval;

pub static TIME_LIST: Lazy<Vec<String>> = Lazy::new(||{
    vec![String::from("秒(s)"), String::from("毫秒(ms)") ]
});

pub static UTC_OR_LOCAL: Lazy<Vec<String>> = Lazy::new(||{
    vec![String::from("北京时间"), String::from("UTC") ]
});

pub enum Msg {
    TimestampToString(AttrValue),
    StringToTimestamp(AttrValue),
    TimeOption(AttrValue),
    UtcOrLocal(AttrValue),
}

pub struct Timestamp {
    timestamp: AttrValue,
    time_str: AttrValue,
    option: AttrValue,
    options: Vec<AttrValue>,
    option_index: AttrValue,
    time_zones: Vec<AttrValue>,
    init_time_zone: AttrValue,
    time_zone_index: AttrValue,
}

impl Component for Timestamp {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{
            timestamp: Default::default(),
            time_str: Default::default(),
            option: AttrValue::from("秒(s)"),
            options: TIME_LIST.iter().map(|a|AttrValue::from(a.as_str())).collect(),
            option_index: AttrValue::from("0"),
            time_zones: UTC_OR_LOCAL.iter().map(|a| AttrValue::from(a.as_str())).collect(),
            init_time_zone: AttrValue::from("北京时间"),
            time_zone_index: AttrValue::from("0")
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TimestampToString(timestamp) => {
                self.timestamp = timestamp;
                self.timestamp_to_string(self.timestamp.to_string());
                true
            },
            Msg::StringToTimestamp(time_str) => {
                self.time_str = time_str;
                self.string_to_timestamp(&self.time_str.to_owned());
                true
            },
            Msg::TimeOption(option) => {
                self.option_index = option;
                self.string_to_timestamp(&self.time_str.to_owned());
                true
            }
            Msg::UtcOrLocal(time_zone_index) => {
                self.time_zone_index = time_zone_index;
                self.timestamp_to_string(self.timestamp.to_string());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let timestamp_to_string = ctx.link().callback(Msg::TimestampToString);
        let string_to_timestamp = ctx.link().callback(Msg::StringToTimestamp);
        let select = ctx.link().callback(Msg::TimeOption);
        let time_zones = ctx.link().callback(Msg::UtcOrLocal);
        html!(
            <div class="timestamp">
                <h2 class="text_center">{"时间戳"}<i class="bi bi-arrow-left-right"></i>{"时间字符串"}</h2>
                <div id="height"></div>
                <TimestampInterval/>
                <div  id="height">
                </div>
                <div class="flex">
                    <div class="flex-1">
                        <button type="button" class="custom-btn btn-9">{"转换"}</button>
                    </div>
                    <div class="flex-1">
                        <Input value={self.timestamp.clone()} placeholder={"时间戳"} event={timestamp_to_string} input_type={"text"} on_event={OnEvent::OnChange}/>
                    </div>
                    <div class="flex-1">
                        <Select option={self.option.clone()} options={self.options.clone()} onchange={select}/>
                        <Select option={self.init_time_zone.clone()} options={self.time_zones.clone()} onchange={time_zones}/>
                    </div>
                    <div class="flex-1">
                        <Input value={self.time_str.clone()} placeholder={"时间字符串"} event={string_to_timestamp} input_type={"text"} on_event={OnEvent::OnChange}/>
                    </div>
                </div>
            </div>
        )
    }
}

impl Timestamp {
    fn string_to_timestamp(&mut self, time_str: &str) {
        if time_str == "" {
            return;
        }
        let date = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S").ok();
        if let Some(date) = date {
            if self.time_zone_index == "0" {
                let date: DateTime<FixedOffset> = DateTime::from_local(date, FixedOffset::east_opt(8 * 3600).unwrap());
                if self.option_index == "0" {
                    self.timestamp = AttrValue::from(date.timestamp().to_string());
                } else if self.option_index == "1" {
                    self.timestamp = AttrValue::from(date.timestamp_millis().to_string());
                }
            } else if self.time_zone_index == "1" {
                let date: DateTime<Utc> = DateTime::from_utc(date, Utc);
                if self.option_index == "0" {
                    self.timestamp = AttrValue::from(date.timestamp().to_string());
                } else if self.option_index == "1" {
                    self.timestamp = AttrValue::from(date.timestamp_millis().to_string());
                }
            }
        }
    }

    fn timestamp_to_string(&mut self, timestamp: String) {
        if timestamp == "" {
            return;
        }
        let time_i64 = timestamp.parse::<i64>().unwrap_or_default();
        if self.time_zone_index == "0" {
            if self.option_index == "0" {
                let date = NaiveDateTime::from_timestamp_opt(time_i64, 0).unwrap_or_default();
                let date: DateTime<FixedOffset> = DateTime::from_utc(date, FixedOffset::east_opt(8 * 3600).unwrap());
                self.time_str = AttrValue::from(date.format("%Y-%m-%d %H:%M:%S").to_string());
            } else if self.option_index == "1" {
                let date = NaiveDateTime::from_timestamp_millis(time_i64).unwrap_or_default();
                let date: DateTime<FixedOffset> = DateTime::from_utc(date, FixedOffset::east_opt(8 * 3600).unwrap());
                self.time_str = AttrValue::from(date.format("%Y-%m-%d %H:%M:%S").to_string());
            }
        } else if self.time_zone_index == "1" {
            if self.option_index == "0" {
                let date = NaiveDateTime::from_timestamp_opt(time_i64, 0).unwrap_or_default();
                let date: DateTime<Utc> = DateTime::from_utc(date, Utc);
                self.time_str = AttrValue::from(date.format("%Y-%m-%d %H:%M:%S").to_string());
            } else if self.option_index == "1" {
                let date = NaiveDateTime::from_timestamp_millis(time_i64).unwrap_or_default();
                let date: DateTime<Utc> = DateTime::from_utc(date, Utc);
                self.time_str = AttrValue::from(date.format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
    }
}