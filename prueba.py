import requests
import base64

token = base64.b64encode(b"user1@example.com:password123")
print(token)
headers = {"authorization": f"Basic dXNlcjFAZXhhbXBsZS5jb206cGFzc3dvcmQxMjM="}
print(headers)
# headers = {"WWW-Authenticate": f"Basic {token}"}
response = requests.get("http://0.0.0.0:8001/api/v1/auth/token", headers=headers)

print(response.content)
