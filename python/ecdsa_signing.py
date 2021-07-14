# BitGreen - Example to sign and verify a message using a SECP256K1 private key
# to install the required libraries:
# pip install substrate-interface
# pip install base58
# the "ellipticcurve package is a fork of:"
# https://github.com/starkbank/ecdsa-python
# with a few changes for Bitgreen usage
# https://github.com/polkascan/py-substrate-interface


# system packages
import sys
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

bytesPrivatekey=base58.b58decode_check("7qdnjLy7dcESfsLgdNMm26vtueVZX4Q9Y7AkDBJ5Gvm3z8bgpa7k")
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

bitgreenaccount="GQ2htcEUSahvYp49vWwfnrgTDk8dbQ724d"
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

# define connection parameters
substrate = SubstrateInterface(
    url="wss://testnode.bitg.org",
    #url="ws://127.0.0.1:9944",
    ss58_format=42,
    type_registry_preset='substrate-node-template'
)
# create Substrate Key pair from secret seed
secretseed='episode together nose spoon dose oil faculty zoo ankle evoke admit walnut';  # better to use 24 words!
keypair = Keypair.create_from_mnemonic(secretseed)
print("Address of BitGreen Account: ",keypair.ss58_address)
#keypair = Keypair.create_from_uri('//Alice')

# create call object
print("[INFO] Creating Extrinsic call")
call = substrate.compose_call(
    call_module='Claim',
    call_function='claim_deposit',
    call_params={
        'oldaddress': bitgreenaccount,
        'oldpublickey': pkbase64,
        'signature': sigb64,
    }
)
# execute exstrisic
print("[INFO] Executing Extrinsic call")
extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
try:
    receipt = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
    print("[INFO] Extrinsic '{}' sent and finalized in block '{}'".format(receipt.extrinsic_hash, receipt.block_hash))
except SubstrateRequestException as e:
    print("[INFO] Failed to send: {}".format(e))


