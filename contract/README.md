Ejecute lo siguiente comando el cual generará nuestro archivo WASM. Este es el contrato inteligente que implementaremos a continuacion:

	 ./build.sh

Desplegar contrato:

	near dev-deploy --wasmFile res/non_fungible_token.wasm

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

    near call $ID get_burrito '{"token_id": "1"}' --accountId $ID

Pelear:
near call $ID fight_burritos '{"token_id_burrito1": "1","token_id_burrito2": "2"}' --accountId $ID

Crear nuevo token:

"extra":"{'
    "hp":"10",
    "attack":"10",
    "defense":"10",
    "speed":"10"
'}"


near call $ID new_burrito '{"token_id": "4", "receiver_id": "'yairnava.testnet'", "token_metadata": { "title": "X Burrito", "description": "This is a mega burrito", "media": "","extra":"{'"'hp'":"'5'","'attack'":"'15'","'defense'":"'10'","'speed'":"'20'"}'"}}' --accountId yairnava.testnet --deposit 0.1

near call $ID update_burrito '{"token_id": "4", "extra":"{'"'hp'":"'5'","'attack'":"'15'","'defense'":"'10'","'speed'":"'20'","'win'":"'0'"}'"}' --accountId $ID 

near call $ID get_burrito '{"token_id": "3"}' --accountId $ID

near call $ID get_burrito '{"token_id": "4"}' --accountId $ID

near call $ID fight_burritos '{"token_id_burrito1": "3","token_id_burrito2": "4"}' --accountId $ID