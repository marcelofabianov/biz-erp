# BizRetail ERP

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

## Instalação

...
