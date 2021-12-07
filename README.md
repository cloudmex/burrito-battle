# Burrito Battle

## Descripción 📄

Burrito Battle es un videojuego hecho en Rust y basado en el Protocolo de NEAR, el cual permite adquirir burritos (NFTs) y equiparlos con diferentes accesorios intercambiables que les permite aumentar sus habilidades en batalla.

Elige hasta 3 de tus mejores burritos, equípalos con hasta 3 accesorios y enfréntalos contra los de tus rivales.

### Dinamica de batalla

Las batallas se conforman de máximo 5 rondas o peleas, cada jugador selecciona hasta 3 de sus burritos para la batalla, en cada ronda o pelea el jugador selecciona 1 burrito y hasta 3 accesorios con los cuales combatir al burrito rival, cada burrito cuenta con una cantidad de vidas, y solo podrán ser usados en una pelea aquellos burritos que tengan por lo menos 1 vida.

#### El ganador de la pelea se define de la siguiente manera:

Los combates serán por turnos, para cada turno se define que burrito será el primero en atacar tomando en cuenta su estadística de velocidad, el accesorio equipado y un numero generado aleatoriamente en un rango de 0.1 y 1.0, se toma con la formula (velocidad + accesorios) *número aleatorio. El burrito con el resultado mayor será el primero en atacar.

Una vez definida la prioridad se comienza con los ataques, el burrito con la prioridad de ataque hace el primer movimento tomando en cuenta su estadística de ataque, su accesorio, su tipo y un numero generado eleatoriamente, la formula es (ataque+accesorios)*numero aleatorio, si el burrito atacante tiene ventaja por tipo a este resultado se le suma un 25% de su ataque final, este resultado son los puntos con que se realizara el ataque restandolos a los puntos de defensa del burrito que está siendo atacado, a continuacion se evalua si la defensa del burrito atacado es menor de 0, en éste caso el ganador de la pelea es el burrito atacante, en caso contrario el burrito atacado ahora pasa a ser el atacante tomando en cuenta todo lo antes mencionado, los burritos solo pueden atacar 1 vez por turno, la pelea puede tener n turnos hasta que alguno de los burritos tenga su defensa menor que 0, cuando esto pasa el burrito atacante gana la pelea además de incrementar su contador de victorias en 1 (Éste contador será utilizado para incrementar el nivel y estaísticas bases del burrito en algún momento) y el burrito perdedor pierde una vida.

La batalla continua con la siguende pelea donde se repite todo el proceso anterior, el ganador de la batalla es el que logre ganar 3 de 5 peleas.

#### Típos de burritos
| VS | Fuego | Agua | Planta | Eléctrico | Volador |
| --- | --- | --- | --- | --- | --- |
| Fuego | 0% | +25% | +25% | 0% | 0% |
| Agua | +25% | 0% | 0% | 0% | +25% |
| Planta | +25% | 0% | 0% | +25% | 0% |
| Eléctrico | 0% | 0% | +25% | 0% | +25% |
| Volador | 0% | +25% | 0% | +25% | 0% |

https://docs.github.com/es/github/writing-on-github/working-with-advanced-formatting/organizing-information-with-tables

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

    Ejecute lo siguiente comando el cual generará nuestro archivo WASM en el directorio contract/. Este es el contrato inteligente que implementaremos a continuacion:
         
         ./build.sh
    
## Despliegue 📦

    Desplegar contrato:

        near dev-deploy --wasmFile res/non_fungible_token.wasm

## Métodos del contrato 🚀

    Asignamos el identificador de nuestro contrato desplegado a una constante:

        ID=dev-1637092339023-28098062435852
        echo $ID
    
    El contrato NFT debe inicializarse antes de su uso, por lo que lo inicializaremos con los metadatos predeterminados:

        near call $ID init_contract '{"owner_id": "'$ID'"}' --accountId $ID

    Podremos ver nuestros metadatos inmediatamente después:
    
        near call $ID nft_metadata --accountId $ID

    Obtener cantidad e tokens creados:

        near view $ID get_number_burritos

    Obtener datos de un token:

        near call $ID get_burrito '{"token_id": "3"}' --accountId $ID

    Crear nuevo token:

        near call $ID new_burrito '{"token_id": "4", "receiver_id": "'yairnava.testnet'", "token_metadata": { "title": "X Burrito", "description": "This is a mega burrito", "media": "","extra":"{'"'hp'":"'5'","'attack'":"'15'","'defense'":"'10'","'speed'":"'20'","'win'":"'0'"}'"}}' --accountId yairnava.testnet --deposit 0.1
    
    Modificar token:

        near call $ID update_burrito '{"token_id": "4", "extra":"{'"'hp'":"'5'","'attack'":"'15'","'defense'":"'10'","'speed'":"'20'","'win'":"'0'"}'"}' --accountId $ID 
        
    Combate de 2 burritos

        near call $ID fight_burritos '{"token_id_burrito1": "3","token_id_burrito2": "4"}' --accountId $ID

## Construido con 🛠️

* [RUST](https://www.rust-lang.org/) - Lenguaje de programación usado para contrato inteligente.
* [NEAR CLI](https://docs.near.org/docs/tools/near-cli) - Herramienta de interfaz de línea de comandos para interactuar con cuentas y contratos inteligentes en NEAR.
