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

**Importante**:

- Caso o commit não se encaixe em nenhum dos tipos acima, utilize o tipo `chore`.
- Caso o commit seja de algo complexa crie mensagem mais detalhada com `git commit -a`
- Caso o commit seja de algo feature simples, crie mensagem curta com `git commit -m`

### Exemplo

```bash
git commit -m "feat: Adiciona nova funcionalidade"
```

### Branches

- **master**: Branch principal, contém a versão de produção.
- **develop**: Branch de desenvolvimento, contém a versão de homologação.
- **feature**: Branch de desenvolvimento de novas funcionalidades.
- **hotfix**: Branch de correção de bugs em produção.

### Merge

- O merge deve ser feito via pull request.
- O pull request deve ser revisado por outro membro do time.
- O pull request deve passar por testes automatizados.

### Tags

- As tags devem ser criadas a partir da branch master.
- As tags devem seguir o padrão `vX.Y.Z`, onde `X` é a versão principal, `Y` é a versão secundária e `Z` é a versão de correção.

### Versionamento

O versionamento do serviço deve seguir o padrão [SemVer](https://semver.org/).
