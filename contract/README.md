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


Crear nuevo token:

"extra":"{'
    "hp":"10",
    "attack":"10",
    "defense":"10",
    "speed":"10"
'}"

near call $ID new_burrito '{"token_id": "1", "receiver_id": "'$ID'", "token_metadata": { "title": "Mr Burrito", "description": "This is a burrito", "media": "","extra":"{'"'hp'":"'25'","'attack'":"'15'","'defense'":"'10'","'speed'":"'20'"}'"}}' --accountId $ID --deposit 0.1

near call $ID new_burrito '{"token_id": "2", "receiver_id": "'$ID'", "token_metadata": { "title": "Mega Burrito", "description": "This is a mega burrito", "media": "","extra":"{'"'hp'":"'25'","'attack'":"'15'","'defense'":"'10'","'speed'":"'20'"}'"}}' --accountId $ID --deposit 0.1

near call $ID update_burrito '{"token_id": "1", "extra":"{'"'hp'":"'20'","'attack'":"'15'","'defense'":"'15'","'speed'":"'20'"}'"}' --accountId $ID 

near call $ID get_burrito '{"token_id": "1"}' --accountId $ID

