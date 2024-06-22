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

Os metadados são informações adicionais que oferecem contexto e detalhes relevantes sobre o evento. Eles podem variar de acordo com o tipo específico de evento e suas necessidades particulares. Os dados obrigatórios incluem o `owner_id`, `owner_ip`, `owner_role`, e `owner_type`. O termo "owner" refere-se ao agente responsável por gerar o evento, podendo ser um usuário ou um serviço. O owner_type especifica se o agente é um usuário ou um serviço. `owner_role` indica o papel desempenhado pelo agente na geração do evento, enquanto `owner_ip` registra o endereço IP do agente. Além disso, os metadados também podem incluir a versão do esquema do evento `event_schema_version`, fundamental para garantir a consistência e compatibilidade na evolução do software. O `ownership_id` identifica o proprietário dos dados relacionados ao evento, facilitando a organização e garantindo conformidade com regulamentações como o LGPD.

## Microservice Audit e Tabela Ownership

O Microservice Audit, responsável por registrar todos os eventos, terá uma tabela ownerships dedicada para armazenar detalhes adicionais sobre o proprietário dos dados. Esta abordagem organiza de forma estruturada as informações necessárias para o cumprimento das normativas de privacidade de dados, como o LGPD.
