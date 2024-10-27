import requests
import base64

token = base64.b64encode("lucas:admin")
headers = {"Authorization", f"Basic {token}"}
response = requests.post("http://0.0.0.0:8001/api/v1/token", headers=headers)

print(response.content)
