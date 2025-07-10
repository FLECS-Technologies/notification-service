# NotificationsApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**notificationsPost**](NotificationsApi.md#notificationsPost) | **POST** /notifications | Send notification via the default service |
| [**servicesIdNotificationsPost**](NotificationsApi.md#servicesIdNotificationsPost) | **POST** /services/{id}/notifications | Send notification via the notification service, may contain additional options |


<a name="notificationsPost"></a>
# **notificationsPost**
> notificationsPost(\_notifications\_post\_request)

Send notification via the default service

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **\_notifications\_post\_request** | [**_notifications_post_request**](../Models/_notifications_post_request.md)|  | |

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="servicesIdNotificationsPost"></a>
# **servicesIdNotificationsPost**
> servicesIdNotificationsPost(id, \_services\_\_id\_\_notifications\_post\_request)

Send notification via the notification service, may contain additional options

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |
| **\_services\_\_id\_\_notifications\_post\_request** | [**_services__id__notifications_post_request**](../Models/_services__id__notifications_post_request.md)|  | |

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

