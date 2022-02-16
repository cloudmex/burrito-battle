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

Paso 3: Crear contrato inteligente:

Ejecute el siguiente comando dentro de cada carpeta (burrito e items) el cual generará nuestro archivo WASM en el directorio correspondiente (burrito/ o items/). Estos son los contratos inteligentes que implementaremos a continuación:
         
         ./build.sh
    
## Despliegue 📦

Desplegar ambos contratos (burrito e items) entrar a cada carpeta y ejecutar el siguiente comando:

    near dev-deploy --wasmFile res/non_fungible_token.wasm

## Métodos del contrato 🚀

Asignamos el identificador de nuestro contrato desplegado a una constante:

    Burrito
    ID=dev-1644623937975-51231628183482
    echo $ID

    Accesorios
    ID=dev-1643957848449-43046979351328
    echo $ID

Ambos contratos deben inicializarse antes de su uso, por lo que lo inicializaremos con los metadatos predeterminados:

    near call $ID init_contract '{"owner_id": "'$ID'"}' --accountId $ID

Podremos ver nuestros metadatos inmediatamente después:

    near call $ID nft_metadata --accountId $ID

Obtener cantidad de burritos creados:

    near view $ID get_number_burritos

Obtener cantidad de batallas realizadas:

    near view $ID get_number_battles

Obtener cantidad de accesorios creados:

    near view $ID get_number_accessories
    
Crear nuevo burrito:

    near call $ID nft_mint_token '{"token_owner_id": "'yairnava.testnet'", "colecction": "'Burritos'", "token_metadata": { "title": "Burrito Name", "description": "This is a burrito", "media": "", "extra":""}}' --accountId yairnava.testnet --deposit 0.1
    
Modificar burrito:

    near call $ID update_burrito '{"burrito_id": "2", "extra":"{'"'burrito_type'":"'Fuego'","'hp'":"'5'","'attack'":"'9'","'defense'":"'5'","'speed'":"'7'","'level'":"'1'","'win'":"'0'"}'"}' --accountId yairnava.testnet 

Obtener datos de un burrito:

    near view $ID get_burrito '{"burrito_id": "0"}'

    near view $ID nft_token '{"token_id":"2"}' --accountId yairnava.testnet

Guardar sala de combate jugador vs cpu:
    near call $ID save_battle_player_cpu '{"burrito1_id":"0","burrito2_id":"1","burrito3_id":"2"}' --accountId yairnava.testnet 

Combate jugador vs cpu

    near call $ID fight_player_cpu '{"burrito1_id": "2","accesorio1_burrito1_id":"0","accesorio2_burrito1_id":"1","accesorio3_burrito1_id":"2","burrito_cpu_level":1}' --accountId yairnava.testnet --gas=300000000000000

    near call $ID fight_player_cpu '{"burrito1_id": "1","accesorio1_burrito1_id":"0","accesorio2_burrito1_id":"1","accesorio3_burrito1_id":"3","burrito_cpu_level":1}' --accountId yairnava.testnet --gas=300000000000000

    near call $ID fight_player_cpu '{"burrito1_id": "1","accesorio1_burrito1_id":"0","accesorio2_burrito1_id":"2","accesorio3_burrito1_id":"2","burrito_cpu_level":2}' --accountId yairnava.testnet --gas=300000000000000


    near call $ID fight_player_cpu '{"burrito1_id": "1","accesorio1_burrito1_id":"0","accesorio2_burrito1_id":"1","accesorio3_burrito1_id":"2","burrito_cpu_level":1}' --accountId yairnava.testnet --gas=300000000000000

Combate de 2 burritos

    near call $ID fight_burritos '{"burrito1_id": "0","accesorio1_burrito1_id":"0","accesorio2_burrito1_id":"1","accesorio3_burrito1_id":"2","burrito2_id": "2","accesorio1_burrito2_id":"0","accesorio2_burrito2_id":"1","accesorio3_burrito2_id":"2"}' --accountId yairnh.testnet --gas=300000000000000

Crear nuevo accesorio:

    near call $ID nft_mint_token '{"token_owner_id": "'yairnava.testnet'", "colecction": "'Items'", "token_metadata": { "title": "Thunder Sword", "description": "Thunder Sword 2", "media": "","extra":"{'"'attack'":"'3'","'defense'":"'0'","'speed'":"'0'"}'"}}' --accountId yairnava.testnet --deposit 0.1 --gas=300000000000000

Obtener datos de un accesorio:

    near view $ID get_accessory '{"accessory_id": "0"}'
    
    near view $ID nft_token '{"token_id":"0"}' --accountId yairnava.testnet


## Construido con 🛠️

* [RUST](https://www.rust-lang.org/) - Lenguaje de programación usado para contrato inteligente.
* [NEAR CLI](https://docs.near.org/docs/tools/near-cli) - Herramienta de interfaz de línea de comandos para interactuar con cuentas y contratos inteligentes en NEAR.

// Crear varios Burritos/Items desde market place

for i in {1..5}; do near call dev-1643331107973-95015694722073 market_mint_generic '{ "contractaddress":"dev-1643397987098-14647510574330", "token_owner_id": "yairnh.testnet","colecction":"Burritos_Nativo", "token_metadata": { "title": "Omega Burrito", "description": "This is a mega burrito", "media": "","extra":""}}'  --accountId yairnh.testnet  --amount 0.1 --gas=300000000000000; done

for i in {1..5}; do near call dev-1643331107973-95015694722073 market_mint_generic '{ "contractaddress":"dev-1643395586727-81817628656300", "token_owner_id": "'yairnh.testnet'", "colecction": "'Items_Nativo'", "token_metadata": { "title": "Thunder Sword", "description": "Thunder Sword", "media": "","extra":"{'"'attack'":"'3'","'defense'":"'0'","'speed'":"'0'"}'"}}'   --accountId yairnh.testnet  --amount 0.1 --gas=300000000000000; done