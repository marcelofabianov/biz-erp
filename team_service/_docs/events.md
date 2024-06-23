# Team Service

Relação de eventos do serviço de gerenciamento de equipes, cargos, departamentos e pessoas.

## Eventos

Os eventos são emitidos e ouvidos através do serviço de mensageria.

Eventos ouvidos por este serviço.

- `account.created` - Conta criada
- `account.updated` - Conta atualizada
- `account.removed` - Conta removida
- `account.disabled` - Conta desativada
- `account.enabled` - Conta reativada

Eventos emitidos por este serviço.

- `person.created` - Pessoa criada
- `person.updated` - Pessoa atualizada
- `person.removed` - Pessoa removida
- `person.disabled` - Pessoa desativada
- `person.enabled` - Pessoa reativada

## Schema de Eventos

Os eventos emitidos devem seguir um schema específico.

ID da conta quando zero é uma nova conta criada o banco de dados irá gerar um ID

`account.created`

```json
{
  "producer_service": "team_service",
  "producer_service_id": "123e4567-e89b-12d3-a456-426614174000",
  "trace_id": "7b3bf470-9456-11e8-9eb6-529269fb1459",
  "timestamp": "2024-06-22T12:00:00Z",
  "event_type": "account.created",
  "payload": {
    "id": 0,
    "public_id": "a51f25d8-9be0-468c-9bd4-2b16866a57c6",
    "name": "Marcelo Fabiano",
    "document_registry": "123456789",
    "created_at": "2024-06-23T00:58:12.056414Z",
    "updated_at": "2024-06-23T00:58:12.056415Z",
    "disabled_at": null,
    "deleted_at": null
  },
  "metadata": {
    "event_schema_version": "1.0",
    "environment": "production",
    "owner": {
      "id": 343,
      "public_id": "42b0e0c8-9a93-429e-be47-72b70db57535",
      "role": "admin",
      "ip": "192.168.1.100",
      "type": "user",
    }
    "ownership_id": "1b08116e-74ff-4e71-8a17-26011cfea33f",
  }
}

```

`account.updated`

```json
{
  "producer_service": "team_service",
  "producer_service_id": "123e4567-e89b-12d3-a456-426614174000",
  "trace_id": "7b3bf470-9456-11e8-9eb6-529269fb1459",
  "timestamp": "2024-06-22T12:00:00Z",
  "event_type": "account.updated",
  "payload": {
    "id": 123,
    "public_id": "123e4567-e89b-12d3-a456-426614174000",
    "document_registry": "12345678900",
    "name": "John Doe"
  },
  "metadata": {
    "event_schema_version": "1.0",
    "environment": "production",
    "owner_id": "789",
    "owner_role": "admin",
    "owner_ip": "192.168.1.100",
    "owner_type": "user",
    "ownership_id": "1b08116e-74ff-4e71-8a17-26011cfea33f",
  }
}
```

`account.removed`

```json
{
  "producer_service": "team_service",
  "producer_service_id": "123e4567-e89b-12d3-a456-426614174000",
  "trace_id": "7b3bf470-9456-11e8-9eb6-529269fb1459",
  "timestamp": "2024-06-22T12:00:00Z",
  "event_type": "account.removed",
  "payload": {
    "id": 123,
    "public_id": "123e4567-e89b-12d3-a456-426614174000",
    "document_registry": "12345678900",
    "deleted_at": "2024-06-22T12:00:00Z"
  },
  "metadata": {
    "event_schema_version": "1.0",
    "environment": "production",
    "owner_id": "789",
    "owner_role": "admin",
    "owner_ip": "192.168.1.100",
    "owner_type": "user",
    "ownership_id": "1b08116e-74ff-4e71-8a17-26011cfea33f",
  }
}
```

`account.disabled`

```json
{
  "producer_service": "team_service",
  "producer_service_id": "123e4567-e89b-12d3-a456-426614174000",
  "trace_id": "7b3bf470-9456-11e8-9eb6-529269fb1459",
  "timestamp": "2024-06-22T12:00:00Z",
  "event_type": "account.disabled",
  "payload": {
    "id": 123,
    "public_id": "123e4567-e89b-12d3-a456-426614174000",
    "document_registry": "12345678900",
    "disabled_at": "2024-06-22T12:00:00Z"
  },
  "metadata": {
    "event_schema_version": "1.0",
    "environment": "production",
    "owner_id": "789",
    "owner_role": "admin",
    "owner_ip": "192.168.1.100",
    "owner_type": "user",
    "ownership_id": "1b08116e-74ff-4e71-8a17-26011cfea33f",
  }
}
```

`account.enabled`

