/*
 * Gong API
 *
 * <h2>Overview</h2> <p> The Gong API allows you to: </p> <ol> <li> Receive the following information from Gong: <ol type=\"a\"> <li> Your company's <a href=\"#tag--Calls\">calls</a> in Gong </li> <li> Your company's <a href=\"#tag--Users\">users</a> in Gong </li> <li> Your company's user <a href=\"#tag--Stats\">stats</a> in Gong </li> <li> Your company's user <a href=\"#tag--Settings\">settings</a> in Gong </li> <li> Your company's <a href=\"#tag--Library\">libraries</a> in Gong </li> </ol></li> <li> <a href=\"#post-/v2/calls\">Upload</a> new or  <a href=\"#put-/v2/calls/-id-/media\">update</a>  call recordings in Gong, in order to support cases where you have an internal system that records  calls or obtains them from a third-party entity. </li> <li> <a href=\"#post-/v2/data-privacy/erase-data-for-email-address\">Data Privacy</a>:  Delete users and all their associated elements.</li> <li> Upload <a href=\"#tag--CRM\">CRM</a> data into Gong.  </li> </ol> <p>Check <a href=\"https://app.gong.io/company/api-authentication?currentTab=MY_API_TAB\">here</a> what's your base URL for all API calls. </p> <h2>Authentication</h2>  <p> There are two ways to retrieve credentials to the Gong Public API: </p> <ol><li>Retrieve Manually:<br> <p> In the <a href=\"https://app.gong.io/company/api\">Gong API Page</a> (you must be a technical administrator in Gong), click \"Create\" to receive an <b>Access Key</b>  and an <b>Access Key Secret</b>.<br> </p> <p> Use the Basic Authorization HTTP header (as per <a target=\"_blank\" href=\"https://www.rfc-editor.org/rfc/rfc7617.txt\">RFC</a>) to access the Public API as shown below:<br> <code>Authorization: Basic &lt;token&gt;</code><br> </p> <p> To create the basic token, combine the <b>Access Key</b> and the <b>Access Key Secret</b> with  colon (:) and then encode in Base64 as following:<br> <code>Base64(&lt;accessKey&gt; : &lt;accessKeySecret&gt;)</code><br><br> </p></li> <li>Retrieve through OAuth<br> <p> To obtain the Bearer token, follow the steps described in the <a target=\"_blank\" href=\"https://help.gong.io/hc/en-us/articles/13944551222157-Create-an-app-for-Gong\">Gong OAuth Guide</a>. <br></p> <p> After obtaining the token, use the Bearer Authorization HTTP header (as per <a target=\"_blank\" href=\"https://www.rfc-editor.org/rfc/rfc6750.txt\">RFC</a>) to access the Public API as shown below:<br> <code>Authorization: Bearer &lt;token&gt;</code> </p> </li></ol> <h2>Limits</h2>  <p> By default Gong limits your company's access to the service to 3 API calls per second, and 10,000 API calls per day. </p> <p> When the rate of API calls exceeds these limits an HTTP status code <b>429</b> is returned and a <b>Retry-After</b> header indicates  how many seconds to wait before making a new request. </p><p> If required, contact <a target=\"_blank\" href=\"https://help.gong.io\">help.gong.io</a> to change these limits. </p>  <h2>Cursors</h2>  <p> Some API calls that return a list are limited in the amount of records they may return, so multiple API calls may be  required to bring all records. Such an API call also returns a <b>records</b> field, which contains the number of records  in the current page, the current page number and the total number of records. </p> <p> In cases where the total number of records exceeds the number of records thus far retrieved, the <b>records</b> field will also  contain a <b>cursor</b> field which can be used to access the next page of records. To retrieve the next page, repeat the API call with  the <b>cursor</b> value as supplied by the previous API call. All other request inputs should remain the same. </p> <h2>Forward Compatibility</h2>  <p> When coding a system to accept Gong data, take into account that Gong may, without prior warning, add fields to the JSON output.  It is recommended to future proof your code so that it disregards all JSON fields you don't actually use.  </p><p></p>
 *
 * The version of the OpenAPI document: V2
 * Contact: mail@cedric-ziel.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AnsweredScorecardsFilter : Filter parameters.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnsweredScorecardsFilter {
    /// The date (in the company's time zone) of the call from when to list calls. This value is inclusive. The date should be specified in the following format: YYYY-MM-DD; if not specified, it starts with the earliest recorded call.
    #[serde(rename = "callFromDate", skip_serializing_if = "Option::is_none")]
    pub call_from_date: Option<String>,
    /// The date (in the company's time zone) of the call until when to list calls. This value is exclusive. The date should be specified in the following format: YYYY-MM-DD; if not specified, it ends with the latest recorded call.
    #[serde(rename = "callToDate", skip_serializing_if = "Option::is_none")]
    pub call_to_date: Option<String>,
    /// The review date (in the company's time zone) from when to list calls. This value is inclusive. The date should be specified in the following format: YYYY-MM-DD; if not specified, it starts with the earliest reviewed call.
    #[serde(rename = "reviewFromDate", skip_serializing_if = "Option::is_none")]
    pub review_from_date: Option<String>,
    /// The review date (in the company's time zone) until when to list calls. This value is exclusive. The date should be specified in the following format: YYYY-MM-DD; if not specified, it ends with the latest reviewed call.
    #[serde(rename = "reviewToDate", skip_serializing_if = "Option::is_none")]
    pub review_to_date: Option<String>,
    /// List of user Ids of the team members being reviewed. If not specified, it returns all answered scorecards in the corresponding filter.
    #[serde(rename = "reviewedUserIds", skip_serializing_if = "Option::is_none")]
    pub reviewed_user_ids: Option<Vec<String>>,
    /// List of scorecards Ids. If not specified, it returns all answered scorecards in the corresponding filter.
    #[serde(rename = "scorecardIds", skip_serializing_if = "Option::is_none")]
    pub scorecard_ids: Option<Vec<String>>,
}

impl AnsweredScorecardsFilter {
    /// Filter parameters.
    pub fn new() -> AnsweredScorecardsFilter {
        AnsweredScorecardsFilter {
            call_from_date: None,
            call_to_date: None,
            review_from_date: None,
            review_to_date: None,
            reviewed_user_ids: None,
            scorecard_ids: None,
        }
    }
}

