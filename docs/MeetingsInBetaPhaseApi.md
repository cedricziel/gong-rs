# \MeetingsInBetaPhaseApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_meeting**](MeetingsInBetaPhaseApi.md#add_meeting) | **POST** /v2/meetings | Create a New Gong Meeting (/v2/meetings)
[**delete_meeting**](MeetingsInBetaPhaseApi.md#delete_meeting) | **DELETE** /v2/meetings/{meetingId} | Delete a Gong Meeting (/v2/meetings)
[**integration_status**](MeetingsInBetaPhaseApi.md#integration_status) | **POST** /v2/meetings/integration/status | Validate Gong meeting Integration (/v2/meetings/integration/status)
[**update_meeting**](MeetingsInBetaPhaseApi.md#update_meeting) | **PUT** /v2/meetings/{meetingId} | Update a Gong Meeting (/v2/meetings/{meetingId})



## add_meeting

> models::NewMeetingResponse add_meeting(new_meeting_request)
Create a New Gong Meeting (/v2/meetings)

When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:meetings:user:create'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_meeting_request** | [**NewMeetingRequest**](NewMeetingRequest.md) |  | [required] |

### Return type

[**models::NewMeetingResponse**](NewMeetingResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_meeting

> models::DeleteMeetingRequest delete_meeting(meeting_id, delete_meeting_request)
Delete a Gong Meeting (/v2/meetings)

When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:meetings:user:delete'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meeting_id** | **i64** | Gong's unique identifier for the meeting (up to 20 digits). | [required] |
**delete_meeting_request** | [**DeleteMeetingRequest**](DeleteMeetingRequest.md) |  | [required] |

### Return type

[**models::DeleteMeetingRequest**](DeleteMeetingRequest.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## integration_status

> models::IntegrationStatusResponse integration_status(integration_status_request)
Validate Gong meeting Integration (/v2/meetings/integration/status)

When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:meetings:integration:status'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_status_request** | [**IntegrationStatusRequest**](IntegrationStatusRequest.md) |  | [required] |

### Return type

[**models::IntegrationStatusResponse**](IntegrationStatusResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_meeting

> models::UpdateMeetingResponse update_meeting(meeting_id, update_meeting_request)
Update a Gong Meeting (/v2/meetings/{meetingId})

When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:meetings:user:update'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meeting_id** | **i64** | Gong's unique identifier for the meeting (up to 20 digits). | [required] |
**update_meeting_request** | [**UpdateMeetingRequest**](UpdateMeetingRequest.md) |  | [required] |

### Return type

[**models::UpdateMeetingResponse**](UpdateMeetingResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

