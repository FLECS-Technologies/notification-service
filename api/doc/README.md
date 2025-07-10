# Documentation for Notis API Specification

<a name="documentation-for-api-endpoints"></a>
## Documentation for API Endpoints

All URIs are relative to *http://localhost*

| Class | Method | HTTP request | Description |
|------------ | ------------- | ------------- | -------------|
| *NotificationsApi* | [**notificationsPost**](Apis/NotificationsApi.md#notificationspost) | **POST** /notifications | Send notification via the default service |
*NotificationsApi* | [**servicesIdNotificationsPost**](Apis/NotificationsApi.md#servicesidnotificationspost) | **POST** /services/{id}/notifications | Send notification via the notification service, may contain additional options |
| *ServicesApi* | [**defaultServiceDelete**](Apis/ServicesApi.md#defaultservicedelete) | **DELETE** /default_service | Remove the reference to the default service |
*ServicesApi* | [**defaultServiceGet**](Apis/ServicesApi.md#defaultserviceget) | **GET** /default_service | Get the default service and its type |
*ServicesApi* | [**defaultServicePost**](Apis/ServicesApi.md#defaultservicepost) | **POST** /default_service | Set the default service |
*ServicesApi* | [**schemaServiceTypesServiceTypeConfigGet**](Apis/ServicesApi.md#schemaservicetypesservicetypeconfigget) | **GET** /schema/service_types/{service_type}/config | Get the configuration schema of a service type |
*ServicesApi* | [**servicesGet**](Apis/ServicesApi.md#servicesget) | **GET** /services | Get a list of all registered services, their type and the default service |
*ServicesApi* | [**servicesIdConfigGet**](Apis/ServicesApi.md#servicesidconfigget) | **GET** /services/{id}/config | Get the configuration of a notification service |
*ServicesApi* | [**servicesIdConfigPatch**](Apis/ServicesApi.md#servicesidconfigpatch) | **PATCH** /services/{id}/config | Manipulate the configuration of a notification service |
*ServicesApi* | [**servicesIdConfigSchemaGet**](Apis/ServicesApi.md#servicesidconfigschemaget) | **GET** /services/{id}/config/schema | Get the configuration schema of a notification service |
*ServicesApi* | [**servicesIdDelete**](Apis/ServicesApi.md#servicesiddelete) | **DELETE** /services/{id} | Delete the notification service |
*ServicesApi* | [**servicesIdGet**](Apis/ServicesApi.md#servicesidget) | **GET** /services/{id} | Get the notification service and their type |
*ServicesApi* | [**servicesIdNotificationsSchemaGet**](Apis/ServicesApi.md#servicesidnotificationsschemaget) | **GET** /services/{id}/notifications/schema | Get the schema for sending notifications via the notification service |
*ServicesApi* | [**servicesIdPut**](Apis/ServicesApi.md#servicesidput) | **PUT** /services/{id} | Create a new notification service, or replace an existing one |


<a name="documentation-for-models"></a>
## Documentation for Models

 - [_default_service_get_200_response](./Models/_default_service_get_200_response.md)
 - [_default_service_post_request](./Models/_default_service_post_request.md)
 - [_notifications_post_request](./Models/_notifications_post_request.md)
 - [_services__id__get_200_response](./Models/_services__id__get_200_response.md)
 - [_services__id__notifications_post_request](./Models/_services__id__notifications_post_request.md)
 - [_services__id__put_request](./Models/_services__id__put_request.md)
 - [reason](./Models/reason.md)


<a name="documentation-for-authorization"></a>
## Documentation for Authorization

All endpoints do not require authorization.
