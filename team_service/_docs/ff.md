# Team Service

Detalhamento das features flags do serviço.

## Features Flags

Escopo de FFs:

- `global`: FFs que afetam todo o serviço
- `local`: FFs que afetam apenas um módulo do serviço
- `specific`: FFs que afetam apenas uma funcionalidade do serviço

Lançamento de FFs:

- `gradual`: FFs que são lançadas de forma gradual para um percentual de clientes
- `test`: FFs que são lançadas para teste de funcionalidade
- `temp`: FFs que são lançadas de forma temporária com data de desativação
- `permanent`: FFs que são lançadas de forma permanente para todos os clientes
- `experiment`: FFs que são lançadas para experimento de funcionalidade

Tipos de FFs:

- `security`: FFs que são lançadas para garantir a segurança do serviço
- `operation`: FFs que são lançadas para garantir a operação do serviço
- `business`: FFs que são lançadas para garantir a funcionalidade de negócio do serviço
- `infra`: FFs que são lançadas para garantir a infraestrutura do serviço

Conjunto de FFs do serviço team_service na versão 1.0.0

Em desenvolvimento...

Exemplo de tabela de FFs:

| Nome | Descrição | Valor Padrão | Tipo | Escopo | Lançamento |
|------|-----------|--------------|------|--------|------------|
| `ff1` | Descrição da FF1 | `false` | `security` | `global` | `gradual` |
