import requests


url = "http://localhost:8001/api/v1/recommendations/"
token = "invfin_token"
headers = {
	"Authorization": f"Bearer {token}"
}

data = {
    "entity": "companies",
    "target": "product",
    "userId": None,
    "prodId": "1",
    "numberRecommendations": "",
}

response = requests.get(url, params=data, headers=headers)

print(response.json())
print(response.status_code)
