# BitGreen - Example to sign and verify a message using a SECP256K1 private key
# this version send the claiming request by a post to a BITG Proxy Server
# to install the required libraries:
# pip install substrate-interface
# pip install base58
# the "ellipticcurve package is a fork of:"
# https://github.com/starkbank/ecdsa-python
# with a few changes for Bitgreen usage
# https://github.com/polkascan/py-substrate-interface


# system packages
import sys
import os
import json
#add local path for packages
sys.path.append(".")
# Ecsda module
from ellipticcurve.ecdsa import Ecdsa
from ellipticcurve.privateKey import PrivateKey
# Substrate module
from substrateinterface import SubstrateInterface, Keypair
from substrateinterface.exceptions import SubstrateRequestException
# base64 encoder/decoder
import base64
# base58 encoder/decoder
import base58
# import binary utility
from ellipticcurve.utils.binary import BinaryAscii
#import scale library to load data types
import scalecodec
# library for https post
import requests


# function to load data types registry
def load_type_registry_file(file_path: str) -> dict:

    with open(os.path.abspath(file_path), 'r') as fp:
        data = fp.read()

    return json.loads(data)


# Generate privateKey from PEM string
# You should get it in 
#privateKey = PrivateKey.fromPem("""
#    -----BEGIN EC PARAMETERS-----
#    BgUrgQQACg==
#    -----END EC PARAMETERS-----
#    -----BEGIN EC PRIVATE KEY-----
#    MHQCAQEEIIXwyeh90OW9VJbuHiE4gPckxP+Sl1xgCOeJuLGb0YHYoAcGBSuBBAAK
#    oUQDQgAEM6fLbHdW9qo945SKbOhJLU9lJwwJaK33AgaVHOKbuo8SWPr8ryerBj6g
#    zs/cTNCa7+aNTI8Fc8DrWHXeUFfeYg==
#    -----END EC PRIVATE KEY-----
#""")
# you can create the private key object from a DER encoded key, by calling"PrivateKey.fromDer()"
# you can create the private key from and encoded key in Base64, by calling "PrivateKey.fromBase64()"
# define example accounts that will be the body of the message

bytesPrivatekey=base58.b58decode_check("7vG9qnC7QnG77qAvptaiXHfkrXwH4nkgx6NPoG95JFPVbqkot9gM")
print("[INFO] FULL Private Key in hex: "+bytesPrivatekey.hex())
bl=len(bytesPrivatekey)
privateKey = PrivateKey.fromString(bytesPrivatekey[1:bl-1])
bytesPrivatekeyhex = bytesPrivatekey[1:bl-1].hex()
print("[INFO] Private Key in hex: "+bytesPrivatekeyhex)

# compute public key from private key
publicKey = privateKey.publicKey()
pks=publicKey.toString()
print("[INFO] Public Key binary (string) lenght: ",len(pks))
pkba=[ord(c) for c in pks]
print("[INFO] Public Key as ascii array: ",pkba, "len:",len(pkba))
pkb=bytes(pkba)
print("[INFO] Public Key in hex: ",pkb.hex(), "len:",len(pkb))

bitgreenaccount="GR8x2NLn5fEPbbJW29EHuVjcCvE3dEhyu8"
print("[INFO] Signing a message")
# generate the signature
signature = Ecdsa.sign(bitgreenaccount, privateKey)

# show results on console
print("Signature: "+signature.toBase64())
print("Message Signed: "+bitgreenaccount)
pk =publicKey.toString()
sigb64=signature.toBase64()

ba=bytearray()
for c in pk:
    ba.append(ord(c))
pkbase64b=base64.b64encode(ba)
pkbase64=pkbase64b.decode("ascii")
print("Public Key Base64: ",pkbase64)

# verify the signature
print("[INFO] Signature verification result: ",Ecdsa.verify(bitgreenaccount, signature, publicKey))

# load custom data types
custom_type_registry = load_type_registry_file("../assets/types.json")

# define connection parameters
substrate = SubstrateInterface(
    url="wss://testnode.bitg.org",
    #url="ws://127.0.0.1:9944",
    ss58_format=42,
    type_registry_preset='default',
    type_registry=custom_type_registry

)
# create Substrate Key pair from secret seed
secretseed='episode together nose spoon dose oil faculty zoo ankle evoke admit walnut';  # better to use 24 words!
keypair = Keypair.create_from_mnemonic(secretseed)
print("Address of BitGreen Account: ",keypair.ss58_address)


# Https Post return immediately, check the balance after 10 seconds.
endpoint="http://localhost:3001/claim"
payload={
    'oldaddress':bitgreenaccount,
    'oldpublickey': pkbase64,
    'signature': sigb64,
    'recipient': keypair.ss58_address,
}
print(payload)
response = requests.post(endpoint, data=payload)
print(response.text) #TEXT/HTML
print(response.status_code, response.reason) #HTTP



