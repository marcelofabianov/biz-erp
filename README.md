# BizRetail ERP

**Este projeto é um estudo de aplicação de arquitetura de microsserviços feito por mim não deve ser comercializado!!!**
**Toda a documentação é fictícia e não deve ser levada a sério!**
**O projeto esta sendo mantido por um repositório apenas para facilitar o desenvolvimento feito por mim!**

BizRetail ERP é um software de gestão B2B (Business to Business) que tem como objetivo principal a gestão de empresas de comércio e varejo. O sistema é composto por módulos que atendem as principais áreas de uma empresa, como: vendas, compras, estoque, financeiro, contábil, fiscal, entre outros.

## Tecnologias

- [Rust](https://www.rust-lang.org/)
- [PostgreSQL](https://www.postgresql.org/)
- [Kafka](https://kafka.apache.org/)
- [gRPC](https://grpc.io/)
- [Protocol Buffers](https://developers.google.com/protocol-buffers)
- [OpenTelemetry](https://opentelemetry.io/)
- [Kong](https://konghq.com/)

## Arquitetura

O sistema é composto por diversos microserviços que se comunicam através do protocolo gRPC e Kafka. Cada microserviço é responsável por uma área específica. A comunicação entre os microserviços é feita através de eventos, que são publicados em tópicos do Kafka. Cada microserviço é responsável por consumir os eventos que são relevantes para o seu contexto.

## Microserviços

- [Cost Center Service](cost_center_service/README.md)
- [Client Contractor Service](client_contractor_service/README.md)
- [Notifications Service](notifications_service/README.md)
- [Identity Provider Service](identity_provider_service/README.md)
- [Team Service](team_service/README.md)

### Estudos em aberto

Feature Flags

Uso de serviços para gerenciamento de Features Flags.

opções:

- [Unleash](https://unleash.github.io/)
- [LaunchDarkly](https://launchdarkly.com/)
- [ConfigCat](https://configcat.com/)
- [Flagr](https://checkr.github.io/flagr/)
- [Togglz](https://www.togglz.org/)
- [FF4J](https://ff4j.github.io/)
- [Flipper](https://www.ff4j.org/)
- [BulletTrain](https://bullet-train.io/)

Secret Manager

Uso de serviços para gerenciamento de segredos.

opções:

- [Vault](https://www.vaultproject.io/)
- [Confidant](https://lyft.github.io/confidant/)
- [Sops](https://getsops.io/)
- [SealedSecrets](https://sealed-secrets.netlify.app/)
- [Azure Key Vault](https://azure.microsoft.com/pt-br/services/key-vault/)
- [AWS Secrets Manager](https://aws.amazon.com/pt/secrets-manager/)
- [GCP Secret Manager](https://cloud.google.com/secret-manager)

Identity and Access Management

Uso de serviços para gerenciamento de identidade e acesso.

opções:

- [Keycloak](https://www.keycloak.org/)
- [Auth0](https://auth0.com/)
- [Okta](https://www.okta.com/)
- [Cognito](https://aws.amazon.com/pt/cognito/)
- [Azure Active Directory](https://azure.microsoft.com/pt-br/services/active-directory/)
- [GCP Identity Platform](https://cloud.google.com/identity-platform)
