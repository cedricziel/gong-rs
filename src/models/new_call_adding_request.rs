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

/// NewCallAddingRequest : New call metadata
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewCallAddingRequest {
    /// A call's unique identifier in the PBX or the recording system. Gong uses this identifier to prevent repeated attempts to upload the same recording.
    #[serde(rename = "clientUniqueId")]
    pub client_unique_id: String,
    /// The title of the call. This title is available in the Gong system for indexing and search.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The purpose of the call. This optional field is a free text of up to 255 characters.
    #[serde(rename = "purpose", skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    /// The date and time the call was scheduled to begin in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC);
    #[serde(rename = "scheduledStart", skip_serializing_if = "Option::is_none")]
    pub scheduled_start: Option<String>,
    /// The date and time the call was scheduled to end in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC);
    #[serde(rename = "scheduledEnd", skip_serializing_if = "Option::is_none")]
    pub scheduled_end: Option<String>,
    /// The actual date and time when the call started in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC);
    #[serde(rename = "actualStart")]
    pub actual_start: String,
    /// The actual call duration in seconds.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<f32>,
    /// A list of the call's participants. A party must be provided for the primaryUser.
    #[serde(rename = "parties")]
    pub parties: Vec<models::CallParticipant>,
    /// Whether the call is inbound (someone called the company), outbound (a rep dialed someone outside the company), or a conference call.
    #[serde(rename = "direction")]
    pub direction: Direction,
    /// The disposition of the call. The disposition is free text of up to 255 characters.
    #[serde(rename = "disposition", skip_serializing_if = "Option::is_none")]
    pub disposition: Option<String>,
    /// A list of references to external systems such as CRM, Telephony System, Case Management, etc.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<models::CallUploadContext>>,
    /// Optional metadata associated with the call (represented as text). Gong stores this metadata and it can be used for troubleshooting.
    #[serde(rename = "customData", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<String>,
    #[serde(rename = "speakersTimeline", skip_serializing_if = "Option::is_none")]
    pub speakers_timeline: Option<Box<models::SpeakersTimeline>>,
    /// The URL of the conference call by which users join the meeting
    #[serde(rename = "meetingUrl", skip_serializing_if = "Option::is_none")]
    pub meeting_url: Option<String>,
    /// The code identifies the provider conferencing or telephony system. For example: zoom, clearslide, gotomeeting, ringcentral, outreach, insidesales, etc. These values are predefined by Gong, please contact help@gong.io to find the proper value for your system.
    #[serde(rename = "callProviderCode", skip_serializing_if = "Option::is_none")]
    pub call_provider_code: Option<String>,
    /// The URL from which Gong can download the media file. The URL must be unique, the audio or video file must be a maximum of 1.5GB. The content-type must either start with 'audio' or 'video,' or should be 'application/octet-stream' or 'binary/octet-stream' followed by a subtype that specifies a supported file type (WAV, MP3, MP4, MKV and FLAC). If you provide this URL, you should not perform the 'Add call media' step.
    #[serde(rename = "downloadMediaUrl", skip_serializing_if = "Option::is_none")]
    pub download_media_url: Option<String>,
    /// Optional workspace identifier. If specified, the call will be placed into this workspace, otherwise, the default algorithm for workspace placement will be applied.
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// The language code the call should be transcribed to. This field is optional as Gong automatically detects the language spoken in the call and transcribes it accordingly. Set this field only if you are sure of the language the call is in. Valid values are: af-ZA, am-ET, ar-AE, ar-BH, ar-DZ, ar-EG, ar-IL, ar-IQ, ar-JO, ar-KW, ar-LB, ar-MA, ar-MR, ar-OM, ar-PS, ar-QA, ar-SA, ar-TN, ar-YE, az-AZ, bg-BG, bn-BD, bn-IN, bs-BA, ca-ES, cs-CZ, da-DK, de-AT, de-CH, de-DE, el-GR, en-AB, en-AU, en-CA, en-GB, en-IE, en-IN, en-NZ, en-PH, en-SG, en-US, en-WL, en-ZA, es-AR, es-BO, es-CL, es-CO, es-CR, es-DO, es-EC, es-ES, es-GT, es-HN, es-MX, es-NI, es-PA, es-PE, es-PR, es-PY, es-SV, es-US, es-UY, et-EE, eu-ES, fa-IR, fi-FI, fil-PH, fr-BE, fr-CA, fr-CH, fr-FR, gl-ES, gu-IN, he-IL, hi-IN, hr-HR, hu-HU, hy-AM, id-ID, is-IS, it-CH, it-IT, ja-JP, jv-ID, ka-GE, kk-KZ, km-KH, kn-IN, ko-KR, lo-LA, lt-LT, lv-LV, mk-MK, ml-IN, mn-MN, mr-IN, ms-MY, my-MM, ne-NP, nl-BE, nl-NL, no-NO, pa-Guru-IN, pl-PL, pt-BR, pt-PT, ro-RO, ru-RU, si-LK, sk-SK, sl-SI, sq-AL, sr-RS, su-ID, sv-SE, sw-KE, sw-TZ, ta-IN, ta-LK, ta-MY, ta-SG, te-IN, th-TH, tr-TR, uk-UA, ur-IN, ur-PK, uz-UZ, vi-VN, yue-Hant-HK, zh-CN, zh-TW, zu-ZA
    #[serde(rename = "languageCode", skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// The Gong internal user ID of the team member who hosted the call.
    #[serde(rename = "primaryUser")]
    pub primary_user: String,
}

impl NewCallAddingRequest {
    /// New call metadata
    pub fn new(client_unique_id: String, actual_start: String, parties: Vec<models::CallParticipant>, direction: Direction, primary_user: String) -> NewCallAddingRequest {
        NewCallAddingRequest {
            client_unique_id,
            title: None,
            purpose: None,
            scheduled_start: None,
            scheduled_end: None,
            actual_start,
            duration: None,
            parties,
            direction,
            disposition: None,
            context: None,
            custom_data: None,
            speakers_timeline: None,
            meeting_url: None,
            call_provider_code: None,
            download_media_url: None,
            workspace_id: None,
            language_code: None,
            primary_user,
        }
    }
}
/// Whether the call is inbound (someone called the company), outbound (a rep dialed someone outside the company), or a conference call.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "Inbound")]
    Inbound,
    #[serde(rename = "Outbound")]
    Outbound,
    #[serde(rename = "Conference")]
    Conference,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Inbound
    }
}

