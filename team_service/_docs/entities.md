# Team Service

Relação de entidades do serviço de gerenciamento de equipes, cargos, departamentos e pessoas.

## Entities

Account (Conta)

- **ownership_id**: `account` | Propriedade dos dados LGPD
- **trace_id**: `uuid` `required` | ID do trace
- **id**: `pk` `int` `unique` `required` | ID da conta
- **public_id**: `index` `uuid` `unique` `required` | Public ID da conta
- **name**: `string` `required` | Nome da conta
- **document_registry**: `string` `required` | Registro do documento da conta
- **disabled_at**: `datetime` `optional` | Data de desativação da conta
- **deleted_at**: `datetime` `optional` | Data de exclusão da conta

Department (Departamento)

- **ownership_id**: `account` | Propriedade dos dados LGPD
- **trace_id**: `uuid` `required` | ID do trace
- **id**: `pk` `int` `unique` `required` | ID do departamento
- **public_id**: `index` `uuid` `unique` `required` | Public ID do departamento
- **account_id**: `fk` `int` `required` | ID da conta
- **name**: `string` `required` | Nome do departamento
- **description**: `string` `optional` | Descrição do departamento
- **disabled_at**: `datetime` `optional` | Data de desativação do departamento
- **created_at**: `datetime` `required` | Data de criação do departamento
- **updated_at**: `datetime` `required` | Data de atualização do departamento
- **deleted_at**: `datetime` `optional` | Data de exclusão do departamento

Position (Cargo)

- **ownership_id**: `account` | Propriedade dos dados LGPD
- **trace_id**: `uuid` `required` | ID do trace
- **id**: `pk` `int` `unique` `required` | ID do cargo
- **public_id**: `index` `uuid` `unique` `required` | Public ID do cargo
- **department_id**: `fk` `int` `required` | ID do departamento
- **name**: `string` `required` | Nome do cargo
- **description**: `string` `optional` | Descrição do cargo
- **disabled_at**: `datetime` `optional` | Data de desativação do cargo
- **created_at**: `datetime` `required` | Data de criação do cargo
- **updated_at**: `datetime` `required` | Data de atualização do cargo
- **deleted_at**: `datetime` `optional` | Data de exclusão do cargo

Role (Função)

- **ownership_id**: `account` | Propriedade dos dados LGPD
- **trace_id**: `uuid` `required` | ID do trace
- **id**: `pk` `int` `unique` `required` | ID da função
- **public_id**: `index` `uuid` `unique` `required` | Public ID da função
- **name**: `string` `required` | Nome da função
- **description**: `string` `optional` | Descrição da função
- **disabled_at**: `datetime` `optional` | Data de desativação da função
- **created_at**: `datetime` `required` | Data de criação da função
- **updated_at**: `datetime` `required` | Data de atualização da função
- **deleted_at**: `datetime` `optional` | Data de exclusão da função

Team (Equipe)

- **ownership_id**: `account` | Propriedade dos dados LGPD
- **trace_id**: `uuid` `required` | ID do trace
- **id**: `pk` `int` `unique` `required` | ID da equipe
- **public_id**: `index` `uuid` `unique` `required` | Public ID da equipe
- **account_id**: `fk` `int` `required` | ID da conta
- **name**: `string` `required` | Nome da equipe
- **description**: `string` `optional` | Descrição da equipe
- **disabled_at**: `datetime` `optional` | Data de desativação da equipe
- **created_at**: `datetime` `required` | Data de criação da equipe
- **updated_at**: `datetime` `required` | Data de atualização da equipe
- **deleted_at**: `datetime` `optional` | Data de exclusão da equipe

Person (Pessoa)

- **ownership_id**: `account` | Propriedade dos dados LGPD
- **trace_id**: `uuid` `required` | ID do trace
- **id**: `pk` `int` `unique` `required` | ID da pessoa
- **public_id**: `index` `uuid` `unique` `required` | Public ID da pessoa
- **account_id**: `fk` `int` `required` | ID da conta
- **team_id**: `fk` `int` `optional` | ID da equipe
- **role_id**: `fk` `int` `optional` | ID da função
- **position_id**: `fk` `int` `optional` | ID do cargo
- **department_id**: `fk` `int` `optional` | ID do departamento
- **name**: `string` `required` | Nome da pessoa
- **email**: `string` `required` | E-mail da pessoa
- **phone**: `string` `optional` | Telefone da pessoa
- **mobile_phone**: `string` `optional` | Celular da pessoa
- **document_registry**: `string` `required` | Registro do documento da pessoa
- **disabled_at**: `datetime` `optional` | Data de desativação da pessoa
- **created_at**: `datetime` `required` | Data de criação da pessoa
- **updated_at**: `datetime` `required` | Data de atualização da pessoa
- **deleted_at**: `datetime` `optional` | Data de exclusão da pessoa

## Relationships

- Conta (Account) tem muitos Departamentos (Department)
- Conta (Account) tem muitos Cargos (Position)
- Conta (Account) tem muitas Funções (Role)
- Conta (Account) tem muitas Equipes (Team)
- Conta (Account) tem muitas Pessoas (Person)
- Departamento (Department) tem muitos Cargos (Position)
- Departamento (Department) tem muitas Pessoas (Person)
- Cargo (Position) tem muitas Pessoas (Person)
- Função (Role) tem muitas Pessoas (Person)
- Equipe (Team) tem muitas Pessoas (Person)
- Pessoa (Person) pertence a uma Conta (Account)
- Pessoa (Person) pertence a uma Equipe (Team)
- Pessoa (Person) pertence a uma Função (Role)
- Pessoa (Person) pertence a um Cargo (Position)
- Pessoa (Person) pertence a um Departamento (Department)

## Conceitos de atributos específicos

- **ownership_id: O ownership_id é um atributo que indica a propriedade dos dados de acordo com a LGPD (Lei Geral de Proteção de Dados). Ele é utilizado para identificar a entidade responsável pela gestão e proteção dos dados, garantindo a conformidade com a legislação de proteção de dados pessoais.

- **trace_id: O trace_id é um identificador único e global utilizado para rastrear o fluxo de uma transação ou operação através de múltiplos serviços em uma arquitetura distribuída. Ele permite correlacionar eventos e logs relacionados a uma mesma solicitação ou transação, facilitando o diagnóstico de problemas, a análise de desempenho e o monitoramento de sistemas distribuídos.
