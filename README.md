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

Para um detalhamento maior sobre a arquitetura acesse este documento: [Arquitetura](_docs/architecture.md)

## Microserviços

Cada microservice tem sua própria documentação, acesse para mais detalhes:

- [Cost Center Service](cost_center_service/README.md)
- [Client Contractor Service](client_contractor_service/README.md)
- [Notifications Service](notifications_service/README.md)
- [Identity Provider Service](identity_provider_service/README.md)
- [Team Service](team_service/README.md)

### Estudos em aberto

Para entender quais estudos estão em aberto acesse este documento: [Estudos em aberto](_docs/open_studies.md)

