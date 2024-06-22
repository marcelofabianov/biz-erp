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

Este serviço deve ser...

- Versionado conforme semântica de versionamento
- Documentado e mantido
- Mantido pelo versonador GIT
- Mantido pelo CI/CD conforme pipeline definida
- Monitorado conforme padrão de monitoramento

Deve ter...

- Banco de dados Postgres
- Conexão com Kafka
- Testes unitários
- Testes de integração
- Testes de contrato
- Testes de carga
- Testes de segurança
- Logs estruturados e centralizados

Não deve ter...

- Dependências desnecessárias
- Código duplicado
- Código comentado
- Código inseguro
- Código não testado
- Código não documentado

### Restrições

- Linguagem Rust
- Banco de dados Postgres
- Kafka como broker de mensagens
- Docker como ambiente de execução
- Git como versionador
- CI/CD com pipeline definida GITHUB ACTIONS
- Monitoramento com Elastic Stack

### Atributos de qualidade

Atributos de qualidade que buscamos:

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
