![Image text](https://github.com/cloudmex/burrito-battle/blob/new_standard/assets/Logotipo.png)

## Descripción 📄

Burrito Battle es un videojuego hecho en Rust y basado en el Protocolo de NEAR, el cual permite adquirir burritos (NFTs) y equiparlos con diferentes accesorios intercambiables que les permite aumentar sus habilidades en batalla.

Elige uno de tus burritos, equípalo con hasta 3 accesorios y enfréntalo contra los de tus rivales.

### Dinámica de batalla

Las batallas consisten en que cada jugador selecciona uno de sus burritos y cada uno selecciona hasta 3 accesorios con los cuales combatir al burrito rival, cada burrito cuenta con una cantidad de vidas, y solo podrán ser usados en una pelea aquellos burritos que tengan por lo menos 1 vida.

### Determinar ganador de una batalla

Los combates serán por turnos, para determinar quien será el jugador en atacar primero se tomará en cuenta su estadística de velocidad, el accesorio equipado y un número generado aleatoriamente en un rango de 0.1 y 1.0, se toma con la formula (velocidad + accesorios) * número aleatorio. El burrito con el resultado mayor será el primero en atacar.

Una vez definida la prioridad se comienza con los ataques, el burrito con la prioridad de ataque hace el primer movimiento tomando en cuenta su estadística de ataque, su accesorio, su tipo y un número generado aleatoriamente, la fórmula es (ataque+accesorios)*número aleatorio, si el burrito atacante tiene ventaja por tipo a este resultado se le suma un 25% de su ataque final, este resultado son los puntos con que se realizara el ataque restándolos a los puntos de salud del burrito rival, los cuales serán el total de la suma de todas sus estadísticas base del burrito que está siendo atacado, a continuación se evalúa si los puntos de salud del burrito atacado es menor de 0, en este caso el ganador de la pelea es el burrito atacante, en caso contrario el burrito atacado ahora pasa a ser el atacante tomando en cuenta todo lo antes mencionado, los burritos solo pueden atacar 1 vez por turno, teniendo la capacidad de realizar un ataque normal y un ataque pesado (máximo 3 por batalla) y el burrito defensor tendrá la capacidad de utilizar un escudo para defenderse (máximo 3 escudos por batalla) la pelea puede tener n turnos hasta que alguno de los burritos tenga sus puntos de salud sean menor que 0, cuando esto pasa el burrito atacante gana la pelea además de incrementar su contador de victorias en 1 (Este contador será utilizado para incrementar el nivel y estadísticas bases del burrito en algún momento) y el burrito perdedor pierde una vida.

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

Ejecute el siguiente comando dentro de cada carpeta (Burrito, Items y STRW-Tokens) el cual generará nuestro archivo WASM en el directorio correspondiente (contracts/burrito/ , contracts/items/ y contracts/strw-token/ ). Además de que la consola preguntará si deseamos desplegar el contrato correspondiente.
    
    ./build.sh

## Métodos de los contratos 🚀

Asignamos el identificador de nuestro contrato desplegado a una constante (Sustituir el ID por el del contrato desplegado):

    Burrito
    ID=dev-1649707732162-66282708367055
    echo $ID

    Accesorios
    ID=dev-1647986467816-61735125036881
    echo $ID

    STRW-TOKEN
    ID=dev-1648843322449-70578827831792
    echo $ID

Los 3 contratos deben inicializarse antes de su uso, por lo que lo haremos con los siguientes comandos dependiendo del contrato:

    Burrito
    near call $ID init_contract '{"owner_id":"'$ID'"}' --accountId $ID
    near view $ID nft_metadata

    Accesorios
    near call $ID init_contract '{"owner_id": "'$ID'"}' --accountId $ID

    STRW-TOKEN
    near call $ID init_contract '{"owner_id": "yairnava.testnet", "treasury_id": "yairnh.testnet", "strw_mint_cost": 600000, "strw_reset_cost": 30000, "strw_evolve_cost": 100000}' --accountId $ID

Modificar icono

    near call $ID update_metadata_icon '{"icon": "data:image/jpeg;base64,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"}' --accountId $ID

### Burritos

Obtener cantidad de burritos creados:

    near view $ID get_number_burritos
    
Crear nuevo burrito:

near call $ID nft_mint '{"token_owner_id": "'yairnava.testnet'", "token_metadata": { "title": "", "description": "", "media": "", "extra":""}}' --accountId yairnava.testnet --deposit 5 --gas=300000000000000
    
Modificar burrito:

    near call $ID update_burrito '{"burrito_id": "0", "extra":"{'"'burrito_type'":"'Fuego'","'hp'":"'3'","'attack'":"'7'","'defense'":"'7'","'speed'":"'7'","'level'":"'1'","'win'":"'10'","'global_win'":"'10'"}'"}' --accountId yairnava.testnet 

