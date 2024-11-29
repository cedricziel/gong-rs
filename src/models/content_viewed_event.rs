/*
 * Gong API
 *
 * <h2>Overview</h2> <p> The Gong API allows you to: </p> <ol> <li> Receive the following information from Gong: <ol type=\"a\"> <li> Your company's <a href=\"#tag--Calls\">calls</a> in Gong </li> <li> Your company's <a href=\"#tag--Users\">users</a> in Gong </li> <li> Your company's user <a href=\"#tag--Stats\">stats</a> in Gong </li> <li> Your company's user <a href=\"#tag--Settings\">settings</a> in Gong </li> <li> Your company's <a href=\"#tag--Library\">libraries</a> in Gong </li> </ol></li> <li> <a href=\"#post-/v2/calls\">Upload</a> new or  <a href=\"#put-/v2/calls/-id-/media\">update</a>  call recordings in Gong, in order to support cases where you have an internal system that records  calls or obtains them from a third-party entity. </li> <li> <a href=\"#post-/v2/data-privacy/erase-data-for-email-address\">Data Privacy</a>:  Delete users and all their associated elements.</li> <li> Upload <a href=\"#tag--CRM\">CRM</a> data into Gong.  </li> </ol> <p>Check <a href=\"https://app.gong.io/company/api-authentication?currentTab=MY_API_TAB\">here</a> what's your base URL for all API calls. </p> <h2>Authentication</h2>  <p> There are two ways to retrieve credentials to the Gong Public API: </p> <ol><li>Retrieve Manually:<br> <p> In the <a href=\"https://app.gong.io/company/api\">Gong API Page</a> (you must be a technical administrator in Gong), click \"Create\" to receive an <b>Access Key</b>  and an <b>Access Key Secret</b>.<br> </p> <p> Use the Basic Authorization HTTP header (as per <a target=\"_blank\" href=\"https://www.rfc-editor.org/rfc/rfc7617.txt\">RFC</a>) to access the Public API as shown below:<br> <code>Authorization: Basic &lt;token&gt;</code><br> </p> <p> To create the basic token, combine the <b>Access Key</b> and the <b>Access Key Secret</b> with  colon (:) and then encode in Base64 as following:<br> <code>Base64(&lt;accessKey&gt; : &lt;accessKeySecret&gt;)</code><br><br> </p></li> <li>Retrieve through OAuth<br> <p> To obtain the Bearer token, follow the steps described in the <a target=\"_blank\" href=\"https://help.gong.io/hc/en-us/articles/13944551222157-Create-an-app-for-Gong\">Gong OAuth Guide</a>. <br></p> <p> After obtaining the token, use the Bearer Authorization HTTP header (as per <a target=\"_blank\" href=\"https://www.rfc-editor.org/rfc/rfc6750.txt\">RFC</a>) to access the Public API as shown below:<br> <code>Authorization: Bearer &lt;token&gt;</code> </p> </li></ol> <h2>Limits</h2>  <p> By default Gong limits your company's access to the service to 3 API calls per second, and 10,000 API calls per day. </p> <p> When the rate of API calls exceeds these limits an HTTP status code <b>429</b> is returned and a <b>Retry-After</b> header indicates  how many seconds to wait before making a new request. </p><p> If required, contact <a target=\"_blank\" href=\"https://help.gong.io\">help.gong.io</a> to change these limits. </p>  <h2>Cursors</h2>  <p> Some API calls that return a list are limited in the amount of records they may return, so multiple API calls may be  required to bring all records. Such an API call also returns a <b>records</b> field, which contains the number of records  in the current page, the current page number and the total number of records. </p> <p> In cases where the total number of records exceeds the number of records thus far retrieved, the <b>records</b> field will also  contain a <b>cursor</b> field which can be used to access the next page of records. To retrieve the next page, repeat the API call with  the <b>cursor</b> value as supplied by the previous API call. All other request inputs should remain the same. </p> <h2>Forward Compatibility</h2>  <p> When coding a system to accept Gong data, take into account that Gong may, without prior warning, add fields to the JSON output.  It is recommended to future proof your code so that it disregards all JSON fields you don't actually use.  </p><p></p>
 *
 * The version of the OpenAPI document: V2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ContentViewedEvent : Content Viewed event
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentViewedEvent {
    /// The unique identifier of the reporting system. It is the same value in all events originating from the same system.
    #[serde(rename = "reportingSystem")]
    pub reporting_system: String,
    /// The date and time when the event happened in the ISO-8601 format (e.g., '2021-08-01T02:30:00+05:00' or '2021-08-01T08:00:00Z', where Z stands for UTC);
    #[serde(rename = "eventTimestamp")]
    pub event_timestamp: String,
    /// The original id of the event as designated in the reporting system.
    #[serde(rename = "eventId", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// The id of the content that was viewed in the reporting system.
    #[serde(rename = "contentId")]
    pub content_id: String,
    /// The url of the content that was viewed in the reporting system. This is the url that is was accessed by the viewer.
    #[serde(rename = "contentUrl")]
    pub content_url: String,
    /// Human readable title of the content.
    #[serde(rename = "contentTitle")]
    pub content_title: String,
    /// The name of the action like \"Document Viewed\" or \"Presentation Opened\".
    #[serde(rename = "viewActionTitle", skip_serializing_if = "Option::is_none")]
    pub view_action_title: Option<String>,
    /// The id of the share action that corresponds to this view event, in case there can be more than one share per content.
    #[serde(rename = "shareId", skip_serializing_if = "Option::is_none")]
    pub share_id: Option<String>,
    /// The link to a page that presents additional information about this event.
    #[serde(rename = "viewInfoUrl", skip_serializing_if = "Option::is_none")]
    pub view_info_url: Option<String>,
    #[serde(rename = "viewer", skip_serializing_if = "Option::is_none")]
    pub viewer: Option<Box<models::Actor>>,
    /// A list of references to external systems such as CRM, Telephony System, Case Management, etc.
    #[serde(rename = "crmContext", skip_serializing_if = "Option::is_none")]
    pub crm_context: Option<Vec<models::CallUploadContext>>,
    /// A list of additional properties for the content
    #[serde(rename = "contentProperties", skip_serializing_if = "Option::is_none")]
    pub content_properties: Option<Vec<models::GenericProperty>>,
    /// A list of additional properties for the event
    #[serde(rename = "eventProperties", skip_serializing_if = "Option::is_none")]
    pub event_properties: Option<Vec<models::GenericProperty>>,
    /// \"User-Agent\" header value for browser based interaction
    #[serde(rename = "userAgent", skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// The application identification string in case of interaction via mobile application (bundle identifier or package name).
    #[serde(rename = "mobileAppId", skip_serializing_if = "Option::is_none")]
    pub mobile_app_id: Option<String>,
    /// Platform on which the interaction was made
    #[serde(rename = "agentPlatform", skip_serializing_if = "Option::is_none")]
    pub agent_platform: Option<AgentPlatform>,
    /// Optional workspace identifier. If specified, the event will be placed into this workspace, otherwise, the default algorithm for workspace placement will be applied.
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "nonCompanyParticipants", skip_serializing_if = "Option::is_none")]
    pub non_company_participants: Option<Vec<models::Actor>>,
    #[serde(rename = "moreInfoUrl", skip_serializing_if = "Option::is_none")]
    pub more_info_url: Option<String>,
    #[serde(rename = "actionName", skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[serde(rename = "sharer", skip_serializing_if = "Option::is_none")]
    pub sharer: Option<Box<models::Sharer>>,
    #[serde(rename = "sharingMessageSubject", skip_serializing_if = "Option::is_none")]
    pub sharing_message_subject: Option<String>,
    #[serde(rename = "sharingMessageBody", skip_serializing_if = "Option::is_none")]
    pub sharing_message_body: Option<String>,
}

impl ContentViewedEvent {
    /// Content Viewed event
    pub fn new(reporting_system: String, event_timestamp: String, content_id: String, content_url: String, content_title: String) -> ContentViewedEvent {
        ContentViewedEvent {
            reporting_system,
            event_timestamp,
            event_id: None,
            content_id,
            content_url,
            content_title,
            view_action_title: None,
            share_id: None,
            view_info_url: None,
            viewer: None,
            crm_context: None,
            content_properties: None,
            event_properties: None,
            user_agent: None,
            mobile_app_id: None,
            agent_platform: None,
            workspace_id: None,
            non_company_participants: None,
            more_info_url: None,
            action_name: None,
            sharer: None,
            sharing_message_subject: None,
            sharing_message_body: None,
        }
    }
}
/// Platform on which the interaction was made
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AgentPlatform {
    #[serde(rename = "Windows, Linux, MacOS, iOS, Android")]
    WindowsCommaLinuxCommaMacOsCommaIOsCommaAndroid,
}

impl Default for AgentPlatform {
    fn default() -> AgentPlatform {
        Self::WindowsCommaLinuxCommaMacOsCommaIOsCommaAndroid
    }
}

