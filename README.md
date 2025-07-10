# Notification-Service Notis

This application provides an API to easily send notifications via configurable notification services.

## Configuration

Notis is configured via a json config file. The file location can be controlled via an environment variable
`NOTIS_CONFIG_PATH`.

### General configuration

| Parameter                    | Optional | Description                                                                                                                                             |
|------------------------------|----------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
| port                         | No       | The port on which notis listens for requests                                                                                                            |
| trace_filter                 | Yes      | Controls which trace levels are shown in the logs, if omitted `info` is used                                                                            |
| notification_services        | Yes      | A map of strings to [notification service configurations](#notification-services), if omitted an empty map will be used (i.e. no service is configured) |
| default_notification_service | Yes      | References a notification service configured in `notification_services` by its key, if omitted no default service will be available                     |

### Notification services

The following notification service configurations are supported. If your preferred service is missing contact us or open
a pull request.

> [!IMPORTANT]
> You have to specify the type property (e.g. "type": "SMTP") for each service configuration even if it is not required
> by the schema

#### SMTP

This service can send notifications via email. You have to configure a smtp server, sender and receivers.

<details>
  <summary>Example configuration</summary>

```json
{
  "type": "SMTP",
  "server_url": "smtp.example.com",
  "credentials": {
    "authentication_identity": "my_user",
    "secret": "my_password"
  },
  "connection_type": "Tls",
  "auth_mechanism": "Login",
  "sender": {
    "name": "Alice",
    "email": "alice@mail.com"
  },
  "receivers": [
    {
      "name": "Bob",
      "email": "bob@bob-self-hosting.com"
    },
    {
      "name": "Charlie",
      "email": "charlie@mail.ca"
    }
  ]
}
```

</details>
<details>
  <summary>Configuration schema</summary>

```json
{
  "$defs": {
    "ConnectionType": {
      "enum": [
        "Tls",
        "StartTls",
        "PlainUnsecure"
      ],
      "type": "string"
    },
    "Credentials": {
      "properties": {
        "authentication_identity": {
          "type": "string"
        },
        "secret": {
          "type": "string"
        }
      },
      "required": [
        "authentication_identity",
        "secret"
      ],
      "type": "object"
    },
    "Mailbox": {
      "properties": {
        "email": {
          "format": "email",
          "type": "string"
        },
        "name": {
          "type": [
            "string",
            "null"
          ]
        }
      },
      "required": [
        "email"
      ],
      "type": "object"
    },
    "Mechanism": {
      "enum": [
        "Plain",
        "Login",
        "Xoauth2"
      ],
      "type": "string"
    }
  },
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "properties": {
    "auth_mechanism": {
      "anyOf": [
        {
          "$ref": "#/$defs/Mechanism"
        },
        {
          "type": "null"
        }
      ]
    },
    "connection_type": {
      "$ref": "#/$defs/ConnectionType"
    },
    "credentials": {
      "$ref": "#/$defs/Credentials"
    },
    "receivers": {
      "items": {
        "$ref": "#/$defs/Mailbox"
      },
      "type": "array"
    },
    "sender": {
      "$ref": "#/$defs/Mailbox"
    },
    "server_url": {
      "type": "string"
    }
  },
  "required": [
    "server_url",
    "credentials",
    "connection_type",
    "sender",
    "receivers"
  ],
  "title": "Config",
  "type": "object"
}
```

</details>

#### Log

This service just logs all notifications in the log of notis and is meant for testing and debugging. The only
configurable property is the level notifications should be logged under.

<details>
  <summary>Example configuration</summary>

```json
{
  "type": "LOG",
  "level": "Info"
}
```

</details>
<details>
  <summary>Configuration schema</summary>

```json
{
  "$defs": {
    "Level": {
      "enum": [
        "Error",
        "Warn",
        "Info",
        "Debug",
        "Trace"
      ],
      "type": "string"
    }
  },
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "properties": {
    "level": {
      "$ref": "#/$defs/Level"
    }
  },
  "required": [
    "level"
  ],
  "title": "Config",
  "type": "object"
}
```

</details>

## API

Notis provides an http REST API. The specification can be found at [./api/openapi.yaml](./api/openapi.yaml) with a
generated documentation [here](./api/doc/README.md).