# CompanyUsersAggregateActivityResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | A Gong request reference Id, generated for this request. Can be used for troubleshooting purposes. | [optional]
**records** | Option<[**models::Records**](Records.md)> |  | [optional]
**users_aggregate_activity_stats** | Option<[**Vec<models::UserActivity>**](UserActivity.md)> | A list, in which each item specifies one user's activities. | [optional]
**time_zone** | Option<**String**> | The company's defined timezone in Gong. | [optional]
**to_date_time** | Option<**String**> | The date and time in the ISO-8601 format, for example: '2024-01-17T16:20:05-03:00' or '2016-02-16T03:57:04.834+05:30' or '2020-11-20T21:30:07.233692Z', where Z stands for UTC, when the list of results ends. | [optional]
**from_date_time** | Option<**String**> | The date and time in the ISO-8601 format, for example: '2024-01-17T16:20:05-03:00' or '2016-02-16T03:57:04.834+05:30' or '2020-11-20T21:30:07.233692Z', where Z stands for UTC, when the list of results starts. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


