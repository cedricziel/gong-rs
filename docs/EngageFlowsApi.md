# \EngageFlowsApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_prospects**](EngageFlowsApi.md#assign_prospects) | **POST** /v2/flows/prospects/assign | Assign prospects (contacts or leads) to an Engage flow  (/v2/flows/prospects/assign)
[**get_flows_for_prospects**](EngageFlowsApi.md#get_flows_for_prospects) | **POST** /v2/flows/prospects | List assigned flows for the given prospects (/v2/flows/prospects)
[**list_flows**](EngageFlowsApi.md#list_flows) | **GET** /v2/flows | List Gong Engage flows (/v2/flows)



## assign_prospects

> models::AssignFlowResponse assign_prospects(assign_flow_request_v2)
Assign prospects (contacts or leads) to an Engage flow  (/v2/flows/prospects/assign)

Use this endpoint to assign a number of prospects to a flow. Prospects can be contacts or leads in your CRM.  Generate a list of comma separated CRM Ids of the prospects you want to add to a flow. The flowInstanceOwnerEmail parameter is the email address of the Gong user who set up the flow instance and owns the flow to-dos.  You can assign up to 200 prospects to a flow in a single request.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:flows:write'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assign_flow_request_v2** | [**AssignFlowRequestV2**](AssignFlowRequestV2.md) |  | [required] |

### Return type

[**models::AssignFlowResponse**](AssignFlowResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_for_prospects

> models::ProspectsFlowsResponse get_flows_for_prospects(prospects_assigned_flows_request_v2)
List assigned flows for the given prospects (/v2/flows/prospects)

Get the Gong Engage flows assigned to the given prospects.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:flows:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prospects_assigned_flows_request_v2** | [**ProspectsAssignedFlowsRequestV2**](ProspectsAssignedFlowsRequestV2.md) |  | [required] |

### Return type

[**models::ProspectsFlowsResponse**](ProspectsFlowsResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_flows

> models::FlowsResponse list_flows(flow_owner_email, cursor, workspace_id)
List Gong Engage flows (/v2/flows)

Engage flows have the following visibility types:   * Company: visible to everyone in the company, can only be edited by users with edit permissions.   * Personal: reps can set up their own flows which are only available to them.   * Shared: reps can share a flow with other reps. Those reps can then add leads to the flow.   This endpoint returns all company flows, together with personal flows and flows shared with the user specified in the flowEmailOwner parameter.   When accessed through a Bearer token authorization method, use the 'api:flows:read' scope. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_owner_email** | **String** | Email of the Gong user whose personal flows and flows shared with the user you want to return. | [required] |
**cursor** | Option<**String**> | When paging is needed, provide the value supplied by the previous API call to bring the following page of records. |  |
**workspace_id** | Option<**String**> | Optional Workspace identifier, if supplied the API will return only the flows belonging to this workspace. |  |

### Return type

[**models::FlowsResponse**](FlowsResponse.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

