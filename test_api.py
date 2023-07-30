import requests

url = f"http://localhost:8001/api/v1/recommendations/"
token = "invfin_token_123"
headers = {
	"Authorization": f"Bearer {token}"
}
# invfin_token_123
# xyz_token_456
data = {
    "entity": "companies",
    "target": "product",
    "userId": None,
    "prodId": "2",
    "numberRecommendations": "",
}

response = requests.get(url, params=data, headers=headers)

print(response.json())
print(response.status_code)
