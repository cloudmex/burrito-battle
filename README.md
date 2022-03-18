# Burrito Battle

## Descripción 📄

Burrito Battle es un videojuego hecho en Rust y basado en el Protocolo de NEAR, el cual permite adquirir burritos (NFTs) y equiparlos con diferentes accesorios intercambiables que les permite aumentar sus habilidades en batalla.

Elige hasta 3 de tus mejores burritos, equípalos con hasta 3 accesorios y enfréntalos contra los de tus rivales.

## Diapositivas de proyecto 🖥️

    https://docs.google.com/presentation/d/1TZpxOEumc4svX0-PUf6RAsr-8uzvaWEWvXAaXoHIK9M/edit#slide=id.p

### Dinámica de batalla

Las batallas se conforman de máximo 5 rondas o peleas, cada jugador selecciona hasta 3 de sus burritos para la batalla, en cada ronda o pelea el jugador selecciona 1 burrito y hasta 3 accesorios con los cuales combatir al burrito rival, cada burrito cuenta con una cantidad de vidas, y solo podrán ser usados en una pelea aquellos burritos que tengan por lo menos 1 vida.

### Determinar ganador de una batalla

Los combates serán por turnos, para cada turno se define que burrito será el primero en atacar tomando en cuenta su estadística de velocidad, el accesorio equipado y un número generado aleatoriamente en un rango de 0.1 y 1.0, se toma con la formula (velocidad + accesorios) *número aleatorio. El burrito con el resultado mayor será el primero en atacar.

Una vez definida la prioridad se comienza con los ataques, el burrito con la prioridad de ataque hace el primer movimiento tomando en cuenta su estadística de ataque, su accesorio, su tipo y un número generado aleatoriamente, la fórmula es (ataque+accesorios)*número aleatorio, si el burrito atacante tiene ventaja por tipo a este resultado se le suma un 25% de su ataque final, este resultado son los puntos con que se realizara el ataque restándolos a los puntos de defensa del burrito que está siendo atacado, a continuación se evalúa si la defensa del burrito atacado es menor de 0, en este caso el ganador de la pelea es el burrito atacante, en caso contrario el burrito atacado ahora pasa a ser el atacante tomando en cuenta todo lo antes mencionado, los burritos solo pueden atacar 1 vez por turno, la pelea puede tener n turnos hasta que alguno de los burritos tenga su defensa menor que 0, cuando esto pasa el burrito atacante gana la pelea además de incrementar su contador de victorias en 1 (Este contador será utilizado para incrementar el nivel y estadísticas bases del burrito en algún momento) y el burrito perdedor pierde una vida.

La batalla continua con la siguiente pelea donde se repite todo el proceso anterior, el ganador de la batalla es el que logre ganar 3 de 5 peleas.

### Típos de burritos
| VS | Fuego🔥| Agua💧 | Planta🌱 | Eléctrico⚡ | Volador💨 |
| --- | --- | --- | --- | --- | --- |
| Fuego🔥 | 0% | +25%💧 | +25%🔥 | 0% | 0% |
| Agua💧 | +25%💧 | 0% | 0% | 0% | +25%💨 |
| Planta🌱 | +25%🔥 | 0% | 0% | +25%🌱 | 0% |
| Eléctrico⚡ | 0% | 0% | +25%🌱 | 0% | +25%⚡ |
| Volador💨  | 0% | +25%💨 | 0% | +25%⚡ | 0% |

## Instalación 🔧 

Para ejecutar este proyecto localmente, debe seguir los siguientes pasos:

Paso 1: requisitos previos

1. Asegúrese de haber instalado [Node.js] ≥ 12 (recomendamos usar [nvm])
2. Asegúrese de haber instalado yarn: `npm install -g yarn`
3. Instalar dependencias: `yarn install`
4. Cree una cuenta de prueba de NEAR
5. Instale NEAR CLI globalmente: [near-cli] es una interfaz de línea de comandos (CLI) para interactuar con NEAR blockchain.

Paso 2: Configure su NEAR CLI

Configure su near-cli para autorizar su cuenta de prueba creada recientemente:

    near login
         
    
## Despliegue 📦

Ejecute el siguiente comando dentro de cada carpeta (Burrito, Items y STRW-Tokens) el cual generará nuestro archivo WASM en el directorio correspondiente (contracts/burrito/ , contracts/items/ y contracts/strw-token/ ). Además de que la consola preguntará si deseamos desplegar el contrato.
    
    ./build.sh

Desplegar y Migrar Contratos:

    near deploy --wasmFile wasmFile.wasm --initFunction "migrate" --initArgs "{}" --accountId $ID

## Métodos de los contratos 🚀

Asignamos el identificador de nuestro contrato desplegado a una constante (Sustituir el ID por el del contrato desplegado):

    Burrito
    ID=dev-1647245153470-55139133456484
    echo $ID

    Accesorios
    ID=dev-1645212248150-33385648447581
    echo $ID

    STRW-TOKEN
    ID=dev-1645837411235-48460272126519
    echo $ID

Los 3 contratos deben inicializarse antes de su uso, por lo que lo haremos con los siguientes comandos dependiendo del contrato:

    Burrito
    near call $ID init_contract '{"owner_id":"'$ID'"}' --accountId $ID

    Accesorios
    near call $ID init_contract '{"owner_id": "'$ID'"}' --accountId $ID

    STRW-TOKEN
    near call $ID init_contract '{"owner_id": "yairnava.testnet", "treasury_id": "yairnh.testnet", "strw_mint_cost": 600000, "strw_reset_cost": 30000, "strw_evolve_cost": 100000}' --accountId $ID

