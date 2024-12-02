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

/// struct for passing parameters to the method [`delete_generic_crm_integration`]
#[derive(Clone, Debug)]
pub struct DeleteGenericCrmIntegrationParams {
    /// The ID of the integration to delete
    pub integration_id: i64,
    /// A unique identifier generated and sent by you to allow troubleshooting. Valid characters are letters, numbers, dashes and underscores.
    pub client_request_id: String
}

/// struct for passing parameters to the method [`get_crm_objects`]
#[derive(Clone, Debug)]
pub struct GetCrmObjectsParams {
    /// Integration ID generated when creating the integration
    pub integration_id: i64,
    /// Requested objects type
    pub object_type: String,
    /// <b>Request Body:</b> The requested objects crm ids (should be sent in the request body)
    pub objects_crm_ids: Vec<String>
}

/// struct for passing parameters to the method [`get_request_status`]
#[derive(Clone, Debug)]
pub struct GetRequestStatusParams {
    /// Integration ID generated when creating the integration
    pub integration_id: i64,
    /// The client request ID used in the asynchronous endpoint you want to get a status for
    pub client_request_id: String
}

/// struct for passing parameters to the method [`list_crm_schema_fields`]
#[derive(Clone, Debug)]
pub struct ListCrmSchemaFieldsParams {
    /// Integration ID generated when creating the integration
    pub integration_id: i64,
    /// Type of object to retrieve the schema fields for (case-sensitive). <br>Omitting this parameter returns the schema for all object types.
    pub object_type: String
}

/// struct for passing parameters to the method [`register_generic_crm_integration`]
#[derive(Clone, Debug)]
pub struct RegisterGenericCrmIntegrationParams {
    pub generic_crm_registration_request: models::GenericCrmRegistrationRequest
}

/// struct for passing parameters to the method [`upload_crm_schema_field`]
#[derive(Clone, Debug)]
pub struct UploadCrmSchemaFieldParams {
    /// Integration ID generated when creating the integration
    pub integration_id: i64,
    /// The object type to set the schema for (case-sensitive)
    pub object_type: String,
    pub generic_schema_field_request: Vec<models::GenericSchemaFieldRequest>
}


