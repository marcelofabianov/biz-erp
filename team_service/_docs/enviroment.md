# Team Service

Detalhamento das variáveis de ambiente do serviço

## Variáveis de ambiente

Conjunto de variáveis de ambiente que entraram na versão 1.0.0

- `DATABASE_URL`: URL de conexão com o banco de dados
- `KAFKA_BROKER`: URL do broker de mensagens
- `KAFKA_GROUP_ID`: ID do grupo de consumidores
- `KAFKA_TOPIC`: Tópico de mensagens
- `LOG_LEVEL`: Nível de log
- `LOG_FORMAT`: Formato de log
- `LOG_OUTPUT`: Saída de log
- `METRICS_PORT`: Porta de métricas
- `METRICS_PATH`: Caminho de métricas
- `HEALTH_PORT`: Porta de saúde
- `HEALTH_CHECK`: Checagem de saúde
- `HEALTH_TIMEOUT`: Tempo de timeout de saúde

## Exemplo

```shell
DATABASE_URL=postgres://user:password@localhost:5432/database
KAFKA_BROKER=localhost:9092
KAFKA_GROUP_ID=team_service
KAFKA_TOPIC=team_service
LOG_LEVEL=info
LOG_FORMAT=json
LOG_OUTPUT=stdout
METRICS_PORT=8080
METRICS_PATH=/metrics
HEALTH_PORT=8081
HEALTH_CHECK=/health
HEALTH_TIMEOUT=5
```
