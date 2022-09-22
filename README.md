![Image text](https://github.com/cloudmex/burrito-battle/blob/master/assets/Logotipo.png)

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

Ejecute el siguiente comando dentro de cada carpeta (Burrito, Items y STRW-Tokens) el cual generará nuestro archivo WASM en el directorio correspondiente (contracts/burrito/ , contracts/items/ , contracts/playervscpu/ y contracts/strw-token/ ). Además de que la consola preguntará si deseamos desplegar el contrato correspondiente.
    
    ./build.sh

## Métodos de los contratos 🚀

Asignamos el identificador de nuestro contrato desplegado a una constante (Sustituir el ID por el del contrato desplegado):

    Burrito
    ID=bb-burritos.testnet
    echo $ID

    Accesorios
    ID=bb-items.testnet
    echo $ID

    STRW-TOKEN
    ID=bb-strw.testnet
    echo $ID

    PVE Battle
    ID=bb-pve.testnet
    echo $ID

    Posiones
    ID=bb-potions.testnet
    echo $ID

Los 4 contratos deben inicializarse antes de su uso, por lo que lo haremos con los siguientes comandos dependiendo del contrato:

    Burrito
    near call $ID init_contract '{"owner_id":"'$ID'","burrito_contract":"'$ID'","items_contract":"'bb-items.testnet'","strw_contract":"'bb-strw.testnet'"}' --accountId dev-1663710126378-66907359558484

    Accesorios
    near call $ID init_contract '{"owner_id": "'$ID'"}' --accountId $ID

    STRW-TOKEN
    near call $ID init_contract '{"owner_id": "'$ID'", "treasury_id": "bb-treasury.testnet", "strw_mint_cost": 50000, "strw_reset_cost": 30000, "strw_evolve_cost": 70000}' --accountId $ID

    PVE Battle
    near call $ID init_contract '{"owner_id":"'$ID'", "burrito_contract":"'dev-1663710126378-66907359558484'","items_contract":"'bb-items.testnet'","strw_contract":"'bb-strw.testnet'", "pve_contract":"'dev-1663738468242-76777974943846'"}' --accountId $ID

    Potions
    near call $ID init_contract '{"owner_id":"'$ID'"}' --accountId $ID

### Burritos

Cambiar de owner

    near call $ID change_owner '{"owner_id": "bb-burrito-battle.sputnikv2.testnet"}' --accountId $ID

Cambiar contratos

    near call $ID change_contracts '{"burrito_contract":"'dev-1663710126378-66907359558484'","items_contract":"'bb-items.testnet'","strw_contract":"'bb-strw.testnet'"}' --accountId $ID

Mostrar contratos

    near view $ID show_contracts

Obtener cantidad de burritos creados:

    near view $ID get_number_burritos
    
Crear nuevo burrito:

near call $ID nft_mint '{"token_owner_id": "'yairnava.testnet'", "token_metadata": { "title": "", "description": "", "media": "", "extra":""}}' --accountId yairnava.testnet --deposit 5 --gas=300000000000000
    
Modificar burrito:

    near call $ID update_burrito '{"burrito_id": "5", "extra":"{'"'burrito_type'":"'Planta'","'hp'":"'0'","'attack'":"'15'","'defense'":"'15'","'speed'":"'15'","'level'":"'1'","'win'":"'10'","'global_win'":"'10'"}'"}' --accountId $ID

Evolucionar burrito:

    near call $ID evolve_burrito '{"burrito_id": "0"}' --accountId yairnava.testnet --deposit 2 --gas=300000000000000

Restaurar burrito:

    near call $ID reset_burrito '{"burrito_id": "4"}' --accountId yairnava.testnet --deposit 1 --gas=300000000000000

Obtener datos de un burrito:

    near call $ID get_burrito '{"burrito_id": "151"}' --accountId yairnava.testnet

    near view $ID nft_token '{"token_id": "5"}'

Obtener datos de burritos de un segmento

    near call $ID nft_tokens '{"from_index": "0", "limit": 50}' --accountId yairnava.testnet --gas=300000000000000

Obtener datos de burritos de un usuario por segmento

        near call $ID nft_tokens_for_owner '{"account_id": "yairnava.testnet", "from_index": "0", "limit": 50}' --accountId yairnava.testnet

Agregar contrato a Whitelist

    near call $ID add_whitelist '{"address_contract":"'bb-burritos.testnet'","contract_name":"'Burritos'"}' --accountId $ID

    near call $ID add_whitelist '{"address_contract":"'bb-pve.testnet'","contract_name":"'PVE'"}' --accountId $ID

    near call $ID add_whitelist '{"address_contract":"'bb-incursions.testnet'","contract_name":"'INCURSION'"}' --accountId $ID

    near call $ID add_whitelist '{"address_contract":"'bb-hospital.testnet'","contract_name":"'HOSPITAL'"}' --accountId $ID

Consultar si un contrato esta en Whitelist

    near call $ID is_white_listed  --accountId yairnava.testnet

### Items

Obtener cantidad de accesorios creados:

    near view $ID get_number_accessories

Crear nuevo accesorio:

    near call $ID mint_token '{"token_owner_id": "'yairnava.testnet'", "colecction": "Items BB", "token_metadata": { "title": "Thunder Sword", "description": "Thunder Sword 2", "media": "","extra":"{'"'attack'":"'3'","'defense'":"'0'","'speed'":"'0'"}'"}}' --accountId yairnava.testnet --deposit 0.1 --gas=300000000000000

Obtener datos de un accesorio:

    near view $ID get_accessory '{"accessory_id": "0"}'
    
    near view $ID nft_token '{"token_id":"0"}' --accountId yairnava.testnet

### STRW-Tokens

