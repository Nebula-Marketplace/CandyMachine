import os
import json

with open("contract.json", "r") as f:
    contract = json.load(f)

for i in range(4997):
    contract.append({
        "owner": "inj1gtqhxvur9j4uwccrwgtlcz9dc7zeae02vs8q2u",
        "uri": "https://example.com/{}".format(i),
    })

with open("contract.json", "w") as f:
    json.dump(contract, f, indent=4)