### Burritos

Obtener cantidad de burritos creados:

    near view $ID get_number_burritos
    
Crear nuevo burrito:

    near call $ID mint_token '{"token_owner_id": "'yairnava.testnet'", "colecction": "Burritos BB", "token_metadata": { "title": "Burrito Name", "description": "This is a burrito", "media": "", "extra":""}}' --accountId yairnava.testnet --deposit 5 --gas=300000000000000
    
Modificar burrito:

    near call $ID update_burrito '{"burrito_id": "0", "extra":"{'"'burrito_type'":"'Fuego'","'hp'":"'5'","'attack'":"'9'","'defense'":"'5'","'speed'":"'7'","'level'":"'1'","'win'":"'10'","'global_win'":"'10'"}'"}' --accountId yairnava.testnet 

Evolucionar burrito:

    near call $ID evolve_burrito '{"burrito_id": "0"}' --accountId yairnava.testnet --deposit 2 --gas=300000000000000

Restaurar burrito:

    near call $ID reset_burrito '{"burrito_id": "0"}' --accountId yairnava.testnet --deposit 1 --gas=300000000000000

Obtener datos de un burrito:

    near view $ID get_burrito '{"burrito_id": "0"}'

    near view $ID nft_token '{"token_id":"0"}' --accountId yairnava.testnet

Crear una partida Jugador vs CPU:

    near call $ID create_battle_player_cpu '{"burrito_id":"'0'", "accesorio1_id":"'0'", "accesorio2_id":"'1'", "accesorio3_id":"'2'"}' --accountId yairnava.testnet

Obtener cantidad de batallas finalizadas:

    near view $ID get_number_battles

Mostrar todo el historial de batallas finalizadas del jugador:

    near call $ID get_battle_rooms_history --accountId yairnava.testnet 

Obtener cantidad de batallas activas:

    near view $ID get_number_battles_actives_cpu

Obtener la sala activa del jugador

    near call $ID get_battle_active_cpu '{}' --accountId yairnava.testnet

Rendirse y finalizar combate activo

    near call $ID surrender_cpu '{}' --accountId yairnava.testnet

Combatir Ronda Player vs CPU [type_move => (1 = Ataque Debil, 2 = Ataque Fuerte, 3 = No Defenderse, 4 = Defenderse)]
    
    near call $ID battle_player_cpu '{"type_move":"'1'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_cpu '{"type_move":"'2'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_cpu '{"type_move":"'3'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_cpu '{"type_move":"'4'"}' --accountId yairnava.testnet --gas=300000000000000

### Items

Obtener cantidad de accesorios creados:

    near view $ID get_number_accessories

Crear nuevo accesorio:

    near call $ID mint_token '{"token_owner_id": "'yairnava.testnet'", "colecction": "Items BB", "token_metadata": { "title": "Thunder Sword", "description": "Thunder Sword 2", "media": "","extra":"{'"'attack'":"'3'","'defense'":"'0'","'speed'":"'0'"}'"}}' --accountId yairnava.testnet --deposit 0.1 --gas=300000000000000

Obtener datos de un accesorio:

    near view $ID get_accessory '{"accessory_id": "0"}'
    
    near view $ID nft_token '{"token_id":"0"}' --accountId yairnava.testnet

### STRW-Tokens

Obtener propietario del contrato STRW-Token
    
    near view $ID get_owner_id

Cambiar propietario del contrato STRW-Token

    near call $ID set_owner_id '{"owner_id": "yairnh.testnet"}' --accountId yairnava.testnet

Obtener lista de mineros STRW-Token
    
    near view $ID get_minters

Agregar minero STRW-Token

    near call $ID add_minter '{"account_id": "yairnh.testnet"}' --accountId yairnava.testnet --deposit 0.000000000000000000000001

Remover minero STRW-Token

    near call $ID remove_minter '{"account_id": "bbtoken.testnet"}' --accountId yairnava.testnet --deposit 0.000000000000000000000001

Minar STRW-Token

    near call $ID mint '{"account_id": "yairnava.testnet", "amount" : "1000000000000000000000000000000"}' --accountId yairnava.testnet --deposit 0.000000000000000000000001

Obtener balance total de STRW-Token
    
    near view $ID ft_total_supply

Obtener balance de una cuenta de STRW-Token

    near view $ID ft_balance_of '{"account_id": "yairnava.testnet"}'

Transferir STRW-Token a una cuenta

    near call $ID ft_transfer '{"receiver_id": "yairnh.testnet", "amount" : "1000000000000000000000000000"}' --accountId yairnava.testnet --deposit 0.000000000000000000000001

Mostrar STRW-Token en Wallet

    near call $ID ft_transfer '{"receiver_id": "yairnava.testnet", "amount":"0", "memo":""}' --accountId yairnava.testnet --deposit 0.000000000000000000000001

Minar tokens y agregarlos al wallet

    near call $ID reward_player '{"player_owner_id": "yairnava.testnet", "tokens_mint" : "1000000000000000000000000000000"}' --accountId $ID --deposit 0.000000000000000000000001

## Construido con 🛠️

* [RUST](https://www.rust-lang.org/) - Lenguaje de programación usado para contrato inteligente.
* [NEAR CLI](https://docs.near.org/docs/tools/near-cli) - Herramienta de interfaz de línea de comandos para interactuar con cuentas y contratos inteligentes en NEAR.