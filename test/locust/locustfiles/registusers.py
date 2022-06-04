from locust import HttpLocust, TaskSet, task

class Tasks(TaskSet):
    @task
    def regist(self):
        for x in range(30000,31000,1):
            self.client.post("/api/user/login", json={"name": str(x), "password": "12345678"})