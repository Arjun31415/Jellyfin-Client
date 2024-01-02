# BaseItemDtoCurrentProgram

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Gets or sets the name. | [optional]
**original_title** | Option<**String**> |  | [optional]
**server_id** | Option<**String**> | Gets or sets the server identifier. | [optional]
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets the id. | [optional]
**etag** | Option<**String**> | Gets or sets the etag. | [optional]
**source_type** | Option<**String**> | Gets or sets the type of the source. | [optional]
**playlist_item_id** | Option<**String**> | Gets or sets the playlist item identifier. | [optional]
**date_created** | Option<**String**> | Gets or sets the date created. | [optional]
**date_last_media_added** | Option<**String**> |  | [optional]
**extra_type** | Option<**String**> |  | [optional]
**airs_before_season_number** | Option<**i32**> |  | [optional]
**airs_after_season_number** | Option<**i32**> |  | [optional]
**airs_before_episode_number** | Option<**i32**> |  | [optional]
**can_delete** | Option<**bool**> |  | [optional]
**can_download** | Option<**bool**> |  | [optional]
**has_subtitles** | Option<**bool**> |  | [optional]
**preferred_metadata_language** | Option<**String**> |  | [optional]
**preferred_metadata_country_code** | Option<**String**> |  | [optional]
**supports_sync** | Option<**bool**> | Gets or sets a value indicating whether [supports synchronize]. | [optional]
**container** | Option<**String**> |  | [optional]
**sort_name** | Option<**String**> | Gets or sets the name of the sort. | [optional]
**forced_sort_name** | Option<**String**> |  | [optional]
**video3_d_format** | Option<[**crate::models::Video3DFormat**](Video3DFormat.md)> |  | [optional]
**premiere_date** | Option<**String**> | Gets or sets the premiere date. | [optional]
**external_urls** | Option<[**Vec<crate::models::ExternalUrl>**](ExternalUrl.md)> | Gets or sets the external urls. | [optional]
**media_sources** | Option<[**Vec<crate::models::MediaSourceInfo>**](MediaSourceInfo.md)> | Gets or sets the media versions. | [optional]
**critic_rating** | Option<**f32**> | Gets or sets the critic rating. | [optional]
**production_locations** | Option<**Vec<String>**> |  | [optional]
**path** | Option<**String**> | Gets or sets the path. | [optional]
**enable_media_source_display** | Option<**bool**> |  | [optional]
**official_rating** | Option<**String**> | Gets or sets the official rating. | [optional]
**custom_rating** | Option<**String**> | Gets or sets the custom rating. | [optional]
**channel_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets the channel identifier. | [optional]
**channel_name** | Option<**String**> |  | [optional]
**overview** | Option<**String**> | Gets or sets the overview. | [optional]
**taglines** | Option<**Vec<String>**> | Gets or sets the taglines. | [optional]
**genres** | Option<**Vec<String>**> | Gets or sets the genres. | [optional]
**community_rating** | Option<**f32**> | Gets or sets the community rating. | [optional]
**cumulative_run_time_ticks** | Option<**i64**> | Gets or sets the cumulative run time ticks. | [optional]
**run_time_ticks** | Option<**i64**> | Gets or sets the run time ticks. | [optional]
**play_access** | Option<[**crate::models::PlayAccess**](PlayAccess.md)> |  | [optional]
**aspect_ratio** | Option<**String**> | Gets or sets the aspect ratio. | [optional]
**production_year** | Option<**i32**> | Gets or sets the production year. | [optional]
**is_place_holder** | Option<**bool**> | Gets or sets a value indicating whether this instance is place holder. | [optional]
**number** | Option<**String**> | Gets or sets the number. | [optional]
**channel_number** | Option<**String**> |  | [optional]
**index_number** | Option<**i32**> | Gets or sets the index number. | [optional]
**index_number_end** | Option<**i32**> | Gets or sets the index number end. | [optional]
**parent_index_number** | Option<**i32**> | Gets or sets the parent index number. | [optional]
**remote_trailers** | Option<[**Vec<crate::models::MediaUrl>**](MediaUrl.md)> | Gets or sets the trailer urls. | [optional]
**provider_ids** | Option<**::std::collections::HashMap<String, String>**> | Gets or sets the provider ids. | [optional]
**is_hd** | Option<**bool**> | Gets or sets a value indicating whether this instance is HD. | [optional]
**is_folder** | Option<**bool**> | Gets or sets a value indicating whether this instance is folder. | [optional]
**parent_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets the parent id. | [optional]
**r#type** | Option<[**crate::models::BaseItemKind**](BaseItemKind.md)> |  | [optional]
**people** | Option<[**Vec<crate::models::BaseItemPerson>**](BaseItemPerson.md)> | Gets or sets the people. | [optional]
**studios** | Option<[**Vec<crate::models::NameGuidPair>**](NameGuidPair.md)> | Gets or sets the studios. | [optional]
**genre_items** | Option<[**Vec<crate::models::NameGuidPair>**](NameGuidPair.md)> |  | [optional]
**parent_logo_item_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets wether the item has a logo, this will hold the Id of the Parent that has one. | [optional]
**parent_backdrop_item_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets wether the item has any backdrops, this will hold the Id of the Parent that has one. | [optional]
**parent_backdrop_image_tags** | Option<**Vec<String>**> | Gets or sets the parent backdrop image tags. | [optional]
**local_trailer_count** | Option<**i32**> | Gets or sets the local trailer count. | [optional]
**user_data** | Option<[**crate::models::BaseItemDtoUserData**](BaseItemDto_UserData.md)> |  | [optional]
**recursive_item_count** | Option<**i32**> | Gets or sets the recursive item count. | [optional]
**child_count** | Option<**i32**> | Gets or sets the child count. | [optional]
**series_name** | Option<**String**> | Gets or sets the name of the series. | [optional]
**series_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets the series id. | [optional]
**season_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets the season identifier. | [optional]
**special_feature_count** | Option<**i32**> | Gets or sets the special feature count. | [optional]
**display_preferences_id** | Option<**String**> | Gets or sets the display preferences id. | [optional]
**status** | Option<**String**> | Gets or sets the status. | [optional]
**air_time** | Option<**String**> | Gets or sets the air time. | [optional]
**air_days** | Option<[**Vec<crate::models::DayOfWeek>**](DayOfWeek.md)> | Gets or sets the air days. | [optional]
**tags** | Option<**Vec<String>**> | Gets or sets the tags. | [optional]
**primary_image_aspect_ratio** | Option<**f64**> | Gets or sets the primary image aspect ratio, after image enhancements. | [optional]
**artists** | Option<**Vec<String>**> | Gets or sets the artists. | [optional]
**artist_items** | Option<[**Vec<crate::models::NameGuidPair>**](NameGuidPair.md)> | Gets or sets the artist items. | [optional]
**album** | Option<**String**> | Gets or sets the album. | [optional]
**collection_type** | Option<**String**> | Gets or sets the type of the collection. | [optional]
**display_order** | Option<**String**> | Gets or sets the display order. | [optional]
**album_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets the album id. | [optional]
**album_primary_image_tag** | Option<**String**> | Gets or sets the album image tag. | [optional]
**series_primary_image_tag** | Option<**String**> | Gets or sets the series primary image tag. | [optional]
**album_artist** | Option<**String**> | Gets or sets the album artist. | [optional]
**album_artists** | Option<[**Vec<crate::models::NameGuidPair>**](NameGuidPair.md)> | Gets or sets the album artists. | [optional]
**season_name** | Option<**String**> | Gets or sets the name of the season. | [optional]
**media_streams** | Option<[**Vec<crate::models::MediaStream>**](MediaStream.md)> | Gets or sets the media streams. | [optional]
**video_type** | Option<[**crate::models::VideoType**](VideoType.md)> |  | [optional]
**part_count** | Option<**i32**> | Gets or sets the part count. | [optional]
**media_source_count** | Option<**i32**> |  | [optional]
**image_tags** | Option<**::std::collections::HashMap<String, String>**> | Gets or sets the image tags. | [optional]
**backdrop_image_tags** | Option<**Vec<String>**> | Gets or sets the backdrop image tags. | [optional]
**screenshot_image_tags** | Option<**Vec<String>**> | Gets or sets the screenshot image tags. | [optional]
**parent_logo_image_tag** | Option<**String**> | Gets or sets the parent logo image tag. | [optional]
**parent_art_item_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets wether the item has fan art, this will hold the Id of the Parent that has one. | [optional]
**parent_art_image_tag** | Option<**String**> | Gets or sets the parent art image tag. | [optional]
**series_thumb_image_tag** | Option<**String**> | Gets or sets the series thumb image tag. | [optional]
**image_blur_hashes** | Option<[**crate::models::BaseItemDtoImageBlurHashes**](BaseItemDto_ImageBlurHashes.md)> |  | [optional]
**series_studio** | Option<**String**> | Gets or sets the series studio. | [optional]
**parent_thumb_item_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Gets or sets the parent thumb item id. | [optional]
**parent_thumb_image_tag** | Option<**String**> | Gets or sets the parent thumb image tag. | [optional]
**parent_primary_image_item_id** | Option<**String**> | Gets or sets the parent primary image item identifier. | [optional]
**parent_primary_image_tag** | Option<**String**> | Gets or sets the parent primary image tag. | [optional]
**chapters** | Option<[**Vec<crate::models::ChapterInfo>**](ChapterInfo.md)> | Gets or sets the chapters. | [optional]
**location_type** | Option<[**crate::models::LocationType**](LocationType.md)> |  | [optional]
**iso_type** | Option<[**crate::models::IsoType**](IsoType.md)> |  | [optional]
**media_type** | Option<**String**> | Gets or sets the type of the media. | [optional]
**end_date** | Option<**String**> | Gets or sets the end date. | [optional]
**locked_fields** | Option<[**Vec<crate::models::MetadataField>**](MetadataField.md)> | Gets or sets the locked fields. | [optional]
**trailer_count** | Option<**i32**> | Gets or sets the trailer count. | [optional]
**movie_count** | Option<**i32**> | Gets or sets the movie count. | [optional]
**series_count** | Option<**i32**> | Gets or sets the series count. | [optional]
**program_count** | Option<**i32**> |  | [optional]
**episode_count** | Option<**i32**> | Gets or sets the episode count. | [optional]
**song_count** | Option<**i32**> | Gets or sets the song count. | [optional]
**album_count** | Option<**i32**> | Gets or sets the album count. | [optional]
**artist_count** | Option<**i32**> |  | [optional]
**music_video_count** | Option<**i32**> | Gets or sets the music video count. | [optional]
**lock_data** | Option<**bool**> | Gets or sets a value indicating whether [enable internet providers]. | [optional]
**width** | Option<**i32**> |  | [optional]
**height** | Option<**i32**> |  | [optional]
**camera_make** | Option<**String**> |  | [optional]
**camera_model** | Option<**String**> |  | [optional]
**software** | Option<**String**> |  | [optional]
**exposure_time** | Option<**f64**> |  | [optional]
**focal_length** | Option<**f64**> |  | [optional]
**image_orientation** | Option<[**crate::models::ImageOrientation**](ImageOrientation.md)> |  | [optional]
**aperture** | Option<**f64**> |  | [optional]
**shutter_speed** | Option<**f64**> |  | [optional]
**latitude** | Option<**f64**> |  | [optional]
**longitude** | Option<**f64**> |  | [optional]
**altitude** | Option<**f64**> |  | [optional]
**iso_speed_rating** | Option<**i32**> |  | [optional]
**series_timer_id** | Option<**String**> | Gets or sets the series timer identifier. | [optional]
**program_id** | Option<**String**> | Gets or sets the program identifier. | [optional]
**channel_primary_image_tag** | Option<**String**> | Gets or sets the channel primary image tag. | [optional]
**start_date** | Option<**String**> | Gets or sets the start date of the recording, in UTC. | [optional]
**completion_percentage** | Option<**f64**> | Gets or sets the completion percentage. | [optional]
**is_repeat** | Option<**bool**> | Gets or sets a value indicating whether this instance is repeat. | [optional]
**episode_title** | Option<**String**> | Gets or sets the episode title. | [optional]
**channel_type** | Option<[**crate::models::ChannelType**](ChannelType.md)> |  | [optional]
**audio** | Option<[**crate::models::ProgramAudio**](ProgramAudio.md)> |  | [optional]
**is_movie** | Option<**bool**> | Gets or sets a value indicating whether this instance is movie. | [optional]
**is_sports** | Option<**bool**> | Gets or sets a value indicating whether this instance is sports. | [optional]
**is_series** | Option<**bool**> | Gets or sets a value indicating whether this instance is series. | [optional]
**is_live** | Option<**bool**> | Gets or sets a value indicating whether this instance is live. | [optional]
**is_news** | Option<**bool**> | Gets or sets a value indicating whether this instance is news. | [optional]
**is_kids** | Option<**bool**> | Gets or sets a value indicating whether this instance is kids. | [optional]
**is_premiere** | Option<**bool**> | Gets or sets a value indicating whether this instance is premiere. | [optional]
**timer_id** | Option<**String**> | Gets or sets the timer identifier. | [optional]
**current_program** | Option<[**crate::models::BaseItemDtoCurrentProgram**](BaseItemDto_CurrentProgram.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

