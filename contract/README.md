Ejecute lo siguiente comando el cual generará nuestro archivo WASM. Este es el contrato inteligente que implementaremos a continuacion:

	 ./build.sh

Desplegar contrato:

	near dev-deploy --wasmFile res/non_fungible_token.wasm

Asignamos el identificador de nuestro contrato desplegado a una constante:

    ID=dev-1636579705824-12962852261111
    echo $ID

El contrato NFT debe inicializarse antes de su uso, por lo que lo inicializaremos con los metadatos predeterminados:

	near call $ID new_default_meta '{"owner_id": "'$ID'"}' --accountId $ID

Podremos ver nuestros metadatos inmediatamente después:

	near call $ID nft_metadata --accountId $ID

Obtener cantidad e tokens creados:
	
	near view $ID get_tokens 

Obtener datos de un token:

	near call $ID get_token '{"token_id": "1}' --accountId $ID

    near call $ID get_tokenJson '{"token_id": "1"}' --accountId $ID


Crear nuevo token:

"extra":"{'
    "hp":"10",
    "attack":"10",
    "defense":"10",
    "speed":"10"
'}"

near call $ID nft_mint '{"token_id": "1", "receiver_id": "'$ID'", "token_metadata": { "title": "Mr Burrito", "description": "This is a burrito", "media": "","extra":"{'"'hp'":"'25'","'attack'":"'15'","'defense'":"'10'","'speed'":"'20'"}'"}}' --accountId $ID --deposit 0.1