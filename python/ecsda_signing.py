# BitGreen - Example to sign and verify a message using a SECP256K1 private key
# to install the required libraries:
# pip install starkbank-ecdsa
# pip install substrate-interface
# further reference an documentation is here:
# https://github.com/starkbank/ecdsa-python
# https://github.com/polkascan/py-substrate-interface

# Ecsda module
from ellipticcurve.ecdsa import Ecdsa
from ellipticcurve.privateKey import PrivateKey
# Substrate module
from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException


# Generate privateKey from PEM string
# You should get it in 
privateKey = PrivateKey.fromPem("""
    -----BEGIN EC PARAMETERS-----
    BgUrgQQACg==
    -----END EC PARAMETERS-----
    -----BEGIN EC PRIVATE KEY-----
    MHQCAQEEIIXwyeh90OW9VJbuHiE4gPckxP+Sl1xgCOeJuLGb0YHYoAcGBSuBBAAK
    oUQDQgAEM6fLbHdW9qo945SKbOhJLU9lJwwJaK33AgaVHOKbuo8SWPr8ryerBj6g
    zs/cTNCa7+aNTI8Fc8DrWHXeUFfeYg==
    -----END EC PRIVATE KEY-----
""")
# you can create the private key object from a DER encoded key, by calling"PrivateKey.fromDer()"
# you can create the private key from and encoded key in Base64, by calling "PrivateKey.fromBase64()"
# define example accounts that will be the body of the message
bitgreenaccount="GHqy2W8N4NYLtEVSVdKTtPPT4oPc8GXLbM"

print("[INFO] Signing a message")
# generate the signature
signature = Ecdsa.sign(bitgreenaccount, privateKey)

print("[INFO] Computing the public key from private key")
# compute public key from private key
publicKey = privateKey.publicKey()

# show results on console
print("Signature: "+signature.toBase64())
print("Message Signed: "+bitgreenaccount)
print("Public Key: ")
print(publicKey.toPem())

# you should post to the blockchain the following fields:
#bitgreenaccount, bitgreensubstrateaccount, signature.toBase64(),publicKey.toPem()

# verify the signature
print("[INFO] Signature verification result: ",Ecdsa.verify(bitgreenaccount, signature, publicKey))

# define connection parameters
substrate = SubstrateInterface(
    #url="wss://testnode.bitg.org",
    url="ws://127.0.0.1:9944",
    ss58_format=42,
    type_registry_preset='substrate-node-template'
)
# create Substrate Key pair from secret seed
secretseed='episode together nose spoon dose oil faculty zoo ankle evoke admit walnut';  # better to use 24 words!
keypair = Keypair.create_from_mnemonic(secretseed)
# create call object
print("[INFO] Creating Extrinsic call")
call = substrate.compose_call(
    call_module='Claim',
    call_function='claim_deposit',
    call_params={
        'oldaddress': bitgreenaccount,
        'oldpublickey': publicKey.toPem(),
        'signature': signature.toBase64()
    }
)
# execute exstrisic
print("[INFO] Executing Extrinsic call")
extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
try:
    receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
    print("[INFO] Extrinsic '{}' sent and included in block '{}'".format(receipt.extrinsic_hash, receipt.block_hash))
except SubstrateRequestException as e:
    print("[INFO] Failed to send: {}".format(e))


