# discord-ws-http-bridge
WS/HTTP bridge to the Discord API

- [x] Utilizzare la ggsbotrs-gateway-queue al posto di fare una POST direttamente al worker del bot
- [ ] Utilizzare la queue solo nel caso in cui si ottenga una response di tipo 429 dal Worker in modo da rendere il worker come endpoint predefinito e quando invece arrivano tante richieste passare alla queue