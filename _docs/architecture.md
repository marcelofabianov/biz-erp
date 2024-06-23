# Biz Retail

## Visão Geral da arquitetura do projeto Biz Retail.

O projeto consiste na implementação de uma arquitetura de microservices distribuídos, onde cada serviço é responsável por domínios específicos do negócio. A comunicação entre os serviços é feita através de gRPC, enquanto a interação com clientes externos ocorre via HTTPS através de um API Gateway. Eventos são gerados pelos microservices e enviados para Kafka, onde são armazenados para fins de auditoria e traceabilidade.

## Componentes Principais

**Microservices:**

- [Cost Center Service](../cost_center_service/README.md)
- [Client Contractor Service](../client_contractor_service/README.md)
- [Notifications Service](../notifications_service/README.md)
- [Identity Provider Service](../identity_provider_service/README.md)
- [Team Service](../team_service/README.md)
- [Audit Service](../audit_service/README.md)

**API Gateway:**

Utilizado para expor endpoints HTTPS para clientes externos e rotear requisições para os microservices via gRPC internamente.

**Katka:**

Plataforma de streaming utilizada para a troca de mensagens assíncronas entre os microservices e para o armazenamento de eventos.

**Armazenamento de Eventos Trace**

Escolha entre MongoDB ou DynamoDB para armazenar eventos gerados pelos microservices, garantindo rastreabilidade e capacidade de auditoria.

**Secret Manager:**

Estudo em aberto para escolha de um serviço de gerenciamento de segredos.

**Feature Flags:**

Estudo em aberto para escolha de um serviço de gerenciamento de Feature Flags.

**Identity and Access Management:**

O Padrao OAuth2 para o serviço de autenticação e autorização junto a API Gateway.
Ate o momento foi definido o uso do serviço Keycloak para gerenciamento de identidade e acesso.
Mas pode ser alterado conforme a necessidade do projeto.

**Observability:**

Utilização do OpenTelemetry para coleta de métricas e rastreamento de requisições.
Integração com ElasticSearch e Kibana para visualização de logs.

**Estrutura de Eventos:**

A estrutura de eventos desempenha um papel fundamental em arquiteturas de microservices, especialmente quando se trata de comunicação assíncrona e integração entre sistemas distribuídos.

**Producer Service e Producer Service ID**

- **Producer Service**: Identifica o microserviço que gerou o evento. Isso é útil para rastrear a origem dos eventos e para fins de auditoria.

- **Producer Service ID**: Um ID único atribuído ao produtor do evento, garantindo que cada instância do serviço tenha uma identificação exclusiva.

**Trace ID**

Um ID único utilizado para rastrear o fluxo de um evento através de vários microserviços. Isso permite correlacionar eventos relacionados e facilita a depuração de problemas em sistemas distribuídos.

**Timestamp**

Indica o momento exato em que o evento foi gerado. Essa informação é crucial para análise de dados, monitoramento de desempenho e para garantir a ordem de eventos em sistemas distribuídos.

**Event Type**

Define o tipo de evento que ocorreu, como "account.removed" no exemplo fornecido. Cada tipo de evento geralmente corresponde a uma ação ou estado específico no sistema.

**Payload**

Contém os dados específicos associados ao evento, como informações sobre uma conta que foi removida no exemplo dado. O payload varia de acordo com o tipo de evento e pode incluir diferentes campos e estruturas de dados.

**Metadata**

Os metadados são informações adicionais que oferecem contexto e detalhes relevantes sobre o evento. Eles podem variar de acordo com o tipo específico de evento e suas necessidades particulares. Existem campos obrigatório a serem informado será apresentado no próximo tópico.

## Arquitetura de Event Sourcing

O Event Sourcing é uma técnica que consiste em armazenar todos os eventos que ocorrem no sistema, permitindo reconstruir o estado atual a partir do histórico de eventos. Essa abordagem é útil para garantir a consistência e a integridade dos dados, além de possibilitar a auditoria e a rastreabilidade de todas as operações realizadas no sistema.

Para implementar o Event Sourcing, cada microserviço algo manipular registros em seu banco de dados deverá gerar um evento correspondente, que será enviado para o kafka e consumido pelo microservice audit. O microservice audit será responsável por armazenar os eventos em um banco de dados dedicado, garantindo a rastreabilidade e a integridade dos dados.

