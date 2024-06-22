# Team Service

Detalhamento sobre políticas de commits do serviço

## Commits

Os commits devem seguir o padrão de mensagem definido pelo [Conventional Commits](https://www.conventionalcommits.org/).

### Tipos

- **feat**: Nova funcionalidade
- **fix**: Correção de bug
- **docs**: Documentação
- **style**: Formatação
- **refactor**: Refatoração
- **test**: Testes
- **chore**: Tarefas de build

### Mensagem

- Os commits devem estar no imperativo, com a primeira letra maiúscula e sem ponto final.
- A mensagem deve ser clara e objetiva, descrevendo o que foi feito.
- A mensagem deve ter no máximo 100 caracteres.
- A mensagem deve ser escrita em português após o tipo.

### Exemplo

```bash
git commit -m "feat: Adiciona nova funcionalidade"
```

### Versionamento

O versionamento do serviço deve seguir o padrão [SemVer](https://semver.org/).
