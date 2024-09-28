import requests

url = "http://127.0.0.1:8000/api/v1/custom/products"
# token = "invfin_token_123"
headers = {
    # "Authorization": f"Bearer {token}"
}

# data = {
#     "entity": "companies",
#     "target": "product",
#     "userId": None,
#     "prodId": "2",
#     "numberRecommendations": "",
# }

payload = {"target": "Automotive", "products": []}

response = requests.post(url, json=payload, headers=headers)
print(response.text)
print(response.json())
