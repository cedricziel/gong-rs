# \CoachingApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_users1**](CoachingApi.md#list_users1) | **GET** /v2/coaching | List all coaching metrics (/v2/coaching)



## list_users1

> models::CoachingMetrics list_users1(workspace_id, manager_id, from, to)
List all coaching metrics (/v2/coaching)

List all of the coaching metrics of a manager.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:coaching:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i64** |  | [required] |
**manager_id** | **i64** |  | [required] |
**from** | **String** | Association time filter - only the associations made after that time will be listed. The time is in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC); if not specified all association events will be listed. | [required] |
**to** | **String** | Association time filter - only the associations made after that time will be listed. The time is in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC); if not specified all association events will be listed. | [required] |

### Return type

[**models::CoachingMetrics**](CoachingMetrics.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

