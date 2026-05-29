
# GGsBot.rs 

<img src="./resources/graph.svg" />

## TODOs
- [ ] Creare delle UI per la configurazione dei sottocomandi
> Esempio:
> /ext setup nasa:apod
>
> Qui compare una UI apposita per configurare nasa:apod 
>
> (NOTA: A questo punto non e' piu' possibile fare /ext setup nasa)

- [x] Rendere il comando /nasa apod accessibile anche nelle chat private e (in quel caso) rimuovere ephemeral
- [x] Modificare i comando /ext setup, /ext teardown, ... in modo che mostrino solo i comandi configurabili

- [x] Re-implementare la gestione delle queue in modo che sia piu' dinamica
    // TODO: Modificare il dispatcher delle queue in modo che QueueMessage contenga un campo message_type di tipo String
    // Aggiungere inoltre oltre al metodo name() della queue nel trait Queue anche un metodo message_type
    // In modo che si possa creare un handler per ogni singolo tipo di dato...

## Credits

based on [stateless-discord-bot](https://github.com/siketyan/stateless-discord-bot)