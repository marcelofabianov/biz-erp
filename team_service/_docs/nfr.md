# Team Service

Relação de requisitos não funcionais do serviço de gerenciamento de equipes, cargos, departamentos e pessoas.

## Requisitos não funcionais

Conjunto de NFRs do serviço team_service na versão 1.0.0

### Tecnologias

- [Rust](https://www.rust-lang.org/)
- [Postgres](https://www.postgresql.org/)
- [Kafka](https://kafka.apache.org/)

### Dependências

- [SQLx](https://crates.io/crates/sqlx)
- [Kafka](https://crates.io/crates/kafka)
- [Tokio](https://crates.io/crates/tokio)
- [Chrono](https://crates.io/crates/chrono)
- [dotenv](https://crates.io/crates/dotenv)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)
- [uuid](https://crates.io/crates/uuid)

### Ambiente

- [Docker](https://www.docker.com/)
- [Git](https://git-scm.com/)

### Requisitos

- Este serviço deve ser capaz de escalar horizontalmente
- Este serviço deve ser capaz de se comunicar com outros serviços
- Este serviço deve ter seu banco de dados relacional
- Este serviço deve ter camada de comunicação GRPC
- Este serviço deve emitir eventos para um broker de mensagens
- Este serviço deve ouvir eventos de um broker de mensagens
- Este serviço deve ter logs estruturados conforme padrão da especificação
- Este serviço deve ter métricas de saúde e performance
- Este serviço deve ter testes unitários
- Este serviço deve ter testes de integração
- Este serviço deve ter testes de contrato
- Este serviço deve ter testes de carga
- Este serviço deve ter testes de segurança

### Restrições

- Este serviço deve ser desenvolvido em Rust
- Este serviço deve ser executado em ambiente Docker
- Este serviço deve ser versionado conforme semântica de versionamento
- Este serviço deve ser documentado conforme padrão OpenAPI
- Este serviço deve ser publicado em um repositório Git
- Este serviço deve ser CI/CD conforme pipeline definida
- Este serviço deve ser monitorado conforme padrão de monitoramento
- Este serviço deve ser orquestrado conforme padrão de orquestração

### Atributos de qualidade

- Disponibilidade
- Confiabilidade
- Escalabilidade
- Desempenho
- Segurança
- Manutenibilidade
- Testabilidade
- Usabilidade
- Interoperabilidade
- Portabilidade
- Eficiência
- Flexibilidade
- Conformidade
- Rastreabilidade
- Auditabilidade
