import requests


url = "http://localhost:8001/api/v1/recommendations/"
token = "invfin_token"
headers = {
	"Authorization": f"Bearer {token}"
}

data = {
    "entity": "",
    "target": "generic",
    "user_id": "",
    "prod_id": "",
    "number_recommendations": "",
}

response = requests.get(url, params=data, headers=headers)

print(response.json())
print(response.status_code)