Evolucionar burrito:

    near call $ID evolve_burrito '{"burrito_id": "0"}' --accountId yairnava.testnet --deposit 2 --gas=300000000000000

Restaurar burrito:

    near call $ID reset_burrito '{"burrito_id": "0"}' --accountId yairnava.testnet --deposit 1 --gas=300000000000000

Obtener datos de un burrito:

    near view $ID get_burrito '{"burrito_id": "0"}'

    near view $ID nft_token '{"token_id": "0"}'

Obtener datos de burritos de un segmento

    near call $ID nft_tokens '{"from_index": "0", "limit": 50}' --accountId yairnava.testnet

Obtener datos de burritos de un usuario por segmento

        near call $ID nft_tokens_for_owner '{"account_id": "yairnava.testnet", "from_index": "0", "limit": 50}' --accountId yairnava.testnet

Obtener cantidad de batallas finalizadas:

    near view $ID get_number_battles

Obtener cantidad de batallas activas Player vs CPU:

    near view $ID get_number_battles_actives_cpu

Obtener la sala activa del jugador Player vs CPU

    near call $ID get_battle_active_cpu '{}' --accountId yairnava.testnet

Crear una partida Jugador vs CPU:

    near call $ID create_battle_player_cpu '{"burrito_id":"'0'", "accesorio1_id":"'0'", "accesorio2_id":"'1'", "accesorio3_id":"'2'"}' --accountId yairnava.testnet --gas=300000000000000

Rendirse y finalizar combate activo Player vs CPU

    near call $ID surrender_cpu '{}' --accountId yairnava.testnet

Combatir Ronda Player vs CPU [type_move => (1 = Ataque Debil, 2 = Ataque Fuerte, 3 = No Defenderse, 4 = Defenderse)]
    
    near call $ID battle_player_cpu '{"type_move":"'1'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_cpu '{"type_move":"'2'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_cpu '{"type_move":"'3'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_cpu '{"type_move":"'4'"}' --accountId yairnava.testnet --gas=300000000000000

Obtener cantidad de batallas activas PvP:

    near view $ID get_number_battles_actives_pvp

Obtener la sala activa del jugador PvP

    near call $ID get_battle_active_pvp '{}' --accountId yairnava.testnet

Borrar todas las salas activas PvP

    near call $ID delete_battle_active_pvp '{}' --accountId yairnava.testnet

Crear una partida Jugador vs CPU:

    near call $ID create_battle_player_pvp '{"burrito_id":"'0'", "accesorio1_id":"'0'", "accesorio2_id":"'0'", "accesorio3_id":"'0'"}' --accountId yairnh.testnet --gas=300000000000000

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
* [Rust Toolchain](https://docs.near.org/docs/develop/contracts/rust/intro#installing-the-rust-toolchain)
* [NEAR CLI](https://docs.near.org/docs/tools/near-cli) - Herramienta de interfaz de línea de comandos para interactuar con cuentas y contratos inteligentes en NEAR.
* [yarn](https://classic.yarnpkg.com/en/docs/install#mac-stable)