# Team Service

Detalhamento da comunicação do serviço com o gateway

## Gateway

O gateway é responsável por intermediar a comunicação entre o serviço e o cliente.
Ele é responsável por traduzir as requisições HTTP em chamadas gRPC e vice-versa.

### Estrutura

O gateway é composto por diversas camadas, cada uma com sua responsabilidade.

- **grpc**: Protocolo de comunicação entre o gateway e o serviço.
- **http**: Protocolo de comunicação entre o gateway e o cliente.

### Políticas

O gateway segue as seguintes políticas:

- **CORS**: Cross-Origin Resource Sharing.
- **JWT**: Autenticação e autorização.
- **Rate Limit**: Limite de requisições.
- **Logging**: Manipulação de logs.
- **Retry**: Retentativas de chamadas.
- **Circuit Breaker**: Quebra de circuito.
- **Timeout**: Tempo limite de chamada.
