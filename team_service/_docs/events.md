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
  "topic_name": "team-service.account.created.v1",
  "producer_service": "team-service",
  "producer_service_id": "3b4a7939-6e73-48c9-9f2a-a3f7a929c50a",
  "trace_id": "92252b34-78bb-44fb-823c-32652ef852ac",
  "timestamp": "2024-06-24T08:59:51.390287887Z",
  "event_type": "team-service.v1.account.created",
  "payload": {
    "id": 0,
    "public_id": "f77b03f7-38e8-4060-ada8-9cd310f1b695",
    "name": "Company Example",
    "document_registry": "12345678909",
    "created_at": "2024-06-24T08:59:51.388166054Z",
    "updated_at": "2024-06-24T08:59:51.388168179Z",
    "disabled_at": null,
    "deleted_at": null
  },
  "metadata": {
    "event_schema_version": "1",
    "environment": "development",
    "owner": {
      "id": 434,
      "public_id": "efb6517c-3ebf-4a0a-ac35-86f70c4afb52",
      "role": "admin",
      "ip": null,
      "owner_type": "USER"
    },
    "ownership_id": "7b31d6ad-8f44-4f3f-b923-0fa1ae8e5c7a"
  }
}
```
