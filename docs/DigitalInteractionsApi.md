# \DigitalInteractionsApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_digital_interaction**](DigitalInteractionsApi.md#add_digital_interaction) | **POST** /v2/digital-interaction | Post a Digital Interaction (/v2/digital-interaction)



## add_digital_interaction

> models::EventAcceptedResponse add_digital_interaction(digital_interaction_request)
Post a Digital Interaction (/v2/digital-interaction)

When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:digital-interactions:write'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**digital_interaction_request** | [**DigitalInteractionRequest**](DigitalInteractionRequest.md) |  | [required] |

### Return type

[**models::EventAcceptedResponse**](EventAcceptedResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