Obtener propietario del contrato
    
    near view $ID get_owner_id

Asignar datos del Straw Token al FT

    near call $ID set_meta '{}' --accountId bb-strw.testnet

Cambiar propietario del contrato

    near call $ID set_owner_id '{"owner_id": "yairnava.testnet"}' --accountId bb-strw.testnet

Obtener lista de mineros
    
    near view $ID get_minters

Obtener costos

    near view $ID get_costs

Actualizar costos

    near call $ID set_costs '{"strw_mint_cost": 50000, "strw_reset_cost": 30000, "strw_evolve_cost": 70000}' --accountId bb-strw.testnet

Cambiar tesorero

    near call $ID set_treasury '{"new_treasury": "yairnava.testnet"}' --accountId bb-strw.testnet

Agregar minero

    near call $ID add_minter '{"account_id": "bb-burritos.testnet"}' --accountId bb-strw.testnet

    near call $ID add_minter '{"account_id": "bb-pve.testnet"}' --accountId bb-strw.testnet
    
    near call $ID add_minter '{"account_id": "bb-incursions.testnet"}' --accountId bb-strw.testnet

    near call $ID add_minter '{"account_id": "bb-hospital.testnet"}' --accountId bb-strw.testnet

Remover minero

    near call $ID remove_minter '{"account_id": "yairnava.testnet"}' --accountId bb-strw.testnet

Minar STRW-Token

    near call $ID mint '{"account_id": "yairnava.testnet", "amount" : "100000000000000000000000000000"}' --accountId bb-strw.testnet

Mostrar STRW-Token en Wallet

    near call $ID ft_transfer '{"receiver_id": "yairnava.testnet", "amount":"0", "memo":""}' --accountId bb-strw.testnet

Obtener balance total de STRW-Token
    
    near view $ID ft_total_supply

Obtener balance de una cuenta de STRW-Token

    near view $ID ft_balance_of '{"account_id": "yairnava.testnet"}'


Verificar si una cuenta puede comprar tokens

    near view $ID can_buy_tokens '{"account_id": "yairnava.testnet"}'

Comprar STRW-Tokens 

    near call $ID buy_tokens '{}' --accountId yairnava.testnet --deposit 1

### Player vs CPU

Cambiar de owner

    near call $ID change_owner '{"owner_id": "bb-burrito-battle.sputnikv2.testnet"}' --accountId $ID

Cambiar contratos

    near call $ID change_contracts '{"burrito_contract":"'dev-1663710126378-66907359558484'","items_contract":"'bb-items.testnet'","strw_contract":"'bb-strw.testnet'" ,"pve_contract":"'dev-1663738468242-76777974943846'"}' --accountId $ID

Mostrar contratos

    near view $ID show_contracts

Obtener si una cuenta está en batalla:

    near view $ID is_in_battle '{"account_id": "yairnava.testnet"}'

Obtener cantidad de batallas finalizadas:

    near view $ID get_number_battles

Obtener cantidad de batallas activas Player vs CPU:

    near view $ID get_number_battles_actives

Obtener la sala activa del jugador Player vs CPU

    near call $ID get_battle_active '{}' --accountId yairnava.testnet

Crear una partida Jugador vs CPU:

    near call $ID create_battle_player_cpu '{"burrito_id":"'0'", "accesorio1_id":"'0'", "accesorio2_id":"'0'", "accesorio3_id":"'0'"}' --accountId yairnava.testnet --gas=300000000000000

Rendirse y finalizar combate activo Player vs CPU

    near call $ID surrender_cpu '{}' --accountId yairnava.testnet --gas=300000000000000

Combatir Ronda Player vs CPU [type_move => (1 = Ataque Debil, 2 = Ataque Fuerte, 3 = No Defenderse, 4 = Defenderse)]
    
    near call $ID battle_player_cpu '{"type_move":"'1'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_cpu '{"type_move":"'2'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_cpu '{"type_move":"'3'"}' --accountId yairnava.testnet --gas=300000000000000
    
    near call $ID battle_player_cpu '{"type_move":"'4'"}' --accountId yairnava.testnet --gas=300000000000000

### Crear Propuestas en DAO

Ejecutar Método:

    sputnikdao proposal call dev-1663710126378-66907359558484 change_contracts '{"burrito_contract":"'dev-1663710126378-66907359558484'","items_contract":"'bb-items.testnet'","strw_contract":"'bb-strw.testnet'"}' --daoAcc bb-burrito-battle --accountId yairnava.testnet

    sputnikdao proposal call dev-1663738468242-76777974943846 change_contracts '{"burrito_contract":"'dev-1663710126378-66907359558484'","items_contract":"'bb-items.testnet'","strw_contract":"'bb-strw.testnet'", "pve_contract":"'dev-1663738468242-76777974943846'"}' --daoAcc bb-burrito-battle --accountId yairnava.testnet

Actualización de contrato:

    sputnikdao proposal upgrade res/burritos.wasm dev-1663710126378-66907359558484 --daoAcc bb-burrito-battle --accountId yairnava.testnet

    sputnikdao proposal upgrade res/pve.wasm dev-1663738468242-76777974943846 --daoAcc bb-burrito-battle --accountId yairnava.testnet
    

## Construido con 🛠️

* [RUST](https://www.rust-lang.org/) - Lenguaje de programación usado para contrato inteligente.
* [Rust Toolchain](https://docs.near.org/docs/develop/contracts/rust/intro#installing-the-rust-toolchain)
* [NEAR CLI](https://docs.near.org/docs/tools/near-cli) - Herramienta de interfaz de línea de comandos para interactuar con cuentas y contratos inteligentes en NEAR.
* [yarn](https://classic.yarnpkg.com/en/docs/install#mac-stable)