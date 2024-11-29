# \CallsApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_call**](CallsApi.md#add_call) | **POST** /v2/calls | Add new call (/v2/calls)
[**get_call**](CallsApi.md#get_call) | **GET** /v2/calls/{id} | Retrieve data for a specific call (/v2/calls/{id})
[**get_call_transcripts**](CallsApi.md#get_call_transcripts) | **POST** /v2/calls/transcript | Retrieve transcripts of calls (/v2/calls/transcript)
[**list_calls**](CallsApi.md#list_calls) | **GET** /v2/calls | Retrieve call data by date range (/v2/calls)
[**list_calls_extensive**](CallsApi.md#list_calls_extensive) | **POST** /v2/calls/extensive | Retrieve detailed call data by various filters (/v2/calls/extensive)
[**list_crm_calls_manual_association**](CallsApi.md#list_crm_calls_manual_association) | **GET** /v2/calls/manual-crm-associations | List all calls that were manually associated with CRM objects (/v2/calls/manual-crm-associations) in Beta Phase



## add_call

> models::NewCallAddingResponse add_call(new_call_adding_request)
Add new call (/v2/calls)

When using this endpoint, either provide a downloadMediaUrl or use the returned callId in a follow-up request to /v2/calls/{id}/media to upload the media file.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:calls:create'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_call_adding_request** | [**NewCallAddingRequest**](NewCallAddingRequest.md) |  | [required] |

### Return type

[**models::NewCallAddingResponse**](NewCallAddingResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_call

> models::SpecificCall get_call(id)
Retrieve data for a specific call (/v2/calls/{id})

Retrieve data for a specific call.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:calls:read:basic'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Gong's unique numeric identifier for the call (up to 20 digits). | [required] |

### Return type

[**models::SpecificCall**](SpecificCall.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_call_transcripts

> models::CallTranscripts get_call_transcripts(public_api_base_request_v2_calls_filter)
Retrieve transcripts of calls (/v2/calls/transcript)

Returns transcripts for calls that took place during the specified date period. If call IDs are specified, only transcripts for calls with those IDs that took place during the time period are returned.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:calls:read:transcript'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_api_base_request_v2_calls_filter** | [**PublicApiBaseRequestV2CallsFilter**](PublicApiBaseRequestV2CallsFilter.md) |  | [required] |

### Return type

[**models::CallTranscripts**](CallTranscripts.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_calls

> models::CallsResponse list_calls(from_date_time, to_date_time, cursor, workspace_id)
Retrieve call data by date range (/v2/calls)

List calls that took place during a specified date range.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:calls:read:basic'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_date_time** | **String** | Date and time (in ISO-8601 format: '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC) from which to list recorded calls. Returns calls that started on or after the specified date and time. If not provided, list starts with earliest call. For web-conference calls recorded by Gong, the date denotes its scheduled time, otherwise, it denotes its actual start time. | [required] |
**to_date_time** | **String** | Date and time (in ISO-8601 format: '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC) until which to list recorded calls. Returns calls that started up to but excluding specified date and time. If not provided, list ends with most recent call. For web-conference calls recorded by Gong, the date denotes its scheduled time, otherwise, it denotes its actual start time. | [required] |
**cursor** | Option<**String**> | When paging is needed, provide the value supplied by the previous API call to bring the following page of records. |  |
**workspace_id** | Option<**String**> | Optional Workspace identifier, if supplied the API will return only the calls belonging to this workspace. |  |

### Return type

[**models::CallsResponse**](CallsResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_calls_extensive

> models::Calls list_calls_extensive(public_api_base_request_with_data_v2_calls_request_filter_with_owners_content_selector)
Retrieve detailed call data by various filters (/v2/calls/extensive)

Lists detailed call data for calls that took place during a specified date range, have specified call IDs or hosted by specified users.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:calls:read:extensive'.  Moreover, clients requesting media download URLs by the contentSelector.exposedFields.media field should also have the scope 'api:calls:read:media-url'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_api_base_request_with_data_v2_calls_request_filter_with_owners_content_selector** | [**PublicApiBaseRequestWithDataV2CallsRequestFilterWithOwnersContentSelector**](PublicApiBaseRequestWithDataV2CallsRequestFilterWithOwnersContentSelector.md) |  | [required] |

### Return type

[**models::Calls**](Calls.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_crm_calls_manual_association

> models::ManualAssociationResponse list_crm_calls_manual_association(from_date_time, cursor)
List all calls that were manually associated with CRM objects (/v2/calls/manual-crm-associations) in Beta Phase

Returns a list of all calls that were manually associated or re-associated with CRM account and deal/opportunity since a given time.  Actions will be listed in the ascending order of their timing.   Notice if a call was associated and later re-associated the API will return both events.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:crm-calls:manual-association:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_date_time** | Option<**String**> | Association time filter - only the associations made after that time will be listed. The time is in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC); if not specified all association events will be listed. |  |
**cursor** | Option<**String**> | When paging is needed, provide the value supplied by the previous API call to bring the following page of records. |  |

### Return type

[**models::ManualAssociationResponse**](ManualAssociationResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