```json
{
  "producer_service": "team_service",
  "producer_service_id": "123e4567-e89b-12d3-a456-426614174000",
  "trace_id": "7b3bf470-9456-11e8-9eb6-529269fb1459",
  "timestamp": "2024-06-22T12:00:00Z",
  "event_type": "account.enabled",
  "payload": {
    "id": 123,
    "public_id": "123e4567-e89b-12d3-a456-426614174000",
    "document_registry": "12345678900",
    "disabled_at": null
  },
  "metadata": {
    "event_schema_version": "1.0",
    "environment": "production",
    "owner_id": "789",
    "owner_role": "admin",
    "owner_ip": "192.168.1.100",
    "owner_type": "user",
    "ownership_id": "1b08116e-74ff-4e71-8a17-26011cfea33f",
  }
}
```

`person.created`

```json
{
  "producer_service": "team_service",
  "producer_service_id": "123e4567-e89b-12d3-a456-426614174000",
  "trace_id": "7b3bf470-9456-11e8-9eb6-529269fb1459",
  "timestamp": "2024-06-22T12:00:00Z",
  "event_type": "person.created",
  "payload": {
    "id": 123,
    "public_id": "123e4567-e89b-12d3-a456-426614174000",
    "account_id": 123,
    "team_id": 123,
    "role_id": 123
  },
  "metadata": {
    "event_schema_version": "1.0",
    "environment": "production",
    "owner_id": "789",
    "owner_role": "admin",
    "owner_ip": "192.168.1.100",
    "owner_type": "user",
    "ownership_id": "1b08116e-74ff-4e71-8a17-26011cfea33f",
  }
}
```

`person.updated`

```json
{
  "producer_service": "team_service",
  "producer_service_id": "123e4567-e89b-12d3-a456-426614174000",
  "trace_id": "7b3bf470-9456-11e8-9eb6-529269fb1459",
  "timestamp": "2024-06-22T12:00:00Z",
  "event_type": "person.updated",
  "payload": {
    "id": 123,
    "public_id": "123e4567-e89b-12d3-a456-426614174000",
    "account_id": 123,
    "team_id": 123,
    "role_id": 123
  },
  "metadata": {
    "event_schema_version": "1.0",
    "environment": "production",
    "owner_id": "789",
    "owner_role": "admin",
    "owner_ip": "192.168.1.100",
    "owner_type": "user",
    "ownership_id": "1b08116e-74ff-4e71-8a17-26011cfea33f",
  }
}
```

`person.removed`

```json
{
  "producer_service": "team_service",
  "producer_service_id": "123e4567-e89b-12d3-a456-426614174000",
  "trace_id": "7b3bf470-9456-11e8-9eb6-529269fb1459",
  "timestamp": "2024-06-22T12:00:00Z",
  "event_type": "person.removed",
  "payload": {
    "id": 123,
    "public_id": "123e4567-e89b-12d3-a456-426614174000",
    "deleted_at": "2024-06-22T12:00:00Z"
  },
  "metadata": {
    "event_schema_version": "1.0",
    "environment": "production",
    "owner_id": "789",
    "owner_role": "admin",
    "owner_ip": "192.168.1.100",
    "owner_type": "user",
    "ownership_id": "1b08116e-74ff-4e71-8a17-26011cfea33f",
  }
}
```

`person.disabled`

```json
{
  "producer_service": "team_service",
  "producer_service_id": "123e4567-e89b-12d3-a456-426614174000",
  "trace_id": "7b3bf470-9456-11e8-9eb6-529269fb1459",
  "timestamp": "2024-06-22T12:00:00Z",
  "event_type": "person.disabled",
  "payload": {
    "id": 123,
    "public_id": "123e4567-e89b-12d3-a456-426614174000",
    "disabled_at": "2024-06-22T12:00:00Z"
  },
  "metadata": {
    "event_schema_version": "1.0",
    "environment": "production",
    "owner_id": "789",
    "owner_role": "admin",
    "owner_ip": "192.168.1.100",
    "owner_type": "user",
    "ownership_id": "1b08116e-74ff-4e71-8a17-26011cfea33f",
  }
}
```

`person.enabled`

```json
{
  "producer_service": "team_service",
  "producer_service_id": "123e4567-e89b-12d3-a456-426614174000",
  "trace_id": "7b3bf470-9456-11e8-9eb6-529269fb1459",
  "timestamp": "2024-06-22T12:00:00Z",
  "event_type": "person.enabled",
  "payload": {
    "id": 123,
    "public_id": "123e4567-e89b-12d3-a456-426614174000",
    "disabled_at": null
  },
  "metadata": {
    "event_schema_version": "1.0",
    "environment": "production",
    "owner_id": "789",
    "owner_role": "admin",
    "owner_ip": "192.168.1.100",
    "owner_type": "user",
    "ownership_id": "1b08116e-74ff-4e71-8a17-26011cfea33f",
  }
}
```
