/*
 * Gong API
 *
 * <h2>Overview</h2> <p> The Gong API allows you to: </p> <ol> <li> Receive the following information from Gong: <ol type=\"a\"> <li> Your company's <a href=\"#tag--Calls\">calls</a> in Gong </li> <li> Your company's <a href=\"#tag--Users\">users</a> in Gong </li> <li> Your company's user <a href=\"#tag--Stats\">stats</a> in Gong </li> <li> Your company's user <a href=\"#tag--Settings\">settings</a> in Gong </li> <li> Your company's <a href=\"#tag--Library\">libraries</a> in Gong </li> </ol></li> <li> <a href=\"#post-/v2/calls\">Upload</a> new or  <a href=\"#put-/v2/calls/-id-/media\">update</a>  call recordings in Gong, in order to support cases where you have an internal system that records  calls or obtains them from a third-party entity. </li> <li> <a href=\"#post-/v2/data-privacy/erase-data-for-email-address\">Data Privacy</a>:  Delete users and all their associated elements.</li> <li> Upload <a href=\"#tag--CRM\">CRM</a> data into Gong.  </li> </ol> <p>Check <a href=\"https://app.gong.io/company/api-authentication?currentTab=MY_API_TAB\">here</a> what's your base URL for all API calls. </p> <h2>Authentication</h2>  <p> There are two ways to retrieve credentials to the Gong Public API: </p> <ol><li>Retrieve Manually:<br> <p> In the <a href=\"https://app.gong.io/company/api\">Gong API Page</a> (you must be a technical administrator in Gong), click \"Create\" to receive an <b>Access Key</b>  and an <b>Access Key Secret</b>.<br> </p> <p> Use the Basic Authorization HTTP header (as per <a target=\"_blank\" href=\"https://www.rfc-editor.org/rfc/rfc7617.txt\">RFC</a>) to access the Public API as shown below:<br> <code>Authorization: Basic &lt;token&gt;</code><br> </p> <p> To create the basic token, combine the <b>Access Key</b> and the <b>Access Key Secret</b> with  colon (:) and then encode in Base64 as following:<br> <code>Base64(&lt;accessKey&gt; : &lt;accessKeySecret&gt;)</code><br><br> </p></li> <li>Retrieve through OAuth<br> <p> To obtain the Bearer token, follow the steps described in the <a target=\"_blank\" href=\"https://help.gong.io/hc/en-us/articles/13944551222157-Create-an-app-for-Gong\">Gong OAuth Guide</a>. <br></p> <p> After obtaining the token, use the Bearer Authorization HTTP header (as per <a target=\"_blank\" href=\"https://www.rfc-editor.org/rfc/rfc6750.txt\">RFC</a>) to access the Public API as shown below:<br> <code>Authorization: Bearer &lt;token&gt;</code> </p> </li></ol> <h2>Limits</h2>  <p> By default Gong limits your company's access to the service to 3 API calls per second, and 10,000 API calls per day. </p> <p> When the rate of API calls exceeds these limits an HTTP status code <b>429</b> is returned and a <b>Retry-After</b> header indicates  how many seconds to wait before making a new request. </p><p> If required, contact <a target=\"_blank\" href=\"https://help.gong.io\">help.gong.io</a> to change these limits. </p>  <h2>Cursors</h2>  <p> Some API calls that return a list are limited in the amount of records they may return, so multiple API calls may be  required to bring all records. Such an API call also returns a <b>records</b> field, which contains the number of records  in the current page, the current page number and the total number of records. </p> <p> In cases where the total number of records exceeds the number of records thus far retrieved, the <b>records</b> field will also  contain a <b>cursor</b> field which can be used to access the next page of records. To retrieve the next page, repeat the API call with  the <b>cursor</b> value as supplied by the previous API call. All other request inputs should remain the same. </p> <h2>Forward Compatibility</h2>  <p> When coding a system to accept Gong data, take into account that Gong may, without prior warning, add fields to the JSON output.  It is recommended to future proof your code so that it disregards all JSON fields you don't actually use.  </p><p></p>
 *
 * The version of the OpenAPI document: V2
 * Contact: mail@cedric-ziel.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

/// struct for passing parameters to the method [`list_users1`]
#[derive(Clone, Debug)]
pub struct ListUsers1Params {
    pub workspace_id: i64,
    pub manager_id: i64,
    /// Association time filter - only the associations made after that time will be listed. The time is in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC); if not specified all association events will be listed.
    pub from: String,
    /// Association time filter - only the associations made after that time will be listed. The time is in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC); if not specified all association events will be listed.
    pub to: String
}


/// struct for typed errors of method [`list_users1`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListUsers1Error {
    Status400(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// List all of the coaching metrics of a manager.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:coaching:read'.
pub async fn list_users1(configuration: &configuration::Configuration, params: ListUsers1Params) -> Result<models::CoachingMetrics, Error<ListUsers1Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let workspace_id = params.workspace_id;
    let manager_id = params.manager_id;
    let from = params.from;
    let to = params.to;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/coaching", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("workspace-id", &workspace_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("manager-id", &manager_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("from", &from.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("to", &to.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListUsers1Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

