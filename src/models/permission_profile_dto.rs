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

/// PermissionProfileDto : Permission profile
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionProfileDto {
    /// Unique numeric identifier of the permission profile (up to 20 digits).
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Permission profile name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Permission profile description.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "callsAccess", skip_serializing_if = "Option::is_none")]
    pub calls_access: Option<Box<models::CallAccessWithPermissionLevel>>,
    #[serde(rename = "libraryFolderAccess", skip_serializing_if = "Option::is_none")]
    pub library_folder_access: Option<Box<models::LibraryFolderAccess>>,
    #[serde(rename = "dealsAccess", skip_serializing_if = "Option::is_none")]
    pub deals_access: Option<Box<models::DealsAccessWithPermissionLevel>>,
    #[serde(rename = "forecastPermissions", skip_serializing_if = "Option::is_none")]
    pub forecast_permissions: Option<Box<models::ForecastPermissions>>,
    #[serde(rename = "coachingAccess", skip_serializing_if = "Option::is_none")]
    pub coaching_access: Option<Box<models::CoachingAccessWithPermissionLevel>>,
    #[serde(rename = "insightsAccess", skip_serializing_if = "Option::is_none")]
    pub insights_access: Option<Box<models::InsightsAccessWithPermissionLevel>>,
    #[serde(rename = "usageAccess", skip_serializing_if = "Option::is_none")]
    pub usage_access: Option<Box<models::UsageAccessWithPermissionLevel>>,
    #[serde(rename = "emailsAccess", skip_serializing_if = "Option::is_none")]
    pub emails_access: Option<Box<models::EmailsAccessWithPermissionLevel>>,
    /// User can score calls.
    #[serde(rename = "scoreCalls", skip_serializing_if = "Option::is_none")]
    pub score_calls: Option<bool>,
    /// User can download call media.
    #[serde(rename = "downloadCallMedia", skip_serializing_if = "Option::is_none")]
    pub download_call_media: Option<bool>,
    /// User can share calls with customers.
    #[serde(rename = "shareCallsWithCustomers", skip_serializing_if = "Option::is_none")]
    pub share_calls_with_customers: Option<bool>,
    /// User can manually schedule and upload calls.
    #[serde(rename = "manuallyScheduleAndUploadCalls", skip_serializing_if = "Option::is_none")]
    pub manually_schedule_and_upload_calls: Option<bool>,
    /// User can set their own calls as private.
    #[serde(rename = "privateCalls", skip_serializing_if = "Option::is_none")]
    pub private_calls: Option<bool>,
    /// User can delete calls.
    #[serde(rename = "deleteCalls", skip_serializing_if = "Option::is_none")]
    pub delete_calls: Option<bool>,
    /// User can trim calls.
    #[serde(rename = "trimCalls", skip_serializing_if = "Option::is_none")]
    pub trim_calls: Option<bool>,
    /// User can listen in calls.
    #[serde(rename = "listenInCalls", skip_serializing_if = "Option::is_none")]
    pub listen_in_calls: Option<bool>,
    /// User can delete emails.
    #[serde(rename = "deleteEmails", skip_serializing_if = "Option::is_none")]
    pub delete_emails: Option<bool>,
    /// User can view and search calls.
    #[serde(rename = "callsAndSearch", skip_serializing_if = "Option::is_none")]
    pub calls_and_search: Option<bool>,
    /// User can view library pages.
    #[serde(rename = "library", skip_serializing_if = "Option::is_none")]
    pub library: Option<bool>,
    /// User can view deals pages.
    #[serde(rename = "deals", skip_serializing_if = "Option::is_none")]
    pub deals: Option<bool>,
    /// User can create/edit/delete deals boards.
    #[serde(rename = "createEditAndDeleteDealsBoards", skip_serializing_if = "Option::is_none")]
    pub create_edit_and_delete_deals_boards: Option<bool>,
    /// User can perform inline editing of deals.
    #[serde(rename = "dealsInlineEditing", skip_serializing_if = "Option::is_none")]
    pub deals_inline_editing: Option<bool>,
    /// User can view account pages.
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<bool>,
    /// User can view coaching pages.
    #[serde(rename = "coaching", skip_serializing_if = "Option::is_none")]
    pub coaching: Option<bool>,
    /// User can view usage pages.
    #[serde(rename = "usage", skip_serializing_if = "Option::is_none")]
    pub usage: Option<bool>,
    /// User can view team stats page.
    #[serde(rename = "teamStats", skip_serializing_if = "Option::is_none")]
    pub team_stats: Option<bool>,
    /// User can view initiatives page.
    #[serde(rename = "initiatives", skip_serializing_if = "Option::is_none")]
    pub initiatives: Option<bool>,
    /// User can view market page.
    #[serde(rename = "market", skip_serializing_if = "Option::is_none")]
    pub market: Option<bool>,
    /// User can view activity pages.
    #[serde(rename = "activity", skip_serializing_if = "Option::is_none")]
    pub activity: Option<bool>,
    /// User can view forecast pages.
    #[serde(rename = "forecast", skip_serializing_if = "Option::is_none")]
    pub forecast: Option<bool>,
    /// User can manage forecast boards and upload targets.
    #[serde(rename = "forecastManage", skip_serializing_if = "Option::is_none")]
    pub forecast_manage: Option<bool>,
    /// User can manage company email templates.
    #[serde(rename = "engageManageCompanyTemplates", skip_serializing_if = "Option::is_none")]
    pub engage_manage_company_templates: Option<bool>,
    /// User can manage company sequences.
    #[serde(rename = "engageManageCompanySequences", skip_serializing_if = "Option::is_none")]
    pub engage_manage_company_sequences: Option<bool>,
    /// User can snooze flow in to dos for others
    #[serde(rename = "engageSnoozeFlowToDosForOthers", skip_serializing_if = "Option::is_none")]
    pub engage_snooze_flow_to_dos_for_others: Option<bool>,
    /// User can manage general business settings.
    #[serde(rename = "manageGeneralBusinessSettings", skip_serializing_if = "Option::is_none")]
    pub manage_general_business_settings: Option<bool>,
    /// User can manage scorecards.
    #[serde(rename = "manageScorecards", skip_serializing_if = "Option::is_none")]
    pub manage_scorecards: Option<bool>,
    /// User can export calls and coaching metrics data to CSV.
    #[serde(rename = "exportCallsAndCoachingDataToCSV", skip_serializing_if = "Option::is_none")]
    pub export_calls_and_coaching_data_to_csv: Option<bool>,
    /// User can perform inline editing of crm data.
    #[serde(rename = "crmDataInlineEditing", skip_serializing_if = "Option::is_none")]
    pub crm_data_inline_editing: Option<bool>,
    /// User can perform import of crm data.
    #[serde(rename = "crmDataImport", skip_serializing_if = "Option::is_none")]
    pub crm_data_import: Option<bool>,
    /// User can view dashboards page.
    #[serde(rename = "viewRevenueAnalytics", skip_serializing_if = "Option::is_none")]
    pub view_revenue_analytics: Option<bool>,
    /// User can manage revenue analytics.
    #[serde(rename = "manageRevenueAnalytics", skip_serializing_if = "Option::is_none")]
    pub manage_revenue_analytics: Option<bool>,
}

