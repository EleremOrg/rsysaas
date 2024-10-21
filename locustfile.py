import random
from locust import HttpUser, task, constant_pacing


class HelloWorldUser(HttpUser):
    wait_time = constant_pacing(0.1)
    host = "http://127.0.0.1:8001/api/v1"

    def generate_product(self):
        return {
            "specs": {
                "category": "Shirt",
                "gender": "Men",
            },
            "description": "Some descriptioin",
            "currency": "USD",
            "id": str(random.randint(1, 100000000000000000)),
            "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            "price": 88.0,
            "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
        }

    @task
    def insert_products(self):
        products = [self.generate_product() for _ in range(10)]
        self.client.post("/products", json={"products": products, "target": "Clothing"})

    # @task
    def update_products(self):
        pk = str(random.randint(1, 10000))
        products = (
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
            {
                "category": "Shirt",
                "gender": "Men",
                "id": pk,
                "image": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
                "price": 88.0,
                "url": "http://127.0.0.1:8000/api/v1/products/Shirt/Men/",
            },
        )
        self.client.put("/products", json={"products": products, "target": "Clothing"})