Nenhum registro deve ser excluído do banco de dados do audit service, em vez disso será versionado e marcado como desativado.
Isso garante que todos os eventos sejam mantidos para fins de auditoria e rastreabilidade.

Cada registro no banco de dados do audit service terá os campos para garantir a implementação do Event Sourcing são:

- **action_type**: Tipo de ação que gerou o evento, como `new`, `update`, `delete`. `disable`, `enable`.
- **action_at**: Data e hora em que a ação foi realizada.
- **version**: Versão do registro.
- **is_active**: Indica se o registro está ativo ou desativado.

A arquitetura de Event Sourcing é composta por três componentes principais:

1. **Producer Service**: Responsável por gerar eventos e enviá-los para o Kafka.
2. **Kafka**: Plataforma de streaming utilizada para troca de mensagens assíncronas entre os microservices.
3. **Audit Service**: Microserviço responsável por consumir os eventos do Kafka e armazená-los em um banco de dados dedicado.

## Estrutura de Eventos

Os eventos gerados pelos microservices devem seguir uma estrutura padronizada para garantir a consistência e a interoperabilidade entre os sistemas. A estrutura de eventos deve conter os seguintes campos:

- **producer_service**: Identifica o microserviço que gerou o evento.
- **producer_service_id**: ID único atribuído ao produtor do evento.
- **trace_id**: ID único utilizado para rastrear o fluxo de um evento.
- **timestamp**: Data e hora em que o evento foi gerado.
- **event_type**: Tipo de evento que ocorreu.
- **payload**: Dados específicos associados ao evento.
- **metadata**: Informações adicionais sobre o evento.

Os metadados são informações adicionais que oferecem contexto e detalhes relevantes sobre o evento. Eles podem variar de acordo com o tipo específico de evento e suas necessidades particulares. Existem campos obrigatórios a serem inseridos:

- **owner_id**: ID do agente responsável por gerar o evento.
- **owner_ip**: Endereço IP do agente responsável pelo evento.
- **owner_role**: Papel desempenhado pelo agente na geração do evento.
- **owner_type**: Tipo do agente responsável pelo evento. Poder ser um `user` ou `service`.
- **event_schema_version**: Versão do esquema do evento.
- **ownership_id**: ID do proprietário dos dados relacionados ao evento. Para comprimento das normativas do LGPD.

Exemplo de um evento:

```json
{
  "producer_service": "team_service",
  "producer_service_id": "123e4567-e89b-12d3-a456-426614174000",
  "trace_id": "7b3bf470-9456-11e8-9eb6-529269fb1459",
  "timestamp": "2024-06-22T12:00:00Z",
  "event_type": "account.created",
  "payload": {
    "id": 123,
    "public_id": "123e4567-e89b-12d3-a456-426614174000",
    "document_registry": "12345678900",
    "name": "John Doe"
  },
  "metadata": {
    "event_schema_version": "1.0",
    "environment": "production",
    "user_id": "789",
    "user_role": "admin",
    "user_ip": "192.168.1.100",
    "ownership_id": "1b08116e-74ff-4e71-8a17-26011cfea33f",
  }
}
```

O evento é `account.created` e contém informações sobre a criação de uma conta, como o ID, o nome e o registro do documento. Os metadados incluem detalhes sobre o ambiente, o usuário que gerou o evento e o ID do proprietário dos dados relacionados ao evento.

O microservice audit será responsável por consumir os eventos do kafka publicados no topic `audit` e armazená-los em um banco de dados DynamoDB ou MongoDB. Após poderá acrescentar mais informações sobre o evento, como o ID do proprietário dos dados, para garantir a conformidade com as normativas de privacidade de dados, como o LGPD e por fim informação do registro como `is_active`, `version`, `action_type` e `action_at`.

## Microservice Audit e Tabela Ownership

O Microservice Audit, responsável por registrar todos os eventos, terá uma tabela ownerships dedicada para armazenar detalhes adicionais sobre o proprietário dos dados. Esta abordagem organiza de forma estruturada as informações necessárias para o cumprimento das normativas de privacidade de dados, como o LGPD.