/// struct for typed errors of method [`delete_generic_crm_integration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGenericCrmIntegrationError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_crm_objects`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCrmObjectsError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_request_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRequestStatusError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_crm_schema_fields`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCrmSchemaFieldsError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_generic_crm_integration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListGenericCrmIntegrationError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`register_generic_crm_integration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegisterGenericCrmIntegrationError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_crm_schema_field`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadCrmSchemaFieldError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// <style>.public-api-info {    background: rgb(222, 235, 255);}.public-api-tip {    background: rgb(227, 252, 239);}.public-api-parameter {    background: rgba(9,30,66,0.08);}.public-api-note {    background: rgb(234, 230, 255);}.public-api-important {    background: rgb(255, 250, 230);}.public-api-critical {    background: rgb(255, 235, 230);}table, th, td {  border: 1px solid gray;  border-collapse: collapse;}th, td {  padding: 5px;}th {  text-align: left;}img {  border: 2px solid #D3D5D9;}</style><p>Asynchronously deletes a CRM integration and all its associated CRM objects (Accounts, Contacts, Deals, Leads, and Users).</p><p>This endpoint gets the clientRequestId generated by you as the request identifier. Use this to check the status of the delete request by calling the <code>/request-status</code> endpoint,with the clientRequestId. </p><p>A status of DONE indicates that the integration and all its associated crm objects have been successfully deleted. This may take up to 24 hours to be deleted.</p><br>When accessed using a bearer token, this endpoint requires the 'api:crm:integration:delete' scope.<h2>Example</h2><code>DELETE https://api.gong.io/v2/crm/integrations?clientRequestId=1234&integrationId=6286478263646</code>
pub async fn delete_generic_crm_integration(configuration: &configuration::Configuration, params: DeleteGenericCrmIntegrationParams) -> Result<models::AsyncProcessingResponse, Error<DeleteGenericCrmIntegrationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let integration_id = params.integration_id;
    let client_request_id = params.client_request_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/crm/integrations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("integrationId", &integration_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("clientRequestId", &client_request_id.to_string())]);
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
        let local_var_entity: Option<DeleteGenericCrmIntegrationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// <style>.public-api-info {    background: rgb(222, 235, 255);}.public-api-tip {    background: rgb(227, 252, 239);}.public-api-parameter {    background: rgba(9,30,66,0.08);}.public-api-note {    background: rgb(234, 230, 255);}.public-api-important {    background: rgb(255, 250, 230);}.public-api-critical {    background: rgb(255, 235, 230);}table, th, td {  border: 1px solid gray;  border-collapse: collapse;}th, td {  padding: 5px;}th {  text-align: left;}img {  border: 2px solid #D3D5D9;}</style><h2>This API is for use in the <strong>development phase only</strong>, to manually verify that objects are uploaded and processed correctly in Gong.</h2><p>Returns a JSON object where each key is the object’s crm id and the corresponding value is a nested JSON object representing the CRM object fields. Each key in the nested JSON is the field name and the corresponding value is the field value.</p><p>The objects are fetched from the Gong main DB. If the object is not found, the JSON’s value is null.</p><p>The request body contains an array of objects ids.</p><p>The request is limited to 100 objects. If more than 100 objects are requested only the first 100 are returned.</p><p>When accessed using a bearer token, this endpoint requires the 'api:crm:get-objects' scope.</p><h3>Example</h3><h4>Request</h4><code>GET https://api.gong.io/v2/crm/entities?integrationId=6286478263646&objectType=DEAL</code><br><br><code>[\"1234\",\"8765\"] //request body</code>
