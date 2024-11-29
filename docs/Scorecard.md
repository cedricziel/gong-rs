# Scorecard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scorecard_id** | Option<**i64**> | The identifier of the scorecard. | [optional]
**scorecard_name** | Option<**String**> | Scorecard name. | [optional]
**workspace_id** | Option<**i64**> | Scorecard workspaceId. | [optional]
**enabled** | Option<**bool**> | If the scorecard is enabled or not. | [optional]
**updater_user_id** | Option<**i64**> | The user Id of the team member that updated the scorecard. | [optional]
**created** | Option<**String**> | The date and time when the scorecard was created in the ISO-8601 format, for example: '2024-01-17T16:20:05-03:00' or '2016-02-16T03:57:04.834+05:30' or '2020-11-20T21:30:07.233692Z', where Z stands for UTC. | [optional]
**updated** | Option<**String**> | The date and time when the scorecard was updated in the ISO-8601 format, for example: '2024-01-17T16:20:05-03:00' or '2016-02-16T03:57:04.834+05:30' or '2020-11-20T21:30:07.233692Z', where Z stands for UTC. | [optional]
**questions** | Option<[**Vec<models::Question>**](Question.md)> | The questions in the scorecard. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


