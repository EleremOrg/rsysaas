import requests

# url = f"https://api.elerem.com/api/v1/recommendations/"
# token = "invfin_token_123"
# headers = {
# 	"Authorization": f"Bearer {token}"
# }
# # invfin_token_123
# # xyz_token_456
# data = {
#     "entity": "companies",
#     "target": "product",
#     "userId": None,
#     "prodId": "2",
#     "numberRecommendations": "",
# }

# response = requests.get(url, params=data, headers=headers)

# print(response.json())
# print(response.status_code)

payload = {
	"action": "get_diplomes",
	"params": "search=&type_directory=all&etat_fiche=&certification=&autorite=&numero_certif=&niveau_eu=&abrege=&code_nsf=&code_rome=",
	"pager": "1"
}

r = requests.post("https://www.francecompetences.fr/wp/wp-admin/admin-ajax.php", data=payload)

print(r.json())


