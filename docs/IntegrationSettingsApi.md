# \IntegrationSettingsApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**integration_settings**](IntegrationSettingsApi.md#integration_settings) | **POST** /v2/integration-settings | Integration Settings (/v2/integration-settings)



## integration_settings

> models::RegisterGenericCrmResponse integration_settings(integration_settings_request)
Integration Settings (/v2/integration-settings)

When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:integration-settings:write'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_settings_request** | [**IntegrationSettingsRequest**](IntegrationSettingsRequest.md) |  | [required] |

### Return type

[**models::RegisterGenericCrmResponse**](RegisterGenericCrmResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

