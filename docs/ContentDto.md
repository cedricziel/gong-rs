# ContentDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**structure** | Option<[**Vec<models::StructureDto>**](StructureDto.md)> | A list of the agenda of each part of the call. | [optional]
**trackers** | Option<[**Vec<models::TrackerDto>**](TrackerDto.md)> |  | [optional]
**topics** | Option<[**Vec<models::TopicDto>**](TopicDto.md)> |  | [optional]
**points_of_interest** | Option<[**models::PointsOfInterest**](PointsOfInterest.md)> |  | [optional]
**brief** | Option<**String**> | The brief of the call. Returned when it is available and contentSelector.exposedFields.content.brief = true. | [optional]
**outline** | Option<[**Vec<models::CallOutlineSection>**](CallOutlineSection.md)> | Outline of the call, divided into sections. Returned when it is available and contentSelector.exposedFields.content.outline = true. | [optional]
**highlights** | Option<[**Vec<models::CallHighlightsSection>**](CallHighlightsSection.md)> | List of highlights of the call. Returned when it is available and contentSelector.exposedFields.content.highlights = true. | [optional]
**call_outcome** | Option<[**models::CallOutcome**](CallOutcome.md)> |  | [optional]
**key_points** | Option<[**Vec<models::CallKeyPoint>**](CallKeyPoint.md)> | List of key points of the call. Returned when it is available and contentSelector.exposedFields.content.keyPoints = true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


