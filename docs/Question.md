# Question

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**question_id** | Option<**i64**> | The identifier for the question. | [optional]
**question_revision_id** | Option<**i64**> | The identifier for the revision version of the question. | [optional]
**question_text** | Option<**String**> | The question's text. | [optional]
**is_overall** | Option<**bool**> | If the question is the main overall question. | [optional]
**question_type** | Option<**String**> | The type of question. | [optional]
**answer_guide** | Option<**String**> | The guide for answering the question. | [optional]
**min_range** | Option<**i32**> | The minimum value that can be selected in the answer. Null for questions with no range. | [optional]
**max_range** | Option<**i32**> | The maximum value that can be selected in the answer. Null for questions with no range. | [optional]
**answer_options** | Option<[**Vec<models::QuestionOption>**](QuestionOption.md)> | The answer options in case of multi-select or single-select type question. Null otherwise. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


