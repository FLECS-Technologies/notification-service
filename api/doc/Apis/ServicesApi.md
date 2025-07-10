# ServicesApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**defaultServiceDelete**](ServicesApi.md#defaultServiceDelete) | **DELETE** /default_service | Remove the reference to the default service |
| [**defaultServiceGet**](ServicesApi.md#defaultServiceGet) | **GET** /default_service | Get the default service and its type |
| [**defaultServicePost**](ServicesApi.md#defaultServicePost) | **POST** /default_service | Set the default service |
| [**schemaServiceTypesServiceTypeConfigGet**](ServicesApi.md#schemaServiceTypesServiceTypeConfigGet) | **GET** /schema/service_types/{service_type}/config | Get the configuration schema of a service type |
| [**servicesGet**](ServicesApi.md#servicesGet) | **GET** /services | Get a list of all registered services, their type and the default service |
| [**servicesIdConfigGet**](ServicesApi.md#servicesIdConfigGet) | **GET** /services/{id}/config | Get the configuration of a notification service |
| [**servicesIdConfigPatch**](ServicesApi.md#servicesIdConfigPatch) | **PATCH** /services/{id}/config | Manipulate the configuration of a notification service |
| [**servicesIdConfigSchemaGet**](ServicesApi.md#servicesIdConfigSchemaGet) | **GET** /services/{id}/config/schema | Get the configuration schema of a notification service |
| [**servicesIdDelete**](ServicesApi.md#servicesIdDelete) | **DELETE** /services/{id} | Delete the notification service |
| [**servicesIdGet**](ServicesApi.md#servicesIdGet) | **GET** /services/{id} | Get the notification service and their type |
| [**servicesIdNotificationsSchemaGet**](ServicesApi.md#servicesIdNotificationsSchemaGet) | **GET** /services/{id}/notifications/schema | Get the schema for sending notifications via the notification service |
| [**servicesIdPut**](ServicesApi.md#servicesIdPut) | **PUT** /services/{id} | Create a new notification service, or replace an existing one |


<a name="defaultServiceDelete"></a>
# **defaultServiceDelete**
> defaultServiceDelete()

Remove the reference to the default service

### Parameters
This endpoint does not need any parameter.

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="defaultServiceGet"></a>
# **defaultServiceGet**
> _default_service_get_200_response defaultServiceGet()

Get the default service and its type

### Parameters
This endpoint does not need any parameter.

### Return type

[**_default_service_get_200_response**](../Models/_default_service_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="defaultServicePost"></a>
# **defaultServicePost**
> defaultServicePost(\_default\_service\_post\_request)

Set the default service

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **\_default\_service\_post\_request** | [**_default_service_post_request**](../Models/_default_service_post_request.md)|  | |

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="schemaServiceTypesServiceTypeConfigGet"></a>
# **schemaServiceTypesServiceTypeConfigGet**
> Object schemaServiceTypesServiceTypeConfigGet(service\_type)

Get the configuration schema of a service type

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **service\_type** | **String**|  | [default to null] |

### Return type

**Object**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="servicesGet"></a>
# **servicesGet**
> Map servicesGet()

Get a list of all registered services, their type and the default service

### Parameters
This endpoint does not need any parameter.

### Return type

**Map**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="servicesIdConfigGet"></a>
# **servicesIdConfigGet**
> Object servicesIdConfigGet(id)

Get the configuration of a notification service

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

**Object**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="servicesIdConfigPatch"></a>
# **servicesIdConfigPatch**
> servicesIdConfigPatch(id, body)

Manipulate the configuration of a notification service

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |
| **body** | **Object**|  | |

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="servicesIdConfigSchemaGet"></a>
# **servicesIdConfigSchemaGet**
> Object servicesIdConfigSchemaGet(id, patch)

Get the configuration schema of a notification service

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |
| **patch** | **Boolean**|  | [optional] [default to null] |

### Return type

**Object**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="servicesIdDelete"></a>
# **servicesIdDelete**
> servicesIdDelete(id)

Delete the notification service

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="servicesIdGet"></a>
# **servicesIdGet**
> _services__id__get_200_response servicesIdGet(id)

Get the notification service and their type

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

[**_services__id__get_200_response**](../Models/_services__id__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="servicesIdNotificationsSchemaGet"></a>
# **servicesIdNotificationsSchemaGet**
> Object servicesIdNotificationsSchemaGet(id)

Get the schema for sending notifications via the notification service

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |

### Return type

**Object**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="servicesIdPut"></a>
# **servicesIdPut**
> servicesIdPut(id, \_services\_\_id\_\_put\_request)

Create a new notification service, or replace an existing one

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **id** | **String**|  | [default to null] |
| **\_services\_\_id\_\_put\_request** | [**_services__id__put_request**](../Models/_services__id__put_request.md)|  | |

### Return type

null (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

