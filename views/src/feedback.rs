use yew::platform::spawn_local;
use yew::prelude::*;
use crate::request::{HttpRequest, HttpResponse};
use gloo_storage::Storage;
use serde::{Deserialize, Serialize};

const FEEDBACK_LIST: &'static str = "/api/teamup/feedback/record_list";


pub enum Msg {
    Init(FeedbackRecordListRes),
    NextPage,
    PrePage,
}

#[derive(Serialize)]
pub struct Feedback{
    pub page_no: u64,
    pub page_size: u64,
    #[serde(skip)]
    pub res: Option<FeedbackRecordListRes>,
}


impl Component for Feedback {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self{ page_no: 1, page_size: 100, res: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Init(data) => {
                self.res = Some(data);
                true
            }
            Msg::NextPage => {
                self.page_no += 1;
                let call_back = ctx.link().callback(|msg| Msg::Init(msg));
                self.feedback_list(call_back);
                false
            }
            Msg::PrePage => {
                if self.page_no > 1 {
                    self.page_no -= 1;
                    let call_back = ctx.link().callback(|msg| Msg::Init(msg));
                    self.feedback_list(call_back);
                }
                false
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let call_back = ctx.link().callback(|msg| Msg::Init(msg));
            self.feedback_list(call_back);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_next = ctx.link().callback(|_|Msg::NextPage);
        let onclick_pre = ctx.link().callback(|_|Msg::PrePage);
        if let Some(res) = self.res.clone(){
            html!(
                <>
                <nav aria-label="Page navigation example">
                    <ul class="pagination">
                        <li class="page-item"><a class="page-link" onclick={{onclick_pre.clone()}}>{{"Previous"}}</a></li>
                        <li class="page-item"><a class="page-link" onclick={{onclick_next.clone()}}>{{"Next"}}</a></li>
                    </ul>
                </nav>
                {res.to_html()}
                <nav aria-label="Page navigation example">
                    <ul class="pagination">
                        <li class="page-item"><a class="page-link" onclick={{onclick_pre}}>{{"Previous"}}</a></li>
                        <li class="page-item"><a class="page-link" onclick={{onclick_next}}>{{"Next"}}</a></li>
                    </ul>
                </nav>
                </>
            )
        } else {
            html!(
            )
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeedbackRecordListRes {
    pub list: Vec<FeedbackRecordInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FeedbackRecordInfo {
    /// 反馈记录id  field_type: int unsigned
    pub id: u32,
    /// 反馈类型  field_type: varchar(16)  default: ''
    pub feedback_type: serde_json::Value,
    /// 反馈描述  field_type: varchar(128) default: 0
    pub feedback_desc: String,
    /// 图片地址  field_type: json
    pub img_urls: serde_json::Value,
    /// 电话  field_type: varchar(16)  default: ''
    pub phone: String,
    /// QQ  field_type: varchar(16)  default: ''
    pub qq: String,
    /// 邮箱  field_type: varchar(32)  default: ''
    pub email: String,
    /// 日志url  field_type: varchar(128)  default: ''
    pub log_url: String,
    /// 用户id
    pub user_id: String,
    /// 昵称
    pub nickname: String,
    /// 头像
    pub avatar: String,
}

impl FeedbackRecordInfo {
    pub fn to_html(&self) -> Html {
        html!(
             <tr>
                <th scope="row">{{self.id}}</th>
                <td><span><img class="figure-img img-fluid rounded" src={{self.avatar.clone()}}/></span><a>{{self.nickname.clone()}}</a></td>
                <td>{{self.feedback_type.to_string()}}</td>
                <td>{{self.feedback_desc.clone()}}</td>
                <td>{{self.images()}}</td>
                <td>{{self.phone.clone()}}</td>
                <td>{{self.qq.clone()}}</td>
                <td>{{self.email.clone()}}</td>
                <td><a href={{self.log_url.clone()}}>{{"下载"}}</a></td>
              </tr>
        )
    }

    pub fn images(&self) -> Html {
        let img_urls:Vec<String> = serde_json::from_value(self.img_urls.clone()).unwrap_or_default();
        html!(
            <div class="row">
            {
                for img_urls.iter().map(|item|{
                    html!(
                        <a href={{item.clone()}}>{{"下载"}}</a>
                    )
                })
            }
            </div>
        )
    }
}

impl FeedbackRecordListRes {
    pub fn to_html(&self) -> Html {
        html!(
            <table class="table table-bordered">
            <thead>
                <tr>
                    <th scope="col">{{"id"}}</th>
                    <th scope="col">{{"用户"}}</th>
                    <th scope="col">{{"类型"}}</th>
                    <th scope="col">{{"描述"}}</th>
                    <th scope="col">{{"图片"}}</th>
                    <th scope="col">{{"电话"}}</th>
                    <th scope="col">{{"QQ"}}</th>
                    <th scope="col">{{"邮箱"}}</th>
                    <th scope="col">{{"日志"}}</th>
                </tr>
            </thead>
            <tbody>
            {
                for self.list.iter().map(|item|{
                    item.to_html()
                })
            }
            </tbody>
            </table>
        )
    }
}

impl Feedback {
    pub fn feedback_list(&self, callback: Callback<FeedbackRecordListRes>) {
        let host: String = gloo_storage::SessionStorage::get("current_env").unwrap_or_default();
        let params = serde_json::to_string(&self).unwrap_or_default();
        spawn_local(async move {
            let res: HttpResponse<FeedbackRecordListRes> = HttpRequest::post(params, &(host + FEEDBACK_LIST)).await.unwrap();
            let res = res.data.unwrap_or_default();
            callback.emit(res);
        });
    }
}