impl PermissionProfileDto {
    /// Permission profile
    pub fn new() -> PermissionProfileDto {
        PermissionProfileDto {
            id: None,
            name: None,
            description: None,
            calls_access: None,
            library_folder_access: None,
            deals_access: None,
            forecast_permissions: None,
            coaching_access: None,
            insights_access: None,
            usage_access: None,
            emails_access: None,
            score_calls: None,
            download_call_media: None,
            share_calls_with_customers: None,
            manually_schedule_and_upload_calls: None,
            private_calls: None,
            delete_calls: None,
            trim_calls: None,
            listen_in_calls: None,
            delete_emails: None,
            calls_and_search: None,
            library: None,
            deals: None,
            create_edit_and_delete_deals_boards: None,
            deals_inline_editing: None,
            account: None,
            coaching: None,
            usage: None,
            team_stats: None,
            initiatives: None,
            market: None,
            activity: None,
            forecast: None,
            forecast_manage: None,
            engage_manage_company_templates: None,
            engage_manage_company_sequences: None,
            engage_snooze_flow_to_dos_for_others: None,
            manage_general_business_settings: None,
            manage_scorecards: None,
            export_calls_and_coaching_data_to_csv: None,
            crm_data_inline_editing: None,
            crm_data_import: None,
            view_revenue_analytics: None,
            manage_revenue_analytics: None,
        }
    }
}

