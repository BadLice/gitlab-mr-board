use std::rc::Rc;

use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize)]
pub struct MergeRequest {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub web_url: String,
    pub source_branch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
}

pub struct GitLab {
    token: String,
    base_url: String,
    current_user: Option<User>,
}
impl GitLab {
    pub async fn new(host: String, token: String) -> Result<Self, JsValue> {
        let mut instance = Self {
            base_url: format!("https://{}/api/v4", host),
            token,
            current_user: None,
        };
        instance.current_user = Some(instance.fetch_get::<User>("/user").await?);
        Ok(instance)
    }

    async fn fetch_get<T: for<'de> serde::Deserialize<'de>>(
        &self,
        url: &str,
    ) -> Result<T, JsValue> {
        let window = web_sys::window().expect("no global `window` exists");
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);
        let request =
            Request::new_with_str_and_init(format!("{}{}", self.base_url, url).as_str(), &opts)?;
        request.headers().set("Accept", "application/json")?;
        request
            .headers()
            .set("Authorization", format!("Bearer {}", self.token).as_str())?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();
        let json = JsFuture::from(resp.json()?).await?;
        let result_data: T = serde_wasm_bindgen::from_value(json)?;
        Ok(result_data)
    }
    pub async fn get_mr_draft(&self) -> Result<Vec<MergeRequest>, JsValue> {
        match &Rc::new(self).current_user {
            Some(current_user) => {
                let mr_info = self
                    .fetch_get::<Vec<MergeRequest>>(
                        format!(
                            "/merge_requests?scope=all&state=opened&wip=no&author_id={}",
                            &current_user.id
                        )
                        .as_str(),
                    )
                    .await?;
                Ok(mr_info)
            }
            None => Err(JsValue::null()),
        }
    }
    async fn get_mr_reviewed_by_me(&self) -> Result<Vec<MergeRequest>, JsValue> {
        match &Rc::new(self).current_user {
            Some(current_user) => {
                let mr_info = self
                    .fetch_get::<Vec<MergeRequest>>(
                        format!("/merge_requests?scope=all&state=opened&reviewer_id={}&not[author_id]={}",&current_user.id,&current_user.id).as_str(),
                    )
                    .await?;
                Ok(mr_info)
            }
            None => Err(JsValue::null()),
        }
    }
    async fn get_mr_assigned_to_me(&self) -> Result<Vec<MergeRequest>, JsValue> {
        match &Rc::new(self).current_user {
            Some(current_user) => {
                let mr_info = self
                    .fetch_get::<Vec<MergeRequest>>(
                        format!("/merge_requests?scope=all&state=opened&assignee_id={}&not[author_id]={}",&current_user.id,&current_user.id).as_str(),
                    )
                    .await?;
                Ok(mr_info)
            }
            None => Err(JsValue::null()),
        }
    }
    async fn get_mr_mine_ready_assigned_to_someone_else(
        &self,
    ) -> Result<Vec<MergeRequest>, JsValue> {
        match &Rc::new(self).current_user {
            Some(current_user) => {
                let mr_info = self
                    .fetch_get::<Vec<MergeRequest>>(
                        format!("/merge_requests?scope=all&state=opened&author_id={}&not[assignee_id]={}&wip=no",&current_user.id,&current_user.id).as_str(),
                    )
                    .await?;
                Ok(mr_info)
            }
            None => Err(JsValue::null()),
        }
    }
    async fn get_mr_mine_ready_reviewed_by_someone_else(
        &self,
    ) -> Result<Vec<MergeRequest>, JsValue> {
        match &Rc::new(self).current_user {
            Some(current_user) => {
                let mr_info = self
                    .fetch_get::<Vec<MergeRequest>>(
                        format!("/merge_requests?scope=all&state=opened&author_id={}&reviewer_id=any&wip=no",&current_user.id).as_str(),
                    )
                    .await?;
                Ok(mr_info)
            }
            None => Err(JsValue::null()),
        }
    }
    pub async fn get_mr_to_review(&self) -> Result<Vec<MergeRequest>, JsValue> {
        let mut assigned = self.get_mr_assigned_to_me().await?;
        let mut reviewed = self.get_mr_reviewed_by_me().await?;
        assigned.append(&mut reviewed);
        Ok(assigned)
    }
    pub async fn get_mr_under_review(&self) -> Result<Vec<MergeRequest>, JsValue> {
        let mut assigned = self.get_mr_mine_ready_assigned_to_someone_else().await?;
        let mut reviewed = self.get_mr_mine_ready_reviewed_by_someone_else().await?;
        assigned.append(&mut reviewed);
        Ok(assigned)
    }
}
