import requests
import base64
from urllib.request import urlopen, Request
from urllib.parse import urlencode

# token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJlbGVyZW0uY29tIiwic3ViIjoiMSIsImF1ZCI6ImVsZXJlbS5jb20iLCJleHAiOjE3MzIxNDExMjIsImlhdCI6MTczMjA1NDcyMiwianRpIjoiIiwiZ3JvdXBzIjoiQWRtaW4sIEVkaXRvciIsImNvbXBhbnkiOjF9.OsiTZc4KoE1bN0VC7jLZOvtGb8ZdLGmixkea7NV5HCY"

# url = "http://192.168.1.15:8001/api/v1/recommendations/movies"
# params = {"quantity": 1}
# url += "?" + urlencode(params, doseq=True, safe="/")

# req = Request(url, method="GET", headers={'authorization': f'Bearer {token}'})

# with urlopen(req) as response:
#     print(response.status)
#     print(response.read().decode())
# token = base64.b64encode(b"user1@example.com:password123")
# print(token)
# headers = {"authorization": f"Basic dXNlcjFAZXhhbXBsZS5jb206cGFzc3dvcmQxMjM="}
# print(headers)
# # headers = {"WWW-Authenticate": f"Basic {token}"}
# response = requests.get("http://0.0.0.0:8001/api/v1/auth/token", headers=headers)

# print(response.content)
token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJlbGVyZW0uY29tIiwic3ViIjoiMSIsImF1ZCI6ImVsZXJlbS5jb20iLCJleHAiOjE3MzIxNDExMjIsImlhdCI6MTczMjA1NDcyMiwianRpIjoiIiwiZ3JvdXBzIjoiQWRtaW4sIEVkaXRvciIsImNvbXBhbnkiOjF9.OsiTZc4KoE1bN0VC7jLZOvtGb8ZdLGmixkea7NV5HCY"
headers = {"authorization": f"Bearer {token}"}
response = requests.get(
    "http://192.168.1.15:8001/api/v1/recommendations/movies?quantity=1", headers=headers
)

print(response.content)