pub async fn get_crm_objects(configuration: &configuration::Configuration, params: GetCrmObjectsParams) -> Result<models::GetGenericCrmObjectsResponse, Error<GetCrmObjectsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let integration_id = params.integration_id;
    let object_type = params.object_type;
    let objects_crm_ids = params.objects_crm_ids;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/crm/entities", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("integrationId", &integration_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("objectType", &object_type.to_string())]);
    local_var_req_builder = match "multi" {
        "multi" => local_var_req_builder.query(&objects_crm_ids.into_iter().map(|p| ("objectsCrmIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
        _ => local_var_req_builder.query(&[("objectsCrmIds", &objects_crm_ids.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
    };
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
        let local_var_entity: Option<GetCrmObjectsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// <style>.public-api-info {    background: rgb(222, 235, 255);}.public-api-tip {    background: rgb(227, 252, 239);}.public-api-parameter {    background: rgba(9,30,66,0.08);}.public-api-note {    background: rgb(234, 230, 255);}.public-api-important {    background: rgb(255, 250, 230);}.public-api-critical {    background: rgb(255, 235, 230);}table, th, td {  border: 1px solid gray;  border-collapse: collapse;}th, td {  padding: 5px;}th {  text-align: left;}img {  border: 2px solid #D3D5D9;}</style><p>Returns the current status of the request for endpoints run asynchronously: </p><ul>  <li>POST /v2/crm/entities</li>  <li>DELETE /v2/crm/integrations</li></ul><p>When accessed using a bearer token, this endpoint requires the 'api:crm:upload' scope.</p><h3>Status Codes</h3><ul>  <li>PENDING: file is pending being processed</li>  <li>IN_PROGRESS: file is being processed</li>  <li>DONE: all objects in the file were successfully processed</li>  <li>FAILED: failed to parse some objects, or a general error occurred when the file was being processed</li></ul><h3>Correcting a file that failed to be processed:</h3><p>When the status in the response is FAILED do one of the following: </p><ul>  <li>If the response includes a list of errors, correct the errors in the file as follows:     <ol>      <li>Using the <code>errors.line</code> attribute, locate and correct the data in the JSON file.</li>      <li>You can resend the entire LDJSON file, or only the corrected records via the relevant API. <br>Note: The response returns a maximum of 20 errors. To make sure you have corrected all errors, upload the entire file repeatedly until you receive a DONE status.</li>    </ol>  </li>  <li>A single error in the form of: <br>{\"line\":0,\"description\":\".....\"} indicates a general processing error:     <ol>      <li>Fix the LDJSON file according to the error message.</li>      <li>Upload the entire LDJSON file again.</li>    </ol>  </li></ul>
pub async fn get_request_status(configuration: &configuration::Configuration, params: GetRequestStatusParams) -> Result<models::RequestStatusResponse, Error<GetRequestStatusError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let integration_id = params.integration_id;
    let client_request_id = params.client_request_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/crm/request-status", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("integrationId", &integration_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("clientRequestId", &client_request_id.to_string())]);
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
        let local_var_entity: Option<GetRequestStatusError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// <style>.public-api-info {    background: rgb(222, 235, 255);}.public-api-tip {    background: rgb(227, 252, 239);}.public-api-parameter {    background: rgba(9,30,66,0.08);}.public-api-note {    background: rgb(234, 230, 255);}.public-api-important {    background: rgb(255, 250, 230);}.public-api-critical {    background: rgb(255, 235, 230);}table, th, td {  border: 1px solid gray;  border-collapse: collapse;}th, td {  padding: 5px;}th {  text-align: left;}img {  border: 2px solid #D3D5D9;}</style><p>Retrieves a list of the object schema fields.</p><p>When accessed using a bearer token, this endpoint requires the scope 'api:crm:schema'.</p><h3>Example</h3><h4>Request</h4><code>GET https://api.gong.io/v2/crm/entity-schema?integrationId=6286478263646&objectType=ACCOUNT</code><h4>Response</h4><code>{    \"requestId\": \"afjkzqkqglog7ueki5\",    \"objectTypeToSelectedFields\": {        \"ACCOUNT\": [            {                \"name\": \"accountTypePicklist\",                \"label\": \"Account Type\",                \"type\": \"PICKLIST\",                \"lastModified\": null,                \"isDeleted\": false,                \"referenceTo\": null,                \"orderedValueList\": null            },            {                \"name\": \"accountTypePicklist2\",                \"label\": \"Account Type2\",                \"type\": \"PICKLIST\",                \"lastModified\": null,                \"isDeleted\": false,                \"referenceTo\": null,                \"orderedValueList\": null            },            {                \"name\": \"fooBar\",                \"label\": \"Foo Bar\",                \"type\": \"STRING\",                \"lastModified\": null,                \"isDeleted\": false,                \"referenceTo\": null,                \"orderedValueList\": null            }        ]    }}</code>
pub async fn list_crm_schema_fields(configuration: &configuration::Configuration, params: ListCrmSchemaFieldsParams) -> Result<models::ListSelectedFieldsResponse, Error<ListCrmSchemaFieldsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let integration_id = params.integration_id;
    let object_type = params.object_type;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/crm/entity-schema", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("integrationId", &integration_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("objectType", &object_type.to_string())]);
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
        let local_var_entity: Option<ListCrmSchemaFieldsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// <style>.public-api-info {    background: rgb(222, 235, 255);}.public-api-tip {    background: rgb(227, 252, 239);}.public-api-parameter {    background: rgba(9,30,66,0.08);}.public-api-note {    background: rgb(234, 230, 255);}.public-api-important {    background: rgb(255, 250, 230);}.public-api-critical {    background: rgb(255, 235, 230);}table, th, td {  border: 1px solid gray;  border-collapse: collapse;}th, td {  padding: 5px;}th {  text-align: left;}img {  border: 2px solid #D3D5D9;}</style><p>Returns the CRM integration you set up using the <code>PUT /v2/crm/integrations</code> endpoint. You can only have one integration at a time. </p><p>When accessed using a bearer token, this endpoint requires the 'api:crm:integrations:read' scope.</p>
pub async fn list_generic_crm_integration(configuration: &configuration::Configuration) -> Result<models::ListGenericCrmIntegrationsResponse, Error<ListGenericCrmIntegrationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/crm/integrations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<ListGenericCrmIntegrationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// <style>.public-api-info {    background: rgb(222, 235, 255);}.public-api-tip {    background: rgb(227, 252, 239);}.public-api-parameter {    background: rgba(9,30,66,0.08);}.public-api-note {    background: rgb(234, 230, 255);}.public-api-important {    background: rgb(255, 250, 230);}.public-api-critical {    background: rgb(255, 235, 230);}table, th, td {  border: 1px solid gray;  border-collapse: collapse;}th, td {  padding: 5px;}th {  text-align: left;}img {  border: 2px solid #D3D5D9;}</style><p>Creates your CRM integration with Gong. Returns an integrationId, which should be used in requests to the CRM API to enable correct association of CRM data.</p><p>Multiple CRM integrations are not supported. To create a new integration, delete the old one first (<code>DELETE /v2/crm/integrations</code>).</p><p>This includes if you have integrated with Gong using one of the native CRM integrations such as HubSpot or Salesforce.</p><p>When accessed using a bearer token, this endpoint requires the 'api:crm:integration:register' scope.</p>
pub async fn register_generic_crm_integration(configuration: &configuration::Configuration, params: RegisterGenericCrmIntegrationParams) -> Result<models::RegisterGenericCrmResponse, Error<RegisterGenericCrmIntegrationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let generic_crm_registration_request = params.generic_crm_registration_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/crm/integrations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&generic_crm_registration_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RegisterGenericCrmIntegrationError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// <style>.public-api-info {    background: rgb(222, 235, 255);}.public-api-tip {    background: rgb(227, 252, 239);}.public-api-parameter {    background: rgba(9,30,66,0.08);}.public-api-note {    background: rgb(234, 230, 255);}.public-api-important {    background: rgb(255, 250, 230);}.public-api-critical {    background: rgb(255, 235, 230);}table, th, td {  border: 1px solid gray;  border-collapse: collapse;}th, td {  padding: 5px;}th {  text-align: left;}img {  border: 2px solid #D3D5D9;}</style><p>Enables you to add, edit or remove fields to Gong CRM entities, so that you can display additional CRM data in Gong. Fields that are not included in the Gong CRM objects, and are uploaded without updating your schema, are not displayed in Gong.</p><p>Update your schema when:</p><ul>  <li>You create a new integration</li>  <li>There are changes to the schema of any object type. You can also send a schema update request before any request to the <code>/v2/crm/entities</code> API.</li></ul><p>When updating your schema:</p><ul>  <li>Add or change field: include all fields you want in the schema, including those sent previously. Example:     <table>      <tr>        <th>Date</th>        <th>Action</th>        <th>Result</th>      </tr>      <tr>        <td>3/4/2023</td>        <td>Send account schema with fieldA</td>        <td>fieldA added to the account</td>      </tr>      <tr>        <td>6/4/23</td>        <td>Send account schema with fieldA and fieldB</td>        <td>fieldA still included in account schema. fieldB added to the account</td>      </tr>      <tr>        <td>8/4/23</td>        <td>Send account schema with fieldC but without fieldA and fieldB</td>        <td>Receive an error that you must include all fields in the schema</td>      </tr>      <tr>        <td>15/4/23</td>        <td>Send account schema with fieldB, fieldC, and isDeleted for fieldA</td>        <td>fieldA and all it's data are deleted. fieldB and fieldC remain in the schema</td>      </tr>    </table>  </li>  <li>Changing a field: if you change the field type, a new field is created and the data associated with the original field is deleted. Other changes, such as the label do not result in data being deleted.</li>  <li>Deleting a field: to delete a field and it's associated data, send the field with isDeleted = true</li></ul><p>When accessed using a bearer token, this endpoint requires the 'api:crm:schema' scope.</p><h2>Supported field types</h2><p>This table describes the field types that can be added to your schema</p><table>  <tr><th>Field type</th><th>Format in JSON</th><th>Possible values</th></tr>  <tr><td>BOOLEAN</td><td>boolean</td><td>true, false</td></tr>  <tr><td>DATE</td><td>string (ISO-8601 date without time)</td><td>\"2020-05-31\"</td></tr>  <tr><td>DATETIME</td><td>string (ISO-8601 datetime without milliseconds)</td><td>\"2020-12-17T07:37:21+02:00\"<p>\"2020-12-17T05:37:21Z\"</p></td></tr>  <tr><td>PICKLIST</td><td>string - one of the values in an orderedValueList</td><td>\"Analyst\"</td></tr>  <tr><td>NUMBER</td><td>number</td><td>45.66, 8453</td></tr>  <tr><td>PERCENT</td><td>number (between 0 to 100)</td><td>67.3</td></tr>  <tr><td>CURRENCY*</td><td>number</td><td>34.68</td></tr>  <tr><td>PHONENUMBER</td><td>string</td><td>\"+14055766687\"</td></tr>  <tr><td>EMAILADDRESS</td><td>string</td><td>\"john.doe@anywhere.com\"</td></tr>  <tr><td>REFERENCE</td><td>string - the id of another object</td><td>\"48b009drax\"</td></tr>  <tr><td>ID</td><td>string - the id of the object</td><td>\"843hf8484jr84htg\"</td></tr>  <tr><td>STRING</td><td>string</td><td>\"whatever you want\"</td></tr>  <tr><td>URL</td><td>string</td><td>\"https://crm.com/account/6d4r578f\"</td></tr></table><p>* In the integration send a number value, and specify the correct currency symbol in the Gong UI. Currently Gong does not support multiple currencies per company.</p><h3>Example</h3><h4>Request</h4><p><code>POST https://api.gong.io/v2/crm/entity-schema?integrationId=6286478263646&objectType=ACCOUNT</code></p><p><code>[{\"uniqueName\": \"orderId\", \"label\": \"ID\", \"type\": \"ID\", \"lastModified\": \"2020-11-11T08:11:34+01:00\"},</code></p><p><code>{\"uniqueName\": \"parentAccount\", \"label\": \"Main Account\", \"type\": \"REFERENCE\", \"referenceTo\": \"ACCOUNT\", \"lastModified\": \"2020-11-11T08:11:34+01:00\"},</code></p><p><code>{\"uniqueName\": \"category\", \"label\": \"Category\", \"type\": \"PICKLIST\", \"orderedValueList\": [\"Analyst\", \"Competitor\", \"Customer\", \"Integrator\", \"Investor\", \"Partner\", \"Other\"], \"lastModified\": \"2020-11-11T08:11:34+01:00\"}, </code></p><p>// remove custom field</p><p><code>{\"uniqueName\": \"industry\", \"isDeleted\": true, \"label\": \"Industry\", \"type\": \"PICKLIST\", \"lastModified\": \"2020-11-21T08:11:34+01:00\"}] </code></p>
pub async fn upload_crm_schema_field(configuration: &configuration::Configuration, params: UploadCrmSchemaFieldParams) -> Result<models::BaseResponse, Error<UploadCrmSchemaFieldError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let integration_id = params.integration_id;
    let object_type = params.object_type;
    let generic_schema_field_request = params.generic_schema_field_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/crm/entity-schema", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("integrationId", &integration_id.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("objectType", &object_type.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&generic_schema_field_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UploadCrmSchemaFieldError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